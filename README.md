# RDF Dataset Canonicalization in Rust

**WORK IN PROGRESS**

A Rust implementation of the [RDF Dataset Canonicalization](https://www.w3.org/TR/rdf-canon/) algorithm.
The purpose of this implementation is only to understand and evaluate the specification and is **not intended for production use**.

## Prerequisites

- [Oxrdf and Oxttl (from `next` branch of Oxigraph)](https://github.com/oxigraph/oxigraph/tree/next): We use Oxttl to parse N-Quads and Oxrdf to deal with RDF data structures. Note that Oxttl is currently only available in `next` branch of Oxigraph; we will update it as soon as Oxigraph releases v0.4.

## Usage

Add the following dependencies into your Cargo.toml:
(**current limitation**: depending `next` branch of Oxigraph to use `oxttl`; will be updated when Oxigraph releases v0.4)

```toml
[dependencies]
rdf-canon = { git = "https://github.com/yamdan/rdf-canon-rust.git" }
oxrdf = { git = "https://github.com/oxigraph/oxigraph.git", branch = "next" }
oxttl = { git = "https://github.com/oxigraph/oxigraph.git", branch = "next" }
```

Then you can use:
- `rdf_canon::canonicalize` to canonicalize OxRDF `Dataset`, and
- `rdf_canon::serialize` to serialize the canonicalized `Dataset` into a canonical N-Quads

## Example

```rust
use oxrdf::Dataset;
use oxttl::NQuadsParser;
use rdf_canon::{canonicalize, serialize};
use std::io::Cursor;

fn main() {
    let input_doc = r#"<urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" .  # test for canonical N-Quads
_:e0 <http://example.org/vocab#next> _:e1 .
_:e0 <http://example.org/vocab#prev> _:e2 .
_:e1 <http://example.org/vocab#next> _:e2 .
_:e1 <http://example.org/vocab#prev> _:e0 .
_:e2 <http://example.org/vocab#next> _:e0 .
_:e2 <http://example.org/vocab#prev> _:e1 .
"#;
    let expected_canonicalized_doc = r#"<urn:ex:s> <urn:ex:p> "\b\t\n\u000B\f\r\"\\\u007F" .
_:c14n0 <http://example.org/vocab#next> _:c14n2 .
_:c14n0 <http://example.org/vocab#prev> _:c14n1 .
_:c14n1 <http://example.org/vocab#next> _:c14n0 .
_:c14n1 <http://example.org/vocab#prev> _:c14n2 .
_:c14n2 <http://example.org/vocab#next> _:c14n1 .
_:c14n2 <http://example.org/vocab#prev> _:c14n0 .
"#;

    // get dataset from N-Quads document
    let quads = NQuadsParser::new()
        .parse_from_read(Cursor::new(input_doc))
        .into_iter()
        .map(|x| x.unwrap());
    let input_dataset = Dataset::from_iter(quads);

    // canonicalize the dataset
    let canonicalized_dataset = canonicalize(&input_dataset).unwrap();
    let canonicalized_doc = serialize(canonicalized_dataset);

    assert_eq!(canonicalized_doc, expected_canonicalized_doc);
}
```

## Logging Feature for Debug

You can get the YAML-formatted debug log to enable `log` feature.

```rust
use oxrdf::Dataset;
use oxttl::NQuadsParser;
use rdf_canon::{canonicalize, logger::YamlLayer, serialize};
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

    let input_doc = r#"_:e0 <http://example.com/#p1> _:e1 .
_:e1 <http://example.com/#p2> "Foo" .
"#;
    let expected_canonicalized_doc = r#"_:c14n0 <http://example.com/#p1> _:c14n1 .
_:c14n1 <http://example.com/#p2> "Foo" .
"#;

    // get dataset from N-Quads document
    let quads = NQuadsParser::new()
        .parse_from_read(Cursor::new(input_doc))
        .into_iter()
        .map(|x| x.unwrap());
    let input_dataset = Dataset::from_iter(quads);

    // canonicalize the dataset
    let canonicalized_dataset = canonicalize(&input_dataset).unwrap();
    let canonicalized_doc = serialize(canonicalized_dataset);

    assert_eq!(canonicalized_doc, expected_canonicalized_doc);
}
```

The above code generates the following debug log:

```yaml
ca:
  log point: Entering the canonicalization function (4.5.3).
  ca.2:
    log point: Extract quads for each bnode (4.5.3 (2)).
    Bnode to quads:
      e0:
        - _:e0 <http://example.com/#p1> _:e1 .
      e1:
        - _:e0 <http://example.com/#p1> _:e1 .
        - _:e1 <http://example.com/#p2> "Foo" .
  ca.3:
    log point: Calculated first degree hashes (4.5.3 (3)).
    with:
      - identifier: e0
        h1dq:
          log point: Hash First Degree Quads function (4.7.3).
          nquads:
            - _:a <http://example.com/#p1> _:z .
          hash: 24da9a4406b4e66dffa10ad3d4d6dddc388fbf193bb124e865158ef419893957
      - identifier: e1
        h1dq:
          log point: Hash First Degree Quads function (4.7.3).
          nquads:
            - _:z <http://example.com/#p1> _:a .
            - _:a <http://example.com/#p2> "Foo" .
          hash: a994e40b576809985bc0f389308cd9d552fd7c89d028c163848a6b2d33a8583a
  ca.4:
    log point: Create canonical replacements for hashes mapping to a single node (4.5.3 (4)).
    with:
      - identifier: e0
    hash: 24da9a4406b4e66dffa10ad3d4d6dddc388fbf193bb124e865158ef419893957
    canonical label: c14n0
      - identifier: e1
    hash: a994e40b576809985bc0f389308cd9d552fd7c89d028c163848a6b2d33a8583a
    canonical label: c14n1
  ca.5:
    log point: Calculate hashes for identifiers with shared hashes (4.5.3 (5)).
    with:
  ca.6:
    log point: Replace original with canonical labels (4.5.3 (6)).
    issued identifiers map: {e0: c14n0, e1: c14n1}
```

## Changelog

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
