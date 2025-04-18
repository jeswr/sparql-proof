import express from 'express';
import { Parser } from 'sparqljs';

const app = express();
const port = process.env.PORT || 3000;

// Middleware
app.use(express.json());
app.use(express.urlencoded({ extended: true }));

// SPARQL endpoint
app.get('/sparql', (req, res) => {
  const query = req.query.query as string;
  if (!query) {
    return res.status(400).json({ error: 'No query provided' });
  }

  try {
    // Parse and validate the SPARQL query
    const parser = new Parser();
    const parsedQuery = parser.parse(query);
    
    // TODO: Execute the query against your SPARQL store
    // For now, we'll just return the parsed query
    res.json({
      query: parsedQuery,
      message: 'Query parsed successfully'
    });
  } catch (error) {
    res.status(400).json({ error: 'Invalid SPARQL query' });
  }
});

// POST endpoint for SPARQL queries
app.post('/sparql', (req, res) => {
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
    // Parse and validate the SPARQL query
    const parser = new Parser();
    const parsedQuery = parser.parse(query);
    
    // TODO: Execute the query against your SPARQL store
    // For now, we'll just return the parsed query
    res.json({
      query: parsedQuery,
      message: 'Query parsed successfully'
    });
  } catch (error) {
    res.status(400).json({ error: 'Invalid SPARQL query' });
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
