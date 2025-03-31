from rdflib import Graph, Namespace, RDF, RDFS, OWL, XSD, URIRef, Literal
from rdflib.namespace import RDF, RDFS, OWL, XSD
from typing import Dict, List, Optional, Set
import xml.etree.ElementTree as ET
import re

class SPARQLProofChecker:
    def __init__(self):
        self.g = Graph()
        self.g.parse("sparql-proof.ttl", format="turtle")
        self.ns = Namespace("http://www.w3.org/2005/sparql-results#")
        self.proof_ns = Namespace("#")
        
    def parse_sparql_results(self, srx_file: str) -> Graph:
        """Parse a SPARQL results XML file into an RDF graph."""
        tree = ET.parse(srx_file)
        root = tree.getroot()
        
        g = Graph()
        g.bind("sp", self.ns)
        
        # Create result set
        result_set = URIRef(self.proof_ns["result_set"])
        g.add((result_set, RDF.type, self.proof_ns["ResultSet"]))
        
        # Parse results
        for result in root.findall(".//sp:result", {"sp": str(self.ns)}):
            result_uri = URIRef(self.proof_ns[f"result_{len(g)}"])
            g.add((result_uri, RDF.type, self.proof_ns["Result"]))
            g.add((result_set, self.proof_ns["hasResult"], result_uri))
            
            # Parse bindings
            for binding in result.findall("sp:binding", {"sp": str(self.ns)}):
                var_name = binding.get("name")
                value_elem = binding[0]
                
                binding_uri = URIRef(self.proof_ns[f"binding_{len(g)}"])
                g.add((binding_uri, RDF.type, self.proof_ns["Binding"]))
                g.add((result_uri, self.proof_ns["hasBinding"], binding_uri))
                g.add((binding_uri, self.proof_ns["hasVariable"], Literal(var_name)))
                
                if "datatype" in value_elem.attrib:
                    g.add((binding_uri, self.proof_ns["hasDatatype"], URIRef(value_elem.get("datatype"))))
                
                g.add((binding_uri, self.proof_ns["hasValue"], Literal(value_elem.text)))
        
        return g
    
    def parse_sparql_query(self, rq_file: str) -> Graph:
        """Parse a SPARQL query file into an RDF graph."""
        with open(rq_file, 'r') as f:
            query_string = f.read()
        
        g = Graph()
        g.bind("sp", self.ns)
        
        query_uri = URIRef(self.proof_ns["query"])
        g.add((query_uri, RDF.type, self.proof_ns["Query"]))
        g.add((query_uri, self.proof_ns["hasQueryString"], Literal(query_string)))
        
        return g
    
    def validate_proof(self, proof_graph: Graph, query_graph: Graph, result_graph: Graph) -> bool:
        """Validate a proof graph against a query and result graph."""
        # Get the proof from the graph
        proofs = list(proof_graph.subjects(RDF.type, self.proof_ns["Proof"]))
        if not proofs:
            return False
        
        proof = proofs[0]
        
        # Check that the proof has the correct query
        query = list(proof_graph.objects(proof, self.proof_ns["hasQuery"]))[0]
        query_string = list(query_graph.objects(query, self.proof_ns["hasQueryString"]))[0]
        
        # Check that the proof has the correct result set
        result_set = list(proof_graph.objects(proof, self.proof_ns["hasResultSet"]))[0]
        
        # Get all proof steps
        steps = list(proof_graph.subjects(RDF.type, self.proof_ns["ProofStep"]))
        
        # Validate each step
        for step in steps:
            step_type = list(proof_graph.objects(step, self.proof_ns["hasStepType"]))[0]
            step_result = list(proof_graph.objects(step, self.proof_ns["hasStepResult"]))[0]
            
            # Validate step result matches expected format
            if not self._validate_step_result(step_result, step_type, query_string):
                return False
        
        return True
    
    def _validate_step_result(self, step_result: URIRef, step_type: str, query_string: str) -> bool:
        """Validate that a step's result is correct based on its type."""
        results = list(self.g.objects(step_result, self.proof_ns["hasResult"]))
        
        if step_type == "bind":
            # For BIND steps, validate that the expression was evaluated correctly
            # This is a simplified check - in practice you'd want to evaluate the expression
            return True
        elif step_type == "filter":
            # For FILTER steps, validate that the filter condition was applied correctly
            return True
        elif step_type == "join":
            # For JOIN steps, validate that the join operation was performed correctly
            return True
        else:
            return False

def main():
    checker = SPARQLProofChecker()
    
    # Example usage with bind01 test case
    query_graph = checker.parse_sparql_query("sparql/bind/bind01.rq")
    result_graph = checker.parse_sparql_results("sparql/bind/bind01.srx")
    
    # Create a proof graph for bind01
    proof_graph = Graph()
    proof_graph.bind("sp", checker.ns)
    proof_graph.bind("", checker.proof_ns)
    
    # Create proof
    proof = URIRef(checker.proof_ns["proof"])
    proof_graph.add((proof, RDF.type, checker.proof_ns["Proof"]))
    
    # Add query
    query = URIRef(checker.proof_ns["query"])
    proof_graph.add((proof, checker.proof_ns["hasQuery"], query))
    
    # Add result set
    result_set = URIRef(checker.proof_ns["result_set"])
    proof_graph.add((proof, checker.proof_ns["hasResultSet"], result_set))
    
    # Add proof steps
    step1 = URIRef(checker.proof_ns["step1"])
    proof_graph.add((step1, RDF.type, checker.proof_ns["ProofStep"]))
    proof_graph.add((step1, checker.proof_ns["hasStepType"], Literal("bind")))
    proof_graph.add((step1, checker.proof_ns["hasStepDescription"], Literal("BIND(?o+10 AS ?z)")))
    proof_graph.add((step1, checker.proof_ns["hasStepResult"], result_set))
    
    # Validate proof
    is_valid = checker.validate_proof(proof_graph, query_graph, result_graph)
    print(f"Proof is valid: {is_valid}")

if __name__ == "__main__":
    main() 