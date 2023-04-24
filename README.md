# RDF Dataset Canonicalization in Rust

**WORK IN PROGRESS**

A Rust implementation of the [RDF Dataset Canonicalization](https://www.w3.org/TR/rdf-canon/) algorithm.
The purpose of this implementation is only to understand and evaluate the specification and is **not intended for production use**.

## Prerequisites

- [Oxigraph and Oxrdf (forked)](https://github.com/yamdan/oxigraph): A modified version with changes to the Literal generation section to produce Canonical N-Triples and N-Quads. A Pull Request is planned to be submitted to the original repository.
- libclang: Required for building Oxigraph

## Usage

TBD

## Changelog

### Version 0.2.0

We have moved away from using our ad-hoc N-Quads parser and RDF data structures, and have instead adopted the use of [Oxigraph (and its internal OxRDF)](https://github.com/oxigraph/oxigraph).
This change makes it easier to canonicalize Oxigraph's internal data and output.
However, the current version of Oxigraph does not provide the latest [Canonical N-Triples](https://w3c.github.io/rdf-n-triples/spec/#canonical-ntriples) and [Canonical N-Quads](https://w3c.github.io/rdf-n-quads/spec/#canonical-quads) representations.
Therefore, we are currently relying on our [forked Oxigraph and Oxrdf](https://github.com/yamdan/oxigraph) that supports canonical representations.

### Version 0.1.0

Initial release. It does not pass [test060](https://w3c.github.io/rdf-canon/tests/#manifest-urdna2015#test060) since it uses ad-hoc N-Quads parser and serializer. (See #1)
