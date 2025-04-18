Key pointers in this repo:

- reason.n3 contains the vocabulary for notation3 proof
- spl.ttl contains the SPIN vocabulary definition

Other possibly useful work:

- https://luisgalarraga.de/docs/TheWebConf2023IWPD_paper_480.pdf
  - https://gitlab.inria.fr/akatim/sparlqprov-demo
  - https://vldb.org/pvldb/vol14/p3389-galarraga.pdf

Computing provenance over SPARQL:

* https://vldb.org/pvldb/vol14/p3389-galarraga.pdf
* https://olafhartig.de/files/Hartig09_tSPARQL_Preprint.pdf (this is more probabilistic, not relevamt)
* https://dl.acm.org/doi/10.1145/2810037 (has equivalence definitions that might usefully establish how to constrain e.g. definitions of how things are joined)

For **tripleprov**

* TRIPLEPROV: https://dl.acm.org/doi/pdf/10.1145/2566486.2568014
* https://github.com/MarcinWylot/tripleprov_demo?tab=readme-ov-file

SPIN alternative over query algebra rather than syntax:

- https://ceur-ws.org/Vol-1644/paper12.pdf

---

https://mwylot.net/portfolio/rag-project-openai-neo4j-langchain-provenance/

When it comes to traceability and provenance in supply chain contexts; it is worth noting that https://w3c-ccg.github.io/traceability-vocab/ is being developed. This is more a common set of terms such as "shipping number"

We don't neeed to overcomplicate this. Really, all that proof annotated SPARQL needs to indicate is the triples that were used to derive a particular query.

Given that we can do V1 just using query re-writing to ensure that we pull out:

- The source facts given to the SPARQL query
- The the provenance if necessary
