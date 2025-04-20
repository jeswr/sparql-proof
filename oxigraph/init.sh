#!/bin/bash

# Get the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"

# Check if query file argument is provided
if [ -z "$1" ]; then
    echo "Error: Please provide a query file path as the first argument"
    exit 1
fi

query=$1
# echo "Query: $query"
output=$("$SCRIPT_DIR/target/release/sparconvert" -convert "$query")
# echo "Converted query: $output"
oxigraph load --location .oxigraph-store --file "$SCRIPT_DIR/test.ttl"
# # oxigraph query --location .oxigraph-store --query "SELECT ?s ?p ?o WHERE { ?s ?p ?o }" --results-format json | jq
result=$(oxigraph query --location .oxigraph-store --query "$output" --results-format json)

"$SCRIPT_DIR/target/release/sparconvert" -check "$query" "$result"
# rm -rf .oxigraph-store
