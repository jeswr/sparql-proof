I would like to build an RDF vocabulary for describing the proof of how a SPARQL query result is derived.

Please generate the following:
- A turtle file describing the ontology, use relative URIs so that we can set an appropriate base uri at a later date. Please make this a full OWL ontology with domain and range restrictions etc. as I would like to automatically generate SHACL validation shapes from the ontology
- A set of test cases which including:
   - Sample datasets
   - Sample

For the proof language you can take inspiration from the Notation3 proof language which can be seen in use at in the reasoning folder here.