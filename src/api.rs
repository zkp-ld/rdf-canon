use crate::{
    canon::{canonicalize_core, serialize, serialize_graph},
    counter::{HndqCallCounter, SimpleHndqCallCounter},
    CanonicalizationError,
};
use digest::Digest;
use oxrdf::{
    BlankNode, BlankNodeRef, Dataset, Graph, GraphName, GraphNameRef, Quad, QuadRef, Subject,
    SubjectRef, Term, TermRef, Triple, TripleRef,
};
use sha2::Sha256;
use std::collections::HashMap;

/// Returns the serialized canonical form of the canonicalized dataset,
/// where any blank nodes in the input dataset are assigned deterministic identifiers.
///
/// # Examples
///
/// ```
/// use oxrdf::Dataset;
/// use oxttl::NQuadsParser;
/// use rdf_canon::canonicalize;
/// use std::io::Cursor;

/// let input = r#"_:e0 <http://example.org/vocab#next> _:e1 _:g .
/// _:e0 <http://example.org/vocab#prev> _:e2 _:g .
/// _:e1 <http://example.org/vocab#next> _:e2 _:g .
/// _:e1 <http://example.org/vocab#prev> _:e0 _:g .
/// _:e2 <http://example.org/vocab#next> _:e0 _:g .
/// _:e2 <http://example.org/vocab#prev> _:e1 _:g .
/// <urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" _:g .
/// "#;
/// let expected = r#"<urn:ex:s> <urn:ex:p> "\b\t\n\u000B\f\r\"\\\u007F" _:c14n0 .
/// _:c14n1 <http://example.org/vocab#next> _:c14n2 _:c14n0 .
/// _:c14n1 <http://example.org/vocab#prev> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#next> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#prev> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#next> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#prev> _:c14n2 _:c14n0 .
/// "#;
///
/// let input_quads = NQuadsParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap());
/// let input_dataset = Dataset::from_iter(input_quads);
/// let canonicalized = canonicalize(&input_dataset).unwrap();
///
/// assert_eq!(canonicalized, expected);
/// ```
pub fn canonicalize(input_dataset: &Dataset) -> Result<String, CanonicalizationError> {
    let options = CanonicalizationOptions::default();
    canonicalize_with::<Sha256>(input_dataset, &options)
}

/// Returns the serialized canonical form of the canonicalized dataset,
/// where any blank nodes in the input graph are assigned deterministic identifiers.
///
/// # Examples
///
/// ```
/// use oxrdf::Graph;
/// use oxttl::NTriplesParser;
/// use rdf_canon::canonicalize_graph;
/// use std::io::Cursor;

/// let input = r#"_:e0 <http://example.org/vocab#next> _:e1 .
/// _:e0 <http://example.org/vocab#prev> _:e2 .
/// _:e1 <http://example.org/vocab#next> _:e2 .
/// _:e1 <http://example.org/vocab#prev> _:e0 .
/// _:e2 <http://example.org/vocab#next> _:e0 .
/// _:e2 <http://example.org/vocab#prev> _:e1 .
/// <urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" .
/// "#;
/// let expected = r#"<urn:ex:s> <urn:ex:p> "\b\t\n\u000B\f\r\"\\\u007F" .
/// _:c14n0 <http://example.org/vocab#next> _:c14n2 .
/// _:c14n0 <http://example.org/vocab#prev> _:c14n1 .
/// _:c14n1 <http://example.org/vocab#next> _:c14n0 .
/// _:c14n1 <http://example.org/vocab#prev> _:c14n2 .
/// _:c14n2 <http://example.org/vocab#next> _:c14n1 .
/// _:c14n2 <http://example.org/vocab#prev> _:c14n0 .
/// "#;
///
/// let input_triples = NTriplesParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap());
/// let input_graph = Graph::from_iter(input_triples);
/// let canonicalized = canonicalize_graph(&input_graph).unwrap();
///
/// assert_eq!(canonicalized, expected);
/// ```
pub fn canonicalize_graph(input_graph: &Graph) -> Result<String, CanonicalizationError> {
    let options = CanonicalizationOptions::default();
    canonicalize_graph_with::<Sha256>(input_graph, &options)
}

/// Returns the serialized canonical form of the canonicalized dataset,
/// where any blank nodes in the input quads are assigned deterministic identifiers.
///
/// # Examples
///
/// ```
/// use oxrdf::Quad;
/// use oxttl::NQuadsParser;
/// use rdf_canon::canonicalize_quads;
/// use std::io::Cursor;

/// let input = r#"_:e0 <http://example.org/vocab#next> _:e1 _:g .
/// _:e0 <http://example.org/vocab#prev> _:e2 _:g .
/// _:e1 <http://example.org/vocab#next> _:e2 _:g .
/// _:e1 <http://example.org/vocab#prev> _:e0 _:g .
/// _:e2 <http://example.org/vocab#next> _:e0 _:g .
/// _:e2 <http://example.org/vocab#prev> _:e1 _:g .
/// <urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" _:g .
/// "#;
/// let expected = r#"<urn:ex:s> <urn:ex:p> "\b\t\n\u000B\f\r\"\\\u007F" _:c14n0 .
/// _:c14n1 <http://example.org/vocab#next> _:c14n2 _:c14n0 .
/// _:c14n1 <http://example.org/vocab#prev> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#next> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#prev> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#next> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#prev> _:c14n2 _:c14n0 .
/// "#;
///
/// let input_quads: Vec<Quad> = NQuadsParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap())
///     .collect();
/// let canonicalized = canonicalize_quads(&input_quads).unwrap();
///
/// assert_eq!(canonicalized, expected);
/// ```
pub fn canonicalize_quads(input_quads: &[Quad]) -> Result<String, CanonicalizationError> {
    let options = CanonicalizationOptions::default();
    canonicalize_quads_with::<Sha256>(input_quads, &options)
}

#[derive(Default)]
pub struct CanonicalizationOptions {
    pub hndq_call_limit: Option<usize>,
}

/// Given some options (e.g., call limit),
/// returns the serialized canonical form of the canonicalized dataset,
/// where any blank nodes in the input dataset are assigned deterministic identifiers.
///
/// # Examples
///
/// ```
/// use oxrdf::Dataset;
/// use oxttl::NQuadsParser;
/// use rdf_canon::{canonicalize_with, CanonicalizationOptions};
/// use sha2::Sha256;
/// use std::io::Cursor;

/// let input = r#"_:e0 <http://example.org/vocab#next> _:e1 _:g .
/// _:e0 <http://example.org/vocab#prev> _:e2 _:g .
/// _:e1 <http://example.org/vocab#next> _:e2 _:g .
/// _:e1 <http://example.org/vocab#prev> _:e0 _:g .
/// _:e2 <http://example.org/vocab#next> _:e0 _:g .
/// _:e2 <http://example.org/vocab#prev> _:e1 _:g .
/// <urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" _:g .
/// "#;
/// let expected = r#"<urn:ex:s> <urn:ex:p> "\b\t\n\u000B\f\r\"\\\u007F" _:c14n0 .
/// _:c14n1 <http://example.org/vocab#next> _:c14n2 _:c14n0 .
/// _:c14n1 <http://example.org/vocab#prev> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#next> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#prev> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#next> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#prev> _:c14n2 _:c14n0 .
/// "#;
///
/// let input_quads = NQuadsParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap());
/// let input_dataset = Dataset::from_iter(input_quads);
/// let options = CanonicalizationOptions {
///     hndq_call_limit: Some(10000),
/// };
/// let canonicalized = canonicalize_with::<Sha256>(&input_dataset, &options).unwrap();
///
/// assert_eq!(canonicalized, expected);
/// ```
pub fn canonicalize_with<D: Digest>(
    input_dataset: &Dataset,
    options: &CanonicalizationOptions,
) -> Result<String, CanonicalizationError> {
    let issued_identifiers_map = issue_with::<D>(input_dataset, options)?;
    let relabeled_dataset = relabel(input_dataset, &issued_identifiers_map)?;
    Ok(serialize(&relabeled_dataset))
}

/// Given some options (e.g., call limit),
/// returns the serialized canonical form of the canonicalized dataset,
/// where any blank nodes in the input graph are assigned deterministic identifiers.
///
/// # Examples
///
/// ```
/// use oxrdf::Graph;
/// use oxttl::NTriplesParser;
/// use rdf_canon::{canonicalize_graph_with, CanonicalizationOptions};
/// use sha2::Sha256;
/// use std::io::Cursor;

/// let input = r#"_:e0 <http://example.org/vocab#next> _:e1 .
/// _:e0 <http://example.org/vocab#prev> _:e2 .
/// _:e1 <http://example.org/vocab#next> _:e2 .
/// _:e1 <http://example.org/vocab#prev> _:e0 .
/// _:e2 <http://example.org/vocab#next> _:e0 .
/// _:e2 <http://example.org/vocab#prev> _:e1 .
/// <urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" .
/// "#;
/// let expected = r#"<urn:ex:s> <urn:ex:p> "\b\t\n\u000B\f\r\"\\\u007F" .
/// _:c14n0 <http://example.org/vocab#next> _:c14n2 .
/// _:c14n0 <http://example.org/vocab#prev> _:c14n1 .
/// _:c14n1 <http://example.org/vocab#next> _:c14n0 .
/// _:c14n1 <http://example.org/vocab#prev> _:c14n2 .
/// _:c14n2 <http://example.org/vocab#next> _:c14n1 .
/// _:c14n2 <http://example.org/vocab#prev> _:c14n0 .
/// "#;
///
/// let input_triples = NTriplesParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap());
/// let input_graph = Graph::from_iter(input_triples);
/// let options = CanonicalizationOptions {
///     hndq_call_limit: Some(10000),
/// };
/// let canonicalized = canonicalize_graph_with::<Sha256>(&input_graph, &options).unwrap();
///
/// assert_eq!(canonicalized, expected);
/// ```
pub fn canonicalize_graph_with<D: Digest>(
    input_graph: &Graph,
    options: &CanonicalizationOptions,
) -> Result<String, CanonicalizationError> {
    let issued_identifiers_map = issue_graph_with::<D>(input_graph, options)?;
    let relabeled_graph = relabel_graph(input_graph, &issued_identifiers_map)?;
    Ok(serialize_graph(&relabeled_graph))
}

/// Given some options (e.g., call limit),
/// returns the serialized canonical form of the canonicalized dataset,
/// where any blank nodes in the input quads are assigned deterministic identifiers.
///
/// # Examples
///
/// ```
/// use oxrdf::Quad;
/// use oxttl::NQuadsParser;
/// use rdf_canon::{canonicalize_quads_with, CanonicalizationOptions};
/// use sha2::Sha256;
/// use std::io::Cursor;

/// let input = r#"_:e0 <http://example.org/vocab#next> _:e1 _:g .
/// _:e0 <http://example.org/vocab#prev> _:e2 _:g .
/// _:e1 <http://example.org/vocab#next> _:e2 _:g .
/// _:e1 <http://example.org/vocab#prev> _:e0 _:g .
/// _:e2 <http://example.org/vocab#next> _:e0 _:g .
/// _:e2 <http://example.org/vocab#prev> _:e1 _:g .
/// <urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" _:g .
/// "#;
/// let expected = r#"<urn:ex:s> <urn:ex:p> "\b\t\n\u000B\f\r\"\\\u007F" _:c14n0 .
/// _:c14n1 <http://example.org/vocab#next> _:c14n2 _:c14n0 .
/// _:c14n1 <http://example.org/vocab#prev> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#next> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#prev> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#next> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#prev> _:c14n2 _:c14n0 .
/// "#;
///
/// let input_quads: Vec<Quad> = NQuadsParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap())
///     .collect();
/// let options = CanonicalizationOptions {
///     hndq_call_limit: Some(10000),
/// };
/// let canonicalized = canonicalize_quads_with::<Sha256>(&input_quads, &options).unwrap();
///
/// assert_eq!(canonicalized, expected);
/// ```
pub fn canonicalize_quads_with<D: Digest>(
    input_quads: &[Quad],
    options: &CanonicalizationOptions,
) -> Result<String, CanonicalizationError> {
    let input_dataset = Dataset::from_iter(input_quads);
    let issued_identifiers_map = issue_with::<D>(&input_dataset, options)?;
    let relabeled_dataset = relabel(&input_dataset, &issued_identifiers_map)?;
    Ok(serialize(&relabeled_dataset))
}

/// Assigns deterministic identifiers to any blank nodes in the input dataset
/// and returns the assignment result as a map.
///
/// # Examples
///
/// ```
/// use oxrdf::Dataset;
/// use oxttl::NQuadsParser;
/// use rdf_canon::issue;
/// use std::collections::HashMap;
/// use std::io::Cursor;
///
/// let input = r#"
/// _:e0 <http://example.org/vocab#next> _:e1 _:g .
/// _:e0 <http://example.org/vocab#prev> _:e2 _:g .
/// _:e1 <http://example.org/vocab#next> _:e2 _:g .
/// _:e1 <http://example.org/vocab#prev> _:e0 _:g .
/// _:e2 <http://example.org/vocab#next> _:e0 _:g .
/// _:e2 <http://example.org/vocab#prev> _:e1 _:g .
/// "#;
/// let expected_map = HashMap::from([
///     ("g".to_string(), "c14n0".to_string()),
///     ("e0".to_string(), "c14n1".to_string()),
///     ("e1".to_string(), "c14n2".to_string()),
///     ("e2".to_string(), "c14n3".to_string()),
/// ]);
///
/// let input_quads = NQuadsParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap());
/// let input_dataset = Dataset::from_iter(input_quads);
/// let issued_identifiers_map = issue(&input_dataset).unwrap();
///
/// assert_eq!(issued_identifiers_map, expected_map);
/// ```
pub fn issue(input_dataset: &Dataset) -> Result<HashMap<String, String>, CanonicalizationError> {
    let options = CanonicalizationOptions::default();
    issue_with::<Sha256>(input_dataset, &options)
}

/// Assigns deterministic identifiers to any blank nodes in the input graph
/// and returns the assignment result as a map.
///
/// # Examples
///
/// ```
/// use oxrdf::Graph;
/// use oxttl::NTriplesParser;
/// use rdf_canon::issue_graph;
/// use std::collections::HashMap;
/// use std::io::Cursor;
///
/// let input = r#"
/// _:e0 <http://example.org/vocab#next> _:e1 .
/// _:e0 <http://example.org/vocab#prev> _:e2 .
/// _:e1 <http://example.org/vocab#next> _:e2 .
/// _:e1 <http://example.org/vocab#prev> _:e0 .
/// _:e2 <http://example.org/vocab#next> _:e0 .
/// _:e2 <http://example.org/vocab#prev> _:e1 .
/// "#;
/// let expected_map = HashMap::from([
///     ("e0".to_string(), "c14n0".to_string()),
///     ("e1".to_string(), "c14n2".to_string()),
///     ("e2".to_string(), "c14n1".to_string()),
/// ]);
///
/// let input_triples = NTriplesParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap());
/// let input_graph = Graph::from_iter(input_triples);
/// let issued_identifiers_map = issue_graph(&input_graph).unwrap();
///
/// assert_eq!(issued_identifiers_map, expected_map);
/// ```
pub fn issue_graph(input_graph: &Graph) -> Result<HashMap<String, String>, CanonicalizationError> {
    let options = CanonicalizationOptions::default();
    issue_graph_with::<Sha256>(input_graph, &options)
}

/// Assigns deterministic identifiers to any blank nodes in the input quads
/// and returns the assignment result as a map.
///
/// # Examples
///
/// ```
/// use oxrdf::Quad;
/// use oxttl::NQuadsParser;
/// use rdf_canon::issue_quads;
/// use std::collections::HashMap;
/// use std::io::Cursor;
///
/// let input = r#"
/// _:e0 <http://example.org/vocab#next> _:e1 _:g .
/// _:e0 <http://example.org/vocab#prev> _:e2 _:g .
/// _:e1 <http://example.org/vocab#next> _:e2 _:g .
/// _:e1 <http://example.org/vocab#prev> _:e0 _:g .
/// _:e2 <http://example.org/vocab#next> _:e0 _:g .
/// _:e2 <http://example.org/vocab#prev> _:e1 _:g .
/// "#;
/// let expected_map = HashMap::from([
///     ("g".to_string(), "c14n0".to_string()),
///     ("e0".to_string(), "c14n1".to_string()),
///     ("e1".to_string(), "c14n2".to_string()),
///     ("e2".to_string(), "c14n3".to_string()),
/// ]);
///
/// let input_quads: Vec<Quad> = NQuadsParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap())
///     .collect();
/// let issued_identifiers_map = issue_quads(&input_quads).unwrap();
///
/// assert_eq!(issued_identifiers_map, expected_map);
/// ```
pub fn issue_quads(input_quads: &[Quad]) -> Result<HashMap<String, String>, CanonicalizationError> {
    let options = CanonicalizationOptions::default();
    issue_quads_with::<Sha256>(input_quads, &options)
}

/// Given some options (e.g., call limit),
/// assigns deterministic identifiers to any blank nodes in the input dataset
/// and returns the assignment result as a map.
///
/// # Examples
///
/// ```
/// use oxrdf::Dataset;
/// use oxttl::NQuadsParser;
/// use rdf_canon::{issue_with, CanonicalizationOptions};
/// use sha2::Sha256;
/// use std::collections::HashMap;
/// use std::io::Cursor;
///
/// let input = r#"
/// _:e0 <http://example.org/vocab#next> _:e1 _:g .
/// _:e0 <http://example.org/vocab#prev> _:e2 _:g .
/// _:e1 <http://example.org/vocab#next> _:e2 _:g .
/// _:e1 <http://example.org/vocab#prev> _:e0 _:g .
/// _:e2 <http://example.org/vocab#next> _:e0 _:g .
/// _:e2 <http://example.org/vocab#prev> _:e1 _:g .
/// "#;
/// let expected_map = HashMap::from([
///     ("g".to_string(), "c14n0".to_string()),
///     ("e0".to_string(), "c14n1".to_string()),
///     ("e1".to_string(), "c14n2".to_string()),
///     ("e2".to_string(), "c14n3".to_string()),
/// ]);
///
/// let input_quads = NQuadsParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap());
/// let input_dataset = Dataset::from_iter(input_quads);
/// let options = CanonicalizationOptions {
///     hndq_call_limit: Some(10000),
/// };
///
/// let issued_identifiers_map = issue_with::<Sha256>(&input_dataset, &options).unwrap();
///
/// assert_eq!(issued_identifiers_map, expected_map);
/// ```
pub fn issue_with<D: Digest>(
    input_dataset: &Dataset,
    options: &CanonicalizationOptions,
) -> Result<HashMap<String, String>, CanonicalizationError> {
    let hndq_call_counter = SimpleHndqCallCounter::new(options.hndq_call_limit);
    canonicalize_core::<D>(input_dataset, hndq_call_counter)
}

/// Given some options (e.g., call limit),
/// assigns deterministic identifiers to any blank nodes in the input graph
/// and returns the assignment result as a map.
///
/// # Examples
///
/// ```
/// use oxrdf::Graph;
/// use oxttl::NTriplesParser;
/// use rdf_canon::{issue_graph_with, CanonicalizationOptions};
/// use sha2::Sha256;
/// use std::collections::HashMap;
/// use std::io::Cursor;
///
/// let input = r#"
/// _:e0 <http://example.org/vocab#next> _:e1 .
/// _:e0 <http://example.org/vocab#prev> _:e2 .
/// _:e1 <http://example.org/vocab#next> _:e2 .
/// _:e1 <http://example.org/vocab#prev> _:e0 .
/// _:e2 <http://example.org/vocab#next> _:e0 .
/// _:e2 <http://example.org/vocab#prev> _:e1 .
/// "#;
/// let expected_map = HashMap::from([
///     ("e0".to_string(), "c14n0".to_string()),
///     ("e1".to_string(), "c14n2".to_string()),
///     ("e2".to_string(), "c14n1".to_string()),
/// ]);
///
/// let input_triples = NTriplesParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap());
/// let input_graph = Graph::from_iter(input_triples);
/// let options = CanonicalizationOptions {
///     hndq_call_limit: Some(10000),
/// };
///
/// let issued_identifiers_map = issue_graph_with::<Sha256>(&input_graph, &options).unwrap();
///
/// assert_eq!(issued_identifiers_map, expected_map);
/// ```
pub fn issue_graph_with<D: Digest>(
    input_graph: &Graph,
    options: &CanonicalizationOptions,
) -> Result<HashMap<String, String>, CanonicalizationError> {
    let hndq_call_counter = SimpleHndqCallCounter::new(options.hndq_call_limit);
    let input_dataset = Dataset::from_iter(
        input_graph
            .iter()
            .map(|t| QuadRef::new(t.subject, t.predicate, t.object, GraphNameRef::DefaultGraph)),
    );
    canonicalize_core::<D>(&input_dataset, hndq_call_counter)
}

/// Given some options (e.g., call limit),
/// assigns deterministic identifiers to any blank nodes in the input quads
/// and returns the assignment result as a map.
///
/// # Examples
///
/// ```
/// use oxrdf::Quad;
/// use oxttl::NQuadsParser;
/// use rdf_canon::{issue_quads_with, CanonicalizationOptions};
/// use sha2::Sha256;
/// use std::collections::HashMap;
/// use std::io::Cursor;
///
/// let input = r#"
/// _:e0 <http://example.org/vocab#next> _:e1 _:g .
/// _:e0 <http://example.org/vocab#prev> _:e2 _:g .
/// _:e1 <http://example.org/vocab#next> _:e2 _:g .
/// _:e1 <http://example.org/vocab#prev> _:e0 _:g .
/// _:e2 <http://example.org/vocab#next> _:e0 _:g .
/// _:e2 <http://example.org/vocab#prev> _:e1 _:g .
/// "#;
/// let expected_map = HashMap::from([
///     ("g".to_string(), "c14n0".to_string()),
///     ("e0".to_string(), "c14n1".to_string()),
///     ("e1".to_string(), "c14n2".to_string()),
///     ("e2".to_string(), "c14n3".to_string()),
/// ]);
///
/// let input_quads: Vec<Quad> = NQuadsParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap())
///     .collect();
/// let options = CanonicalizationOptions {
///     hndq_call_limit: Some(10000),
/// };
///
/// let issued_identifiers_map = issue_quads_with::<Sha256>(&input_quads, &options).unwrap();
///
/// assert_eq!(issued_identifiers_map, expected_map);
/// ```
pub fn issue_quads_with<D: Digest>(
    input_quads: &[Quad],
    options: &CanonicalizationOptions,
) -> Result<HashMap<String, String>, CanonicalizationError> {
    let input_dataset = Dataset::from_iter(input_quads);
    let hndq_call_counter = SimpleHndqCallCounter::new(options.hndq_call_limit);
    canonicalize_core::<D>(&input_dataset, hndq_call_counter)
}

/// Re-label blank node identifiers in the input dataset according to the issued identifiers map.
/// Note that the output `Dataset` does not retain the order of quads, unlike `Vec<Quad>`.
///
/// # Examples
///
/// ```
/// use oxrdf::Dataset;
/// use oxttl::NQuadsParser;
/// use rdf_canon::relabel;
/// use std::collections::HashMap;
/// use std::io::Cursor;
///
/// let input = r#"
/// _:e0 <http://example.org/vocab#next> _:e1 _:g .
/// _:e0 <http://example.org/vocab#prev> _:e2 _:g .
/// _:e1 <http://example.org/vocab#next> _:e2 _:g .
/// _:e1 <http://example.org/vocab#prev> _:e0 _:g .
/// _:e2 <http://example.org/vocab#next> _:e0 _:g .
/// _:e2 <http://example.org/vocab#prev> _:e1 _:g .
/// "#;
/// let issued_identifiers_map = HashMap::from([
///     ("g".to_string(), "c14n0".to_string()),
///     ("e0".to_string(), "c14n1".to_string()),
///     ("e1".to_string(), "c14n2".to_string()),
///     ("e2".to_string(), "c14n3".to_string()),
/// ]);
/// let expected = r#"
/// _:c14n1 <http://example.org/vocab#next> _:c14n2 _:c14n0 .
/// _:c14n1 <http://example.org/vocab#prev> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#next> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#prev> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#next> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#prev> _:c14n2 _:c14n0 .
/// "#;
///
/// let input_quads = NQuadsParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap());
/// let input_dataset = Dataset::from_iter(input_quads);
/// let labeled_dataset = relabel(&input_dataset, &issued_identifiers_map).unwrap();
/// let expected_quads = NQuadsParser::new()
///     .for_reader(Cursor::new(expected))
///     .map(|x| x.unwrap());
/// let expected_dataset = Dataset::from_iter(expected_quads);
///
/// assert_eq!(labeled_dataset, expected_dataset);
/// ```
pub fn relabel(
    input_dataset: &Dataset,
    issued_identifiers_map: &HashMap<String, String>,
) -> Result<Dataset, CanonicalizationError> {
    input_dataset
        .iter()
        .map(|q| relabel_quad(q, issued_identifiers_map))
        .collect()
}

/// Re-label blank node identifiers in the input graph according to the issued identifiers map.
/// Note that the output `Graph` does not retain the order of triples, unlike `Vec<Triple>`.
///
/// # Examples
///
/// ```
/// use oxrdf::Graph;
/// use oxttl::NTriplesParser;
/// use rdf_canon::relabel_graph;
/// use std::collections::HashMap;
/// use std::io::Cursor;
///
/// let input = r#"
/// _:e0 <http://example.org/vocab#next> _:e1 .
/// _:e0 <http://example.org/vocab#prev> _:e2 .
/// _:e1 <http://example.org/vocab#next> _:e2 .
/// _:e1 <http://example.org/vocab#prev> _:e0 .
/// _:e2 <http://example.org/vocab#next> _:e0 .
/// _:e2 <http://example.org/vocab#prev> _:e1 .
/// "#;
/// let issued_identifiers_map = HashMap::from([
///     ("e0".to_string(), "c14n0".to_string()),
///     ("e1".to_string(), "c14n2".to_string()),
///     ("e2".to_string(), "c14n1".to_string()),
/// ]);
/// let expected = r#"
/// _:c14n0 <http://example.org/vocab#next> _:c14n2 .
/// _:c14n0 <http://example.org/vocab#prev> _:c14n1 .
/// _:c14n2 <http://example.org/vocab#next> _:c14n1 .
/// _:c14n2 <http://example.org/vocab#prev> _:c14n0 .
/// _:c14n1 <http://example.org/vocab#next> _:c14n0 .
/// _:c14n1 <http://example.org/vocab#prev> _:c14n2 .
/// "#;
///
/// let input_triples = NTriplesParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap());
/// let input_graph = Graph::from_iter(input_triples);
/// let labeled_graph = relabel_graph(&input_graph, &issued_identifiers_map).unwrap();
/// let expected_triples = NTriplesParser::new()
///     .for_reader(Cursor::new(expected))
///     .map(|x| x.unwrap());
/// let expected_graph = Graph::from_iter(expected_triples);
///
/// assert_eq!(labeled_graph, expected_graph);
/// ```
pub fn relabel_graph(
    input_graph: &Graph,
    issued_identifiers_map: &HashMap<String, String>,
) -> Result<Graph, CanonicalizationError> {
    input_graph
        .iter()
        .map(|t| relabel_triple(t, issued_identifiers_map))
        .collect()
}

/// Re-label blank node identifiers in the input quads according to the issued identifiers map.
///
/// # Examples
///
/// ```
/// use oxrdf::Quad;
/// use oxttl::NQuadsParser;
/// use rdf_canon::relabel_quads;
/// use std::collections::HashMap;
/// use std::io::Cursor;
///
/// let input = r#"
/// _:e0 <http://example.org/vocab#next> _:e1 _:g .
/// _:e0 <http://example.org/vocab#prev> _:e2 _:g .
/// _:e1 <http://example.org/vocab#next> _:e2 _:g .
/// _:e1 <http://example.org/vocab#prev> _:e0 _:g .
/// _:e2 <http://example.org/vocab#next> _:e0 _:g .
/// _:e2 <http://example.org/vocab#prev> _:e1 _:g .
/// "#;
/// let issued_identifiers_map = HashMap::from([
///     ("g".to_string(), "c14n0".to_string()),
///     ("e0".to_string(), "c14n1".to_string()),
///     ("e1".to_string(), "c14n2".to_string()),
///     ("e2".to_string(), "c14n3".to_string()),
/// ]);
/// let expected = r#"
/// _:c14n1 <http://example.org/vocab#next> _:c14n2 _:c14n0 .
/// _:c14n1 <http://example.org/vocab#prev> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#next> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#prev> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#next> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#prev> _:c14n2 _:c14n0 .
/// "#;
///
/// let input_quads: Vec<Quad> = NQuadsParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap())
///     .collect();
/// let labeled_quads = relabel_quads(&input_quads, &issued_identifiers_map).unwrap();
/// let expected_quads: Vec<Quad> = NQuadsParser::new()
///     .for_reader(Cursor::new(expected))
///     .map(|x| x.unwrap())
///     .collect();
///
/// assert_eq!(labeled_quads, expected_quads);
/// ```
pub fn relabel_quads(
    input_quads: &[Quad],
    issued_identifiers_map: &HashMap<String, String>,
) -> Result<Vec<Quad>, CanonicalizationError> {
    input_quads
        .iter()
        .map(|q| relabel_quad(q.into(), issued_identifiers_map))
        .collect()
}

fn relabel_quad(
    q: QuadRef,
    issued_identifiers_map: &HashMap<String, String>,
) -> Result<Quad, CanonicalizationError> {
    Ok(Quad::new(
        relabel_subject(q.subject, issued_identifiers_map)?,
        q.predicate,
        relabel_term(q.object, issued_identifiers_map)?,
        relabel_graph_name(q.graph_name, issued_identifiers_map)?,
    ))
}

fn relabel_triple(
    t: TripleRef,
    issued_identifiers_map: &HashMap<String, String>,
) -> Result<Triple, CanonicalizationError> {
    Ok(Triple::new(
        relabel_subject(t.subject, issued_identifiers_map)?,
        t.predicate,
        relabel_term(t.object, issued_identifiers_map)?,
    ))
}

fn relabel_subject(
    s: SubjectRef,
    issued_identifiers_map: &HashMap<String, String>,
) -> Result<Subject, CanonicalizationError> {
    match s {
        SubjectRef::BlankNode(blank_node) => {
            match relabel_blank_node(blank_node, issued_identifiers_map) {
                Ok(canonicalized_blank_node) => Ok(Subject::BlankNode(canonicalized_blank_node)),
                Err(e) => Err(e),
            }
        }
        _ => Ok(s.into()),
    }
}

fn relabel_term(
    o: TermRef,
    issued_identifiers_map: &HashMap<String, String>,
) -> Result<Term, CanonicalizationError> {
    match o {
        TermRef::BlankNode(blank_node) => {
            match relabel_blank_node(blank_node, issued_identifiers_map) {
                Ok(canonicalized_blank_node) => Ok(Term::BlankNode(canonicalized_blank_node)),
                Err(e) => Err(e),
            }
        }
        _ => Ok(o.into()),
    }
}

fn relabel_graph_name(
    g: GraphNameRef,
    issued_identifiers_map: &HashMap<String, String>,
) -> Result<GraphName, CanonicalizationError> {
    match g {
        GraphNameRef::BlankNode(blank_node) => {
            match relabel_blank_node(blank_node, issued_identifiers_map) {
                Ok(canonicalized_blank_node) => Ok(GraphName::BlankNode(canonicalized_blank_node)),
                Err(e) => Err(e),
            }
        }
        _ => Ok(g.into()),
    }
}

fn relabel_blank_node(
    b: BlankNodeRef,
    issued_identifiers_map: &HashMap<String, String>,
) -> Result<BlankNode, CanonicalizationError> {
    let canonical_identifier = issued_identifiers_map.get(b.as_str());
    match canonical_identifier {
        Some(id) => Ok(BlankNode::new(id)?),
        None => Err(CanonicalizationError::CanonicalIdentifierNotExist),
    }
}

/// Sort each quad from the canonicalized dataset into code point order.
///
/// # Examples
///
/// ```
/// use oxrdf::{Dataset, Quad};
/// use oxttl::NQuadsParser;
/// use rdf_canon::{relabel, sort};
/// use std::collections::HashMap;
/// use std::io::Cursor;
///
/// let input = r#"
/// _:e0 <http://example.org/vocab#next> _:e1 _:g .
/// _:e0 <http://example.org/vocab#prev> _:e2 _:g .
/// _:e1 <http://example.org/vocab#next> _:e2 _:g .
/// _:e1 <http://example.org/vocab#prev> _:e0 _:g .
/// _:e2 <http://example.org/vocab#next> _:e0 _:g .
/// _:e2 <http://example.org/vocab#prev> _:e1 _:g .
/// "#;
/// let issued_identifiers_map = HashMap::from([
///     ("g".to_string(), "c14n0".to_string()),
///     ("e0".to_string(), "c14n1".to_string()),
///     ("e1".to_string(), "c14n2".to_string()),
///     ("e2".to_string(), "c14n3".to_string()),
/// ]);
/// let expected = r#"
/// _:c14n1 <http://example.org/vocab#next> _:c14n2 _:c14n0 .
/// _:c14n1 <http://example.org/vocab#prev> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#next> _:c14n3 _:c14n0 .
/// _:c14n2 <http://example.org/vocab#prev> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#next> _:c14n1 _:c14n0 .
/// _:c14n3 <http://example.org/vocab#prev> _:c14n2 _:c14n0 .
/// "#;
///
/// let input_quads = NQuadsParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap());
/// let input_dataset = Dataset::from_iter(input_quads);
/// let labeled_dataset = relabel(&input_dataset, &issued_identifiers_map).unwrap();
/// let canonicalized_quads = sort(&labeled_dataset);
/// let expected_quads: Vec<Quad> = NQuadsParser::new()
///     .for_reader(Cursor::new(expected))
///     .map(|x| x.unwrap())
///     .collect();
///
/// assert_eq!(canonicalized_quads, expected_quads);
/// ```
pub fn sort(dataset: &Dataset) -> Vec<Quad> {
    let mut ordered_dataset: Vec<QuadRef> = dataset.iter().collect();
    ordered_dataset.sort_by_cached_key(|q| q.to_string());
    ordered_dataset.iter().map(|q| q.into_owned()).collect()
}

/// Sort each triple from the canonicalized graph into code point order.
///
/// # Examples
///
/// ```
/// use oxrdf::{Graph, Triple};
/// use oxttl::NTriplesParser;
/// use rdf_canon::{relabel_graph, sort_graph};
/// use std::collections::HashMap;
/// use std::io::Cursor;
///
/// let input = r#"
/// _:e0 <http://example.org/vocab#next> _:e1 .
/// _:e0 <http://example.org/vocab#prev> _:e2 .
/// _:e1 <http://example.org/vocab#next> _:e2 .
/// _:e1 <http://example.org/vocab#prev> _:e0 .
/// _:e2 <http://example.org/vocab#next> _:e0 .
/// _:e2 <http://example.org/vocab#prev> _:e1 .
/// "#;
/// let issued_identifiers_map = HashMap::from([
///     ("e0".to_string(), "c14n0".to_string()),
///     ("e1".to_string(), "c14n2".to_string()),
///     ("e2".to_string(), "c14n1".to_string()),
/// ]);
/// let expected = r#"
/// _:c14n0 <http://example.org/vocab#next> _:c14n2 .
/// _:c14n0 <http://example.org/vocab#prev> _:c14n1 .
/// _:c14n1 <http://example.org/vocab#next> _:c14n0 .
/// _:c14n1 <http://example.org/vocab#prev> _:c14n2 .
/// _:c14n2 <http://example.org/vocab#next> _:c14n1 .
/// _:c14n2 <http://example.org/vocab#prev> _:c14n0 .
/// "#;
///
/// let input_triples = NTriplesParser::new()
///     .for_reader(Cursor::new(input))
///     .map(|x| x.unwrap());
/// let input_graph = Graph::from_iter(input_triples);
/// let labeled_graph = relabel_graph(&input_graph, &issued_identifiers_map).unwrap();
/// let canonicalized_triples = sort_graph(&labeled_graph);
/// let expected_triples: Vec<Triple> = NTriplesParser::new()
///     .for_reader(Cursor::new(expected))
///     .map(|x| x.unwrap())
///     .collect();
///
/// assert_eq!(canonicalized_triples, expected_triples);
/// ```
pub fn sort_graph(graph: &Graph) -> Vec<Triple> {
    let mut ordered_graph: Vec<TripleRef> = graph.iter().collect();
    ordered_graph.sort_by_cached_key(|t| t.to_string());
    ordered_graph.iter().map(|t| t.into_owned()).collect()
}
