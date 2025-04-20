import express from 'express';
import { Parser } from 'sparqljs';
import { execSync } from 'child_process';

const app = express();
const port = process.env.PORT || 3000;

// Middleware
app.use(express.json());
app.use(express.urlencoded({ extended: true }));
app.use(express.text({ type: 'application/sparql-query' }));

function execQuery(query: string) {
  if (/^\s*PROVE\s/.test(query)) {
    const proveQuery = query.replace(/^\s*PROVE\s/, '');
    
    const res = execSync(`bash ./oxigraph/init.sh "${proveQuery.replace(/"/g, '\\"')}"`);
    console.log(JSON.stringify(JSON.parse(res.toString()), null, 2));

    console.log(proveQuery);
    return;
    // process.(0);
    // return execProve(query);
  }
  throw new Error('Only PROVE queries are supported');
  // return execQuery(query);
}

// SPARQL endpoint
app.get('/sparql', (req, res) => {
  const query = req.query.query as string;
  if (!query) {
    return res.status(400).json({ error: 'No query provided' });
  }

  try {
    // Parse and validate the SPARQL query
    console.log(query);
    
    const parser = new Parser();
    const parsedQuery = parser.parse(query);
    
    // TODO: Execute the query against your SPARQL store
    // For now, we'll just return the parsed query
    res.json({
      query: parsedQuery,
      message: 'Query parsed successfully'
    });
  } catch (error) {
    console.log(error);
    res.status(400).json({ error: 'Invalid SPARQL query' });
  }
});

// POST endpoint for SPARQL queries
app.post('/sparql', (req, res) => {
  console.log('posted to /sparql with', req.query, req.body);
  let query: string;
  
  // Handle different content types
  if (req.is('application/sparql-query')) {
    query = req.body;
  } else if (req.is('application/x-www-form-urlencoded')) {
    query = req.body.query;
  } else {
    return res.status(400).json({ error: 'Unsupported content type' });
  }

  if (!query) {
    return res.status(400).json({ error: 'No query provided' });
  }

  try {

    try {
      execQuery(query);
    } catch (error) {
      console.log(error);
      return res.status(400).json({ error: 'Invalid SPARQL query' });
    }


    // console.log(query, req);
    
    // Parse and validate the SPARQL query
    // const parser = new Parser();
    // const parsedQuery = parser.parse(query);
    
    // TODO: Execute the query against your SPARQL store
    // For now, we'll just return the parsed query
    res.json({
      // query: parsedQuery,
      message: 'Query parsed successfully'
    });
  } catch (error) {
    // console.log(error);
    res.status(400).json({ error: `Invalid SPARQL query [${error}]` });
  }
});

// Basic route
app.get('/', (req, res) => {
  res.json({ message: 'Welcome to the SPARQL Proof API' });
});

// Start server
app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
}); 
