# SPARQL Proof

This project provides a framework for describing and validating proofs of SPARQL query results. It consists of:

1. A Turtle ontology (`sparql-proof.ttl`) that defines the vocabulary for describing SPARQL query result proofs
2. A Python proof checker (`proof_checker.py`) that validates proofs against queries and results
3. Example test cases based on the SPARQL 1.1 test suite

## Installation

1. Clone this repository
2. Install the Python dependencies:
```bash
pip install -r requirements.txt
```

## Usage

The proof checker can be used to validate proofs of SPARQL query results. Here's an example:

```python
from proof_checker import SPARQLProofChecker

checker = SPARQLProofChecker()

# Parse the query and results
query_graph = checker.parse_sparql_query("sparql/bind/bind01.rq")
result_graph = checker.parse_sparql_results("sparql/bind/bind01.srx")

# Create and validate a proof
proof_graph = create_proof_graph()  # You would implement this based on your needs
is_valid = checker.validate_proof(proof_graph, query_graph, result_graph)
```

## Test Cases

The project includes test cases based on the SPARQL 1.1 test suite. Currently implemented test cases:

1. `bind01` - Basic BIND operation
2. `bind02` - BIND with multiple variables
3. `bind03` - BIND with arithmetic expressions
4. `bind04` - BIND with string operations
5. `bind05` - BIND with type casting

## Ontology

The `sparql-proof.ttl` ontology defines the following main concepts:

- `Proof` - A complete proof of a SPARQL query result
- `ProofStep` - A single step in a proof
- `Query` - A SPARQL query
- `ResultSet` - A set of query results
- `Result` - A single query result
- `Binding` - A variable binding in a result

The ontology includes properties to describe:
- The relationship between proofs, queries, and results
- The sequence of proof steps
- The type and description of each step
- The intermediate results produced by each step
- The variable bindings in results

## Future Work

1. Implement more test cases from the SPARQL 1.1 test suite
2. Add support for more complex SPARQL operations (FILTER, JOIN, etc.)
3. Generate SHACL validation shapes from the ontology
4. Add support for proof visualization
5. Implement more sophisticated proof validation logic
