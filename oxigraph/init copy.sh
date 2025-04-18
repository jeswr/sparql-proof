#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

time {
  query=$(cat query.rq)
  output=$("$SCRIPT_DIR/target/release/sparconvert" "$query")
  echo "$output"
  oxigraph load --location .oxigraph-store --file test.ttl
  # # oxigraph query --location .oxigraph-store --query "SELECT ?s ?p ?o WHERE { ?s ?p ?o }" --results-format json | jq
  oxigraph query --location .oxigraph-store --query "$output" --results-format json | jq
  rm -rf .oxigraph-store
}
