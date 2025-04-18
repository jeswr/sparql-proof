use spargebra::{algebra::{self, GraphPattern}, term::{TermPattern, Variable, NamedNodePattern}, Query};
use uuid::Uuid;

fn generate_variable() -> String {
  "v".to_owned() + &Uuid::new_v4().to_string().replace("-", "")
}

fn extend_pattern(pattern: GraphPattern, variables: &mut Vec<Variable>) -> GraphPattern {
  match pattern {
    GraphPattern::Bgp { patterns } => {
      let variable = Variable::new(generate_variable()).unwrap();
      variables.push(variable.clone());
      
      // Get the first triple pattern from the BGP
      let first_triple   = patterns.first().unwrap();
      
      GraphPattern::Extend {
          inner: Box::new(GraphPattern::Bgp { patterns: patterns.clone() }), 
          variable: variable.clone(), 
          expression: algebra::Expression::FunctionCall(
              algebra::Function::Triple,
              vec![
                  match &first_triple.subject {
                      TermPattern::Variable(v) => algebra::Expression::Variable(v.clone()),
                      TermPattern::NamedNode(n) => algebra::Expression::NamedNode(n.clone()),
                      TermPattern::Literal(l) => algebra::Expression::Literal(l.clone()),
                      _ => panic!("Unsupported term pattern in subject position"),
                  },
                  match &first_triple.predicate {
                    NamedNodePattern::Variable(v) => algebra::Expression::Variable(v.clone()),
                    NamedNodePattern::NamedNode(n) => algebra::Expression::NamedNode(n.clone()),
                },
                  match &first_triple.object {
                      TermPattern::Variable(v) => algebra::Expression::Variable(v.clone()),
                      TermPattern::NamedNode(n) => algebra::Expression::NamedNode(n.clone()),
                      TermPattern::Literal(l) => algebra::Expression::Literal(l.clone()),
                      _ => panic!("Unsupported term pattern in object position"),
                  }
              ]
          ),
      }
    },
    GraphPattern::Path { subject, path, object } => todo!("path"),
    GraphPattern::Join { left, right } => todo!("join"),
    GraphPattern::LeftJoin { left, right, expression } => todo!("left join"),
    GraphPattern::Filter { expr, inner } => todo!("filter"),
    GraphPattern::Union { left, right } => todo!("union"),
    GraphPattern::Graph { name, inner } => todo!("graph"),
    GraphPattern::Extend { inner, variable, expression } => {
      println!("extend");
      GraphPattern::Extend { inner, variable, expression }
    },
    GraphPattern::Minus { left, right } => todo!("minus"),
    GraphPattern::Values { variables, bindings } => todo!("values"),
    GraphPattern::OrderBy { inner, expression } => todo!("order by"),
    GraphPattern::Project { inner, variables } => todo!("project"),
    GraphPattern::Distinct { inner } => todo!("distinct"),
    GraphPattern::Reduced { inner } => todo!("reduced"),
    GraphPattern::Slice { inner, start, length } => todo!("slice"),
    GraphPattern::Group { inner, variables, aggregates } => todo!("group"),
    GraphPattern::Service { name, inner, silent } => todo!("service"),
  }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <SPARQL query>", args[0]);
        std::process::exit(1);
    }
    let query_str = &args[1];
    let mut query = Query::parse(query_str, None).unwrap();
    if let Query::Select { dataset, pattern: GraphPattern::Project { inner, variables }, base_iri } = &mut query {
      let new_pattern = extend_pattern(*inner.clone(), variables);

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
    // println!("{}", query.to_string());
}
