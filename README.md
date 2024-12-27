# RDF Dataset Canonicalization in Rust

**WORK IN PROGRESS**

A Rust implementation of the [RDF Canonicalization algorithm version 1.0 (RDFC-1.0)](https://www.w3.org/TR/rdf-canon/).
Its purpose is for understanding and evaluating the specification, and it's **not intended for production use**.
Please be aware that it is currently very unstable, and breaking changes may occur without notice.

## Prerequisites

Please use Rust 1.70 or higher.

This implementation relies on [Oxrdf](https://crates.io/crates/oxrdf) for handling RDF data structures.
If you aim to canonicalize N-Quads documents rather than Oxrdf Datasets, you'll additionally require [Oxttl](https://crates.io/crates/oxttl) for N-Quads parsing.

## Usage

Add the following dependencies into your Cargo.toml:

```toml
[dependencies]
rdf-canon = "0.15.1"
oxrdf = "0.2.3"
oxttl = "0.1.4"
```

You can then use the `canonicalize` function to transform Oxrdf `Dataset` into canonical N-Quads.

### Example

```rust
use oxrdf::Dataset;
use oxttl::NQuadsParser;
use rdf_canon::canonicalize;
use std::io::Cursor;

let input = r#"_:e0 <http://example.org/vocab#next> _:e1 _:g .
_:e0 <http://example.org/vocab#prev> _:e2 _:g .
_:e1 <http://example.org/vocab#next> _:e2 _:g .
_:e1 <http://example.org/vocab#prev> _:e0 _:g .
_:e2 <http://example.org/vocab#next> _:e0 _:g .
_:e2 <http://example.org/vocab#prev> _:e1 _:g .
<urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" _:g .
"#;

let expected = r#"<urn:ex:s> <urn:ex:p> "\b\t\n\u000B\f\r\"\\\u007F" _:c14n0 .
_:c14n1 <http://example.org/vocab#next> _:c14n2 _:c14n0 .
_:c14n1 <http://example.org/vocab#prev> _:c14n3 _:c14n0 .
_:c14n2 <http://example.org/vocab#next> _:c14n3 _:c14n0 .
_:c14n2 <http://example.org/vocab#prev> _:c14n1 _:c14n0 .
_:c14n3 <http://example.org/vocab#next> _:c14n1 _:c14n0 .
_:c14n3 <http://example.org/vocab#prev> _:c14n2 _:c14n0 .
"#;

let input_quads = NQuadsParser::new()
    .for_reader(Cursor::new(input))
    .map(|x| x.unwrap());
let input_dataset = Dataset::from_iter(input_quads);
let canonicalized = canonicalize(&input_dataset).unwrap();

assert_eq!(canonicalized, expected);
```

## Advanced Usage

### Canonicalizing Graph and Quads

We provide `canonicalize_graph` and `canonicalize_quads` functions to canonicalize a `Graph` and `Vec<Quad>`, respectively.
For example, you can canonicalize RDF graph using `canonicalize_graph` as follows:

```rust
use oxrdf::Graph;
use oxttl::NTriplesParser;
use rdf_canon::canonicalize_graph;
use std::io::Cursor;

let input = r#"_:e0 <http://example.org/vocab#next> _:e1 .
_:e0 <http://example.org/vocab#prev> _:e2 .
_:e1 <http://example.org/vocab#next> _:e2 .
_:e1 <http://example.org/vocab#prev> _:e0 .
_:e2 <http://example.org/vocab#next> _:e0 .
_:e2 <http://example.org/vocab#prev> _:e1 .
<urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" .
"#;

let expected = r#"<urn:ex:s> <urn:ex:p> "\b\t\n\u000B\f\r\"\\\u007F" .
_:c14n0 <http://example.org/vocab#next> _:c14n2 .
_:c14n0 <http://example.org/vocab#prev> _:c14n1 .
_:c14n1 <http://example.org/vocab#next> _:c14n0 .
_:c14n1 <http://example.org/vocab#prev> _:c14n2 .
_:c14n2 <http://example.org/vocab#next> _:c14n1 .
_:c14n2 <http://example.org/vocab#prev> _:c14n0 .
"#;

let input_triples = NTriplesParser::new()
    .for_reader(Cursor::new(input))
    .map(|x| x.unwrap());
let input_graph = Graph::from_iter(input_triples);
let canonicalized = canonicalize_graph(&input_graph).unwrap();

assert_eq!(canonicalized, expected);
```

Here, we interpret the input graph as a dataset that includes it as the default graph,
then execute the canonicalization algorithm.
The output is the default graph obtained from the resulting canonicalized dataset.

### Canonicalized Dataset

The canonicalization algorithm can also return a [canonicalized dataset](https://www.w3.org/TR/rdf-canon/#dfn-canonicalized-dataset) instead of a serialized canonical N-Quads.

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
_:e0 <http://example.org/vocab#next> _:e1 _:g .
_:e0 <http://example.org/vocab#prev> _:e2 _:g .
_:e1 <http://example.org/vocab#next> _:e2 _:g .
_:e1 <http://example.org/vocab#prev> _:e0 _:g .
_:e2 <http://example.org/vocab#next> _:e0 _:g .
_:e2 <http://example.org/vocab#prev> _:e1 _:g .
"#;

let expected = HashMap::from([
    ("g".to_string(), "c14n0".to_string()),
    ("e0".to_string(), "c14n1".to_string()),
    ("e1".to_string(), "c14n2".to_string()),
    ("e2".to_string(), "c14n3".to_string()),
]);

let input_quads = NQuadsParser::new()
    .for_reader(Cursor::new(input))
    .map(|x| x.unwrap());
let input_dataset = Dataset::from_iter(input_quads);
let issued_identifiers_map = issue(&input_dataset).unwrap();

assert_eq!(issued_identifiers_map, expected);
```

### Use Alternate Hash Function

The [RDF Canonicalization algorithm version 1.0 (RDFC-1.0)](https://www.w3.org/TR/rdf-canon/) uses a hash function internally to determine the canonicalized dataset.
While SHA-256 is defined as its default hash function, it also permits the use of alternate hash functions if necessary.
If you want to use an internal hash function rather than SHA-256, you can use the `canonicalize_with` function with the desired hash as shown below.
Here, we use SHA-384 instead of the default SHA-256.

```rust
use oxrdf::Dataset;
use oxttl::NQuadsParser;
use rdf_canon::{canonicalize_with, CanonicalizationOptions};
use sha2::Sha384;
use std::io::Cursor;

let input = r#"_:e0 <http://example.org/vocab#next> _:e1 _:g .
_:e0 <http://example.org/vocab#prev> _:e2 _:g .
_:e1 <http://example.org/vocab#next> _:e2 _:g .
_:e1 <http://example.org/vocab#prev> _:e0 _:g .
_:e2 <http://example.org/vocab#next> _:e0 _:g .
_:e2 <http://example.org/vocab#prev> _:e1 _:g .
<urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" _:g .
"#;

let expected = r#"<urn:ex:s> <urn:ex:p> "\b\t\n\u000B\f\r\"\\\u007F" _:c14n0 .
_:c14n1 <http://example.org/vocab#next> _:c14n3 _:c14n0 .
_:c14n1 <http://example.org/vocab#prev> _:c14n2 _:c14n0 .
_:c14n2 <http://example.org/vocab#next> _:c14n1 _:c14n0 .
_:c14n2 <http://example.org/vocab#prev> _:c14n3 _:c14n0 .
_:c14n3 <http://example.org/vocab#next> _:c14n2 _:c14n0 .
_:c14n3 <http://example.org/vocab#prev> _:c14n1 _:c14n0 .
"#;

let input_quads = NQuadsParser::new()
    .for_reader(Cursor::new(input))
    .map(|x| x.unwrap());
let input_dataset = Dataset::from_iter(input_quads);
let options = CanonicalizationOptions::default();
let canonicalized = canonicalize_with::<Sha384>(&input_dataset, &options).unwrap();

assert_eq!(canonicalized, expected);
```

Note that the output canonicalized dataset can be different depending on the choice of hash function;
this is because the hash function affects the ordering of blank nodes, which in turn affects the output of the canonicalization algorithm.

### Protecting against poison dataset

As mentioned in [https://www.w3.org/TR/rdf-canon/#dataset-poisoning](https://www.w3.org/TR/rdf-canon/#dataset-poisoning),
there are some malicious datasets that can cause the canonicalization algorithm to consume a large amount of computing time.
We provide a call limit on the execution of the Hash N-Degree Quads algorithm to prevent it from running indefinitely due to poisoned data.
The default limit is set to 4000.
If you wish to raise or lower this limit, you can specify the limit using the `canonicalize_with` function as shown below.

```rust
use oxrdf::Dataset;
use oxttl::NQuadsParser;
use rdf_canon::{canonicalize_with, CanonicalizationOptions};
use sha2::Sha256;
use std::io::Cursor;

let input = r#"_:e0 <http://example.org/vocab#next> _:e1 _:g .
_:e0 <http://example.org/vocab#prev> _:e2 _:g .
_:e1 <http://example.org/vocab#next> _:e2 _:g .
_:e1 <http://example.org/vocab#prev> _:e0 _:g .
_:e2 <http://example.org/vocab#next> _:e0 _:g .
_:e2 <http://example.org/vocab#prev> _:e1 _:g .
<urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" _:g .
"#;
let expected = r#"<urn:ex:s> <urn:ex:p> "\b\t\n\u000B\f\r\"\\\u007F" _:c14n0 .
_:c14n1 <http://example.org/vocab#next> _:c14n2 _:c14n0 .
_:c14n1 <http://example.org/vocab#prev> _:c14n3 _:c14n0 .
_:c14n2 <http://example.org/vocab#next> _:c14n3 _:c14n0 .
_:c14n2 <http://example.org/vocab#prev> _:c14n1 _:c14n0 .
_:c14n3 <http://example.org/vocab#next> _:c14n1 _:c14n0 .
_:c14n3 <http://example.org/vocab#prev> _:c14n2 _:c14n0 .
"#;

let input_quads = NQuadsParser::new()
    .for_reader(Cursor::new(input))
    .map(|x| x.unwrap());
let input_dataset = Dataset::from_iter(input_quads);
let options = CanonicalizationOptions {
    hndq_call_limit: Some(10000),
};
let canonicalized = canonicalize_with::<Sha256>(&input_dataset, &options).unwrap();

assert_eq!(canonicalized, expected);
```

### Debug Logging Feature

The YAML-formatted debug log can be obtained by enabling the `log` feature.

```toml
[dependencies]
rdf-canon = { version = "0.15.1", features = ["log"] }
oxrdf = "0.2.3"
oxttl = "0.1.4"
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
        .for_reader(Cursor::new(input))
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
