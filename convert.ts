import { write } from "@jeswr/pretty-turtle";
import deref from "rdf-dereference-store";
import fs from "fs";

async function main() {
  const store = await deref("./spa.jsonld", { localFiles: true });
  store.prefixes["spa"] = "http://meta-sparql.org/vocab/spa#";
  store.prefixes["rdfs"] = "http://www.w3.org/2000/01/rdf-schema#";
  store.prefixes["owl"] = "http://www.w3.org/2002/07/owl#";
  store.prefixes["rdf"] = "http://www.w3.org/1999/02/22-rdf-syntax-ns#";
  store.prefixes["sp"] = "http://spinrdf.org/sp#";
  fs.writeFileSync("./spa.ttl", await write([...store.store], { prefixes: store.prefixes }));
}

main();
