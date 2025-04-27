use spargebra::{algebra::{self, GraphPattern, PropertyPathExpression}, term::{NamedNodePattern, TermPattern, TriplePattern, Variable}, Query};
use uuid::Uuid;

fn generate_variable() -> String {
  "v".to_owned() + &Uuid::new_v4().to_string().replace("-", "")
}

fn path_to_bgp(subject: TermPattern, path: PropertyPathExpression, object: TermPattern) -> GraphPattern {
  match path {
    PropertyPathExpression::NamedNode(n) => {
      GraphPattern::Bgp { patterns: vec![TriplePattern { subject, predicate: NamedNodePattern::NamedNode(n), object }] }
    }
    PropertyPathExpression::Reverse(p) => {
      path_to_bgp(object, *p, subject)
    }
    PropertyPathExpression::Sequence(p1, p2) => {
      let variable = Variable::new(generate_variable()).unwrap();
      let bgp1 = path_to_bgp(subject, *p1, spargebra::term::TermPattern::Variable(variable.clone()));
      let bgp2 = path_to_bgp(spargebra::term::TermPattern::Variable(variable), *p2, object);
      GraphPattern::Join { left: Box::new(bgp1), right: Box::new(bgp2) }
    }
    PropertyPathExpression::Alternative(p1, p2) => {
      let variable = Variable::new(generate_variable()).unwrap();
      let bgp1 = path_to_bgp(subject, *p1, spargebra::term::TermPattern::Variable(variable.clone()));
      let bgp2 = path_to_bgp(spargebra::term::TermPattern::Variable(variable), *p2, object);
      GraphPattern::Union { left: Box::new(bgp1), right: Box::new(bgp2) }
    }
    _ => panic!("Unsupported path expression"),
  }
}

fn extend_pattern(pattern: GraphPattern, variables: &mut Vec<Variable>, by_field: bool) -> GraphPattern {
  // println!("extend_pattern");
  match pattern {
    GraphPattern::Bgp { patterns } => { 
      // println!("bgp");
      convert_bgp(variables, patterns)
    },
    GraphPattern::Path { subject, path, object } => {
      // println!("path");
      extend_pattern(path_to_bgp(subject, path, object), variables, by_field)
    },
    GraphPattern::Join { left, right } => {
      // println!("join");
      let left = extend_pattern(*left, variables, by_field);
      let right = extend_pattern(*right, variables, by_field);
      GraphPattern::Join { left: Box::new(left), right: Box::new(right) }
    },
    GraphPattern::LeftJoin { left, right, expression } => {
      let left = extend_pattern(*left, variables, by_field);
      let right = extend_pattern(*right, variables, by_field);
      GraphPattern::LeftJoin { left: Box::new(left), right: Box::new(right), expression }
    },
    GraphPattern::Filter { expr, inner } => {
      if (by_field) {
        return GraphPattern::Filter { expr: expr, inner: inner }
      }

      todo!("orderby")
    },
    GraphPattern::Union { left, right } => {
      let left = extend_pattern(*left, variables, by_field);
      let right = extend_pattern(*right, variables, by_field);
      GraphPattern::Union { left: Box::new(left), right: Box::new(right) }
    },
    GraphPattern::Graph { name, inner } => todo!("graph"),
    GraphPattern::Extend { inner, variable, expression } => {
      // println!("extend");
      GraphPattern::Extend { inner, variable, expression }
    },
    GraphPattern::Minus { left, right } => todo!("minus"),
    GraphPattern::Values { variables, bindings } => {
      // TODO: See if we can use an existing library to convert the values into unions


      todo!("values")
    },
    GraphPattern::OrderBy { inner, expression } => {
      if (by_field) {
        return GraphPattern::OrderBy { inner: inner, expression: expression }
      }

      todo!("orderby")
    },
    GraphPattern::Project { inner, variables } => todo!("project"),
    // TO IMPLEMENT THIS 
    GraphPattern::Distinct { inner } => {
      if (by_field) {
        return GraphPattern::Distinct { inner: inner }
      }

      todo!("slide")
    },
    GraphPattern::Reduced { inner } => todo!("reduced"),
    GraphPattern::Slice { inner, start, length } => {
      if (by_field) {
        return GraphPattern::Slice { inner: inner, start: start, length: length }
      }

      todo!("slide")
    },
    GraphPattern::Group { inner, variables: ivariables, aggregates } => {
      // println!("group");
      let inner = extend_pattern(*inner, variables, by_field);
      GraphPattern::Group { inner: Box::new(inner), variables: ivariables, aggregates }
      
    },
    GraphPattern::Service { name, inner, silent } => todo!("service"),
  }
}

fn convert_bgp(variables: &mut Vec<Variable>, patterns: Vec<spargebra::term::TriplePattern>) -> GraphPattern {
    // Get the first triple pattern from the BGP
    let first_triple   = patterns.first().unwrap();
      
    let mut result = to_extend(variables, first_triple);

    for pattern in patterns.iter().skip(1) {
        let left = result.clone();
        let right = to_extend(variables, pattern);
        result = GraphPattern::Join { left: Box::new(left), right: Box::new(right) };
      }

    result
}

fn to_extend(variables: &mut Vec<Variable>, first_triple: &spargebra::term::TriplePattern) -> GraphPattern {
    let variable = Variable::new(generate_variable()).unwrap();
    variables.push(variable.clone());

    let TriplePattern { mut subject, predicate, mut object } = first_triple.clone();
    if let TermPattern::BlankNode(b) = subject {
      subject = TermPattern::Variable(Variable::new(generate_variable()).unwrap());
    }
    if let TermPattern::BlankNode(b) = object {
      object = TermPattern::Variable(Variable::new(generate_variable()).unwrap());
    }

    GraphPattern::Extend {
          inner: Box::new(GraphPattern::Bgp { patterns: vec![TriplePattern { subject: subject.clone(), predicate: predicate.clone(), object: object.clone() }] }), 
          variable: variable.clone(), 
          expression: algebra::Expression::FunctionCall(
              algebra::Function::Triple,
              vec![
                  match subject {
                      TermPattern::Variable(v) => algebra::Expression::Variable(v.clone()),
                      TermPattern::NamedNode(n) => algebra::Expression::NamedNode(n.clone()),
                      TermPattern::Literal(l) => algebra::Expression::Literal(l.clone()),
                      TermPattern::BlankNode(_) => panic!("Blank node in subject position"),
                      TermPattern::Triple(_) => panic!("Triple in subject position"),
                  },
                  match predicate {
                    NamedNodePattern::Variable(v) => algebra::Expression::Variable(v.clone()),
                    NamedNodePattern::NamedNode(n) => algebra::Expression::NamedNode(n.clone()),
                  },
                  match object {
                      TermPattern::Variable(v) => algebra::Expression::Variable(v.clone()),
                      TermPattern::NamedNode(n) => algebra::Expression::NamedNode(n.clone()),
                      TermPattern::Literal(l) => algebra::Expression::Literal(l.clone()),
                      TermPattern::BlankNode(_) => panic!("Blank node in object position"),
                      TermPattern::Triple(t) => panic!("Triple in object position"),
                  }
              ]
          ),
      }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} [-convert|-check] <SPARQL query>", args[0]);
        std::process::exit(1);
    }

    let flag = &args[1];
    let query_str = &args[2];
    

    match flag.as_str() {
        "-convert" => {
            let mut query = Query::parse(query_str, None).unwrap();
            if let Query::Select { dataset, pattern: GraphPattern::Project { inner, variables }, base_iri } = &mut query {
                let new_pattern = extend_pattern(*inner.clone(), variables, true);

                let q2 = Query::Select { 
                    dataset: dataset.clone(), 
                    pattern: GraphPattern::Project { 
                        inner: Box::new(new_pattern),
                        variables: variables.to_vec() 
                    }, 
                    base_iri: base_iri.clone() 
                };

                println!("{}", q2.to_string());
            }
        }
        "-check" => {
            let result = &args[3];
            use std::collections::HashMap;

            let result_json = serde_json::from_str::<serde_json::Value>(result).unwrap();
            let result_json = result_json.get("results").unwrap().get("bindings").unwrap().as_array().unwrap();
            let result_json = result_json.iter().map(|binding| {
                binding.as_object().unwrap().iter().map(|(key, value)| (key.to_string(), value.to_string())).collect::<HashMap<String, String>>()
            }).collect::<Vec<HashMap<String, String>>>();

            println!("{:?}", result_json);
        }
        _ => {
            eprintln!("Invalid flag. Use -convert or -check");
            std::process::exit(1);
        }
    }
}
