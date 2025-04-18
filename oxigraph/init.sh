time {
  # Call the sparconvert script
  


  oxigraph load --location .oxigraph-store --file test.ttl
  # oxigraph query --location .oxigraph-store --query "SELECT ?s ?p ?o WHERE { ?s ?p ?o }" --results-format json | jq
  oxigraph query --location .oxigraph-store --query "SELECT ?s ?p ?o ?vdcd847e4a2af4c149fa7d90428686dc5 WHERE { ?s ?p ?o . BIND(TRIPLE(?s, ?p, ?o) AS ?vdcd847e4a2af4c149fa7d90428686dc5) }" --results-format json | jq
  rm -rf .oxigraph-store
}

# Next step - write the bind script
