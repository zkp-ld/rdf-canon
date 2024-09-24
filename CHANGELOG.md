# Changelog

## [0.15.0] - 2024-09-24

- Updated `oxrdf` and `oxttl` dependencies

## [0.15.0-alpha.6] - 2024-08-23

- Updated `oxrdf`, `oxttl`, and `itertools` dependencies

## [0.15.0-alpha.5] - 2024-03-18

- Updated `oxrdf` and `oxttl` dependencies

## [0.15.0-alpha.4] - 2024-02-26

- Added a test feature for generating W3C EARL test report

## [0.15.0-alpha.3] - 2024-02-26

- Added two tests from w3c/rdf-canon
- Updated test manifest
- Updated `oxrdf` and `oxttl` dependency

## [0.15.0-alpha.2] - 2024-01-09

- Updated README

## [0.15.0-alpha.1] - 2024-01-08

- Updated `oxrdf` and `oxttl` from GitHub source to pre-release version on crates.io
- Published pre-release version on crates.io

## [0.14.1] - 2023-12-20

- Added an explicit lifetime name to an associated constant to align with https://github.com/rust-lang/rust/issues/115010
- Updated `itertools` dependency
- Separeted CHANGELOG from README

## [0.14.0] - 2023-09-01

- enables selection of internal hash function
- update `oxrdf` and `oxttl` to the latest ones (as a result, Rust needs to be 1.70 or higher)

## [0.13.0] - 2023-08-02

- add `sort` and `sort_graph` functions to sort canonicalized `Dataset` and `Graph` to get `Vec<Quad>` and `Vec<Triple>`, respectively

## [0.12.0] - 2023-08-02

- add `*_graph` APIs to allow `Graph` as input
- rename `*_with_options` APIs with simplified `*_with`

## [0.11.0] - 2023-07-27

- re-export `serialize` function to enable direct use by users
- update `oxrdf` and `oxttl` dependencies
- add more detailed explanation about `oxrdf` and `oxttl` dependencies to the README

## [0.10.1] - 2023-07-10

- Make `oxrdf` and `oxttl` point to specific commits on GitHub

## [0.10.0] - 2023-07-10

- add `*_quads` APIs to allow input in `Vec<quad>` instead of `Dataset`

## [0.9.1] - 2023-07-04

- fix debug log indentations

## [0.9.0] - 2023-07-03

- modify the test code to use all official test cases from w3c/rdf-canon with their test manifest
- extract `api` module from `canon`
- add info-level log message to notify if there are duplicate hashes in `hash path list`
- fix debug log indentations

## [0.8.0] - 2023-06-26

- update algorithm interfaces to support the canonicalized dataset as additional output
- remove `indexmap` dependency
- update comments for latest specification changes

## [0.7.0] - 2023-06-23

- Add HNDQ call limit as a countermeasure against poison dataset attack by  restricting the maximum number of calls to the Hash N-Degree Quads (HNDQ) algorithm

## [0.6.0] - 2023-06-21

- Use `oxttl` as a N-Quads parser to avoid the dependency on the whole `oxigraph`
- Make logger and YamlLayer public for external crates to use them when debugging
- import the latest-updated test cases from w3c/rdf-canon

## [0.5.0] - 2023-05-15

- Turn logger into a feature: the logger can now be optionally included in our builds, depending on the requirements of each specific build
- Fix logger to avoid unexpected panic
- update `.gitignore`

## [0.4.0] - 2023-04-26

- Add `serialize` function to serialize a normalized dataset into a canonical N-Quads document
- Add an example into README

## [0.3.0] - 2023-04-26

- Revise input/output of canonicalization using OxRDF `Dataset` instead of `Vec<Quad>`
- Some optimizations
- Fix bug related to module scopes

## [0.2.0] - 2023-04-24

We have moved away from using our ad-hoc N-Quads parser and RDF data structures and have instead adopted the use of [Oxigraph (and its internal OxRDF)](https://github.com/oxigraph/oxigraph).
This change makes it easier to canonicalize Oxigraph's internal data and output.
However, the current version of Oxigraph does not provide the latest [Canonical N-Triples](https://w3c.github.io/rdf-n-triples/spec/#canonical-ntriples) and [Canonical N-Quads](https://w3c.github.io/rdf-n-quads/spec/#canonical-quads) representations.
Therefore, we are currently relying on our [forked Oxigraph and Oxrdf](https://github.com/yamdan/oxigraph) that supports canonical representations.

## [0.1.0] - 2023-01-11

Initial release. It does not pass [test060](https://w3c.github.io/rdf-canon/tests/#manifest-urdna2015#test060) since it uses an ad-hoc N-Quads parser and serializer. (See [#1](https://github.com/yamdan/rdf-canon-rust/issues/1))
</details>
