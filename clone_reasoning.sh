#!/bin/bash

# Create reasoning directory if it doesn't exist
mkdir -p reasoning

# Clone the repository with specific commit
git clone --depth 1 https://github.com/eyereasoner/eye.git temp_eye

# Copy all files from the reasoning directory
cp -r temp_eye/reasoning/* reasoning/

# Clean up
rm -rf temp_eye

# Clone SPARQL 1.1 test suite
git clone --depth 1 https://github.com/w3c/rdf-tests.git temp_rdf_tests

# Create sparql directory if it doesn't exist
mkdir -p sparql

# Copy SPARQL 1.1 test files
cp -r temp_rdf_tests/sparql/sparql11/* sparql/

# Clean up
rm -rf temp_rdf_tests

echo "Files have been copied to the reasoning and sparql directories" 