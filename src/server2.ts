// TODO: Investigate using the Comunica HTTP service here
// https://github.com/comunica/comunica/blob/278774283d3eabddbc6374acdcd5677ccc8fd465/packages/actor-init-query/lib/HttpServiceSparqlEndpoint.ts

// #!/usr/bin/env node
import { HttpServiceSparqlEndpoint, QueryEngineBase, IHttpServiceSparqlEndpointArgs } from '@comunica/actor-init-query';

class MyHttpServiceSparqlEndpoint extends HttpServiceSparqlEndpoint {
  constructor(args: IHttpServiceSparqlEndpointArgs) {
    super(args);
    this.engine = new QueryEngineBase(args);
  }

  async query(query: string) {
    const engine = await this.engine;
    const result = await engine.query(query);
    return result;
  }
}



const process: NodeJS.Process = require('process/');

// eslint-disable-next-line node/no-path-concat
const defaultConfigPath = `${__dirname}/../config/config-default.json`;

// eslint-disable-next-line node/no-path-concat
HttpServiceSparqlEndpoint.runArgsInProcess(process.argv.slice(2), process.stdout, process.stderr, `${__dirname}/../`, process.env, defaultConfigPath, code => process.exit(code))
  .catch(error => process.stderr.write(`${error.message}/n`));