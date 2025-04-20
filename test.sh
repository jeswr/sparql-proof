curl -X POST http://localhost:3000/sparql \
  -H "Content-Type: application/sparql-query" \
  -H "Accept: application/sparql-results+json" \
  -d 'SELECT * WHERE { ?s a ?o }'

echo "

--------------------------------

"

curl -X POST http://localhost:3000/sparql \
  -H "Content-Type: application/sparql-query" \
  -H "Accept: application/sparql-results+json" \
  -d 'PROVE SELECT * WHERE { ?s a ?o }'
