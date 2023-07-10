# RDF Dataset Canonicalization in Rust

**WORK IN PROGRESS**

A Rust implementation of the [RDF Dataset Canonicalization](https://www.w3.org/TR/rdf-canon/) algorithm.
Its purpose is for understanding and evaluating the specification, and it's **not intended for production use**.

## Prerequisites

- [Oxrdf and Oxttl (from `next` branch of Oxigraph)](https://github.com/oxigraph/oxigraph/tree/next): These libraries are used to parse N-Quads and handle RDF data structures. Please note that Oxttl is currently only available in the `next` branch of Oxigraph. We'll update this information when Oxigraph officially releases its next version.

## Usage

Add the following dependencies into your Cargo.toml:
(**Current limitation**: dependency on `next` branch of Oxigraph to use `oxttl`; this will be updated once Oxigraph v0.4 is released)

```toml
[dependencies]
rdf-canon = { git = "https://github.com/yamdan/rdf-canon-rust.git" }
oxrdf = { git = "https://github.com/oxigraph/oxigraph.git", branch = "next" }
oxttl = { git = "https://github.com/oxigraph/oxigraph.git", branch = "next" }
```

You can then use the `rdf_canon::canonicalize` to convert OxRDF `Dataset` into canonical N-Quads.

### Example

```rust
use oxrdf::Dataset;
use oxttl::NQuadsParser;
use rdf_canon::canonicalize;
use std::io::Cursor;

let input = r#"<urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" .
_:e0 <http://example.org/vocab#next> _:e1 .
_:e0 <http://example.org/vocab#prev> _:e2 .
_:e1 <http://example.org/vocab#next> _:e2 .
_:e1 <http://example.org/vocab#prev> _:e0 .
_:e2 <http://example.org/vocab#next> _:e0 .
_:e2 <http://example.org/vocab#prev> _:e1 .
"#;
let expected = r#"<urn:ex:s> <urn:ex:p> "\b\t\n\u000B\f\r\"\\\u007F" .
_:c14n0 <http://example.org/vocab#next> _:c14n2 .
_:c14n0 <http://example.org/vocab#prev> _:c14n1 .
_:c14n1 <http://example.org/vocab#next> _:c14n0 .
_:c14n1 <http://example.org/vocab#prev> _:c14n2 .
_:c14n2 <http://example.org/vocab#next> _:c14n1 .
_:c14n2 <http://example.org/vocab#prev> _:c14n0 .
"#;

let input_quads = NQuadsParser::new()
    .parse_from_read(Cursor::new(input))
    .into_iter()
    .map(|x| x.unwrap());
let input_dataset = Dataset::from_iter(input_quads);
let canonicalized = canonicalize(&input_dataset).unwrap();

assert_eq!(canonicalized, expected);
```

## Advanced Usage

### Canonicalized Dataset

The canonicalization algorithm can also return a [canonicalized dataset](https://www.w3.org/TR/rdf-canon/#dfn-canonicalized-dataset).

[RDF Dataset Canonicalization](https://www.w3.org/TR/rdf-canon/)

> A [canonicalized dataset](https://www.w3.org/TR/rdf-canon/#dfn-canonicalized-dataset) is the combination of the following:
> 
> -   an [RDF dataset](https://www.w3.org/TR/rdf11-concepts/#dfn-rdf-dataset) — the [input dataset](https://www.w3.org/TR/rdf-canon/#dfn-input-dataset),
> -   the [input blank node identifier map](https://www.w3.org/TR/rdf-canon/#dfn-input-blank-node-identifier-map) — mapping [blank nodes](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node) in the input dataset to [blank node identifiers](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node-identifier), and
> -   the [issued identifiers map](https://www.w3.org/TR/rdf-canon/#dfn-issued-identifiers-map) from the [canonical issuer](https://www.w3.org/TR/rdf-canon/#dfn-canonical-issuer) — mapping identifiers in the input dataset to canonical identifiers
> 
> A concrete serialization of a [canonicalized dataset](https://www.w3.org/TR/rdf-canon/#dfn-canonicalized-dataset) _MUST_ label all [blank nodes](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node) using the canonical [blank node identifiers](https://www.w3.org/TR/rdf11-concepts/#dfn-blank-node-identifier).

If you prefer to work with a [canonicalized dataset](https://www.w3.org/TR/rdf-canon/#dfn-canonicalized-dataset),
you can use `issue` function to obtain the [issued identifiers map](https://www.w3.org/TR/rdf-canon/#dfn-issued-identifiers-map),
which can be combined with the [input dataset](https://www.w3.org/TR/rdf-canon/#dfn-input-dataset)
(containing the embedded [input blank node identifier map](https://www.w3.org/TR/rdf-canon/#dfn-input-blank-node-identifier-map) in this implementation)
to construct the [canonicalized dataset](https://www.w3.org/TR/rdf-canon/#dfn-canonicalized-dataset).

```rust
use oxrdf::Dataset;
use oxttl::NQuadsParser;
use rdf_canon::issue;
use std::collections::HashMap;
use std::io::Cursor;

let input = r#"
_:e0 <http://example.org/vocab#next> _:e1 .
_:e0 <http://example.org/vocab#prev> _:e2 .
_:e1 <http://example.org/vocab#next> _:e2 .
_:e1 <http://example.org/vocab#prev> _:e0 .
_:e2 <http://example.org/vocab#next> _:e0 .
_:e2 <http://example.org/vocab#prev> _:e1 .
"#;
let expected = HashMap::from([
    ("e0".to_string(), "c14n0".to_string()),
    ("e1".to_string(), "c14n2".to_string()),
    ("e2".to_string(), "c14n1".to_string()),
]);

let input_quads = NQuadsParser::new()
    .parse_from_read(Cursor::new(input))
    .into_iter()
    .map(|x| x.unwrap());
let input_dataset = Dataset::from_iter(input_quads);
let issued_identifiers_map = issue(&input_dataset).unwrap();

assert_eq!(issued_identifiers_map, expected);
```

### Protecting against poison dataset

As mentioned in [https://www.w3.org/TR/rdf-canon/#dataset-poisoning](https://www.w3.org/TR/rdf-canon/#dataset-poisoning),
there are some malicious datasets that can cause the canonicalization algorithm to consume a large amount of computing time.
We provide a call limit on the execution of the Hash N-Degree Quads algorithm to prevent it from running indefinitely due to poisoned data.
The default limit is set to 4000.
If you wish to raise or lower this limit, you can specify the limit using the `canonicalize_with_options` function as shown below.

```rust
let options = CanonicalizationOptions {
    hndq_call_limit: Some(10000),
};
let canonicalized = canonicalize_with_options(&input_dataset, &options).unwrap();    
```

### Debug Logging Feature

The YAML-formatted debug log can be obtained by enabling the `log` feature.

```toml
[dependencies]
rdf-canon = { git = "https://github.com/yamdan/rdf-canon-rust.git", features = ["log"] }
oxrdf = { git = "https://github.com/oxigraph/oxigraph.git", branch = "next" }
oxttl = { git = "https://github.com/oxigraph/oxigraph.git", branch = "next" }
```

```rust
use oxrdf::Dataset;
use oxttl::NQuadsParser;
use rdf_canon::{canonicalize, logger::YamlLayer};
use std::io::Cursor;

// setup for debug logger
use tracing::metadata::LevelFilter;
use tracing_subscriber::prelude::*;
const INDENT_WIDTH: usize = 2;
fn init_logger(level: tracing::Level) {
    let _ = tracing_subscriber::registry()
        .with(YamlLayer::new(INDENT_WIDTH).with_filter(LevelFilter::from_level(level)))
        .try_init();
}

fn main() {
    // initialize debug logger
    init_logger(tracing::Level::DEBUG);

    let input = r#"_:e0 <http://example.com/#p1> _:e1 .
_:e1 <http://example.com/#p2> "Foo" .
"#;
    let expected = r#"_:c14n0 <http://example.com/#p1> _:c14n1 .
_:c14n1 <http://example.com/#p2> "Foo" .
"#;

    // get dataset from N-Quads document
    let input_quads = NQuadsParser::new()
        .parse_from_read(Cursor::new(input))
        .into_iter()
        .map(|x| x.unwrap());
    let input_dataset = Dataset::from_iter(input_quads);

    // canonicalize the dataset
    let canonicalized = canonicalize(&input_dataset).unwrap();

    assert_eq!(canonicalized, expected);
}
```

The above code generates the following debug log:

```yaml
ca:
  log point: Entering the canonicalization function (4.4.3).
  ca.2:
    log point: Extract quads for each bnode (4.4.3 (2)).
    Bnode to quads:
      e0:
        - _:e0 <http://example.com/#p1> _:e1 .
      e1:
        - _:e0 <http://example.com/#p1> _:e1 .
        - _:e1 <http://example.com/#p2> "Foo" .
  ca.3:
    log point: Calculated first degree hashes (4.4.3 (3)).
    with:
      - identifier: e0
        h1dq:
          log point: Hash First Degree Quads function (4.6.3).
          nquads:
            - _:a <http://example.com/#p1> _:z .
          hash: 24da9a4406b4e66dffa10ad3d4d6dddc388fbf193bb124e865158ef419893957
      - identifier: e1
        h1dq:
          log point: Hash First Degree Quads function (4.6.3).
          nquads:
            - _:z <http://example.com/#p1> _:a .
            - _:a <http://example.com/#p2> "Foo" .
          hash: a994e40b576809985bc0f389308cd9d552fd7c89d028c163848a6b2d33a8583a
  ca.4:
    log point: Create canonical replacements for hashes mapping to a single node (4.4.3 (4)).
    with:
      - identifier: e0
    hash: 24da9a4406b4e66dffa10ad3d4d6dddc388fbf193bb124e865158ef419893957
    canonical label: c14n0
      - identifier: e1
    hash: a994e40b576809985bc0f389308cd9d552fd7c89d028c163848a6b2d33a8583a
    canonical label: c14n1
  ca.5:
    log point: Calculate hashes for identifiers with shared hashes (4.4.3 (5)).
    with:
  ca.6:
    log point: Replace original with canonical labels (4.4.3 (6)).
    issued identifiers map: {e0: c14n0, e1: c14n1}
    hndq_call_counter:  { counter: 0, limit: 4000 }
```

## Changelog

### v0.10.0

- add `*_quads` APIs to allow input in `Vec<quad>` instead of `Dataset`

### v0.9.1

- fix debug log indentations

### v0.9.0

- modify the test code to use all official test cases from w3c/rdf-canon with their test manifest
- extract `api` module from `canon`
- add info-level log message to notify if there are duplicate hashes in `hash path list`
- fix debug log indentations

### v0.8.0

- update algorithm interfaces to support the canonicalized dataset as additional output
- remove `indexmap` dependency
- update comments for latest specification changes

### v0.7.0

- Add HNDQ call limit as a countermeasure against poison dataset attack by  restricting the maximum number of calls to the Hash N-Degree Quads (HNDQ) algorithm

### v0.6.0

- Use `oxttl` as a N-Quads parser to avoid the dependency on the whole `oxigraph`
- Make logger and YamlLayer public for external crates to use them when debugging
- import the latest-updated test cases from w3c/rdf-canon

### v0.5.0

- Turn logger into a feature: the logger can now be optionally included in our builds, depending on the requirements of each specific build
- Fix logger to avoid unexpected panic
- update `.gitignore`

### v0.4.0

- Add `serialize` function to serialize a normalized dataset into a canonical N-Quads document
- Add an example into README

### v0.3.0

- Revise input/output of canonicalization using OxRDF `Dataset` instead of `Vec<Quad>`
- Some optimizations
- Fix bug related to module scopes

### v0.2.0

We have moved away from using our ad-hoc N-Quads parser and RDF data structures and have instead adopted the use of [Oxigraph (and its internal OxRDF)](https://github.com/oxigraph/oxigraph).
This change makes it easier to canonicalize Oxigraph's internal data and output.
However, the current version of Oxigraph does not provide the latest [Canonical N-Triples](https://w3c.github.io/rdf-n-triples/spec/#canonical-ntriples) and [Canonical N-Quads](https://w3c.github.io/rdf-n-quads/spec/#canonical-quads) representations.
Therefore, we are currently relying on our [forked Oxigraph and Oxrdf](https://github.com/yamdan/oxigraph) that supports canonical representations.

### v0.1.0

Initial release. It does not pass [test060](https://w3c.github.io/rdf-canon/tests/#manifest-urdna2015#test060) since it uses an ad-hoc N-Quads parser and serializer. (See [#1](https://github.com/yamdan/rdf-canon-rust/issues/1))
