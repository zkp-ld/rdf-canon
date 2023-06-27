use crate::{
    canon::{canonicalize_core, serialize},
    counter::{HndqCallCounter, SimpleHndqCallCounter},
    CanonicalizationError,
};
use oxrdf::{
    BlankNode, BlankNodeRef, Dataset, GraphName, GraphNameRef, Quad, QuadRef, Subject, SubjectRef,
    Term, TermRef,
};
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

/// let input = r#"<urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" .
/// _:e0 <http://example.org/vocab#next> _:e1 .
/// _:e0 <http://example.org/vocab#prev> _:e2 .
/// _:e1 <http://example.org/vocab#next> _:e2 .
/// _:e1 <http://example.org/vocab#prev> _:e0 .
/// _:e2 <http://example.org/vocab#next> _:e0 .
/// _:e2 <http://example.org/vocab#prev> _:e1 .
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
/// let quads = NQuadsParser::new()
///     .parse_from_read(Cursor::new(input))
///     .into_iter()
///     .map(|x| x.unwrap());
/// let input_dataset = Dataset::from_iter(quads);
/// let canonicalized = canonicalize(&input_dataset).unwrap();
///
/// assert_eq!(canonicalized, expected);
/// ```
pub fn canonicalize(input_dataset: &Dataset) -> Result<String, CanonicalizationError> {
    let options = CanonicalizationOptions::default();
    canonicalize_with_options(input_dataset, &options)
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
/// use rdf_canon::{canonicalize_with_options, CanonicalizationOptions};
/// use std::io::Cursor;

/// let input = r#"<urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" .
/// _:e0 <http://example.org/vocab#next> _:e1 .
/// _:e0 <http://example.org/vocab#prev> _:e2 .
/// _:e1 <http://example.org/vocab#next> _:e2 .
/// _:e1 <http://example.org/vocab#prev> _:e0 .
/// _:e2 <http://example.org/vocab#next> _:e0 .
/// _:e2 <http://example.org/vocab#prev> _:e1 .
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
/// let quads = NQuadsParser::new()
///     .parse_from_read(Cursor::new(input))
///     .into_iter()
///     .map(|x| x.unwrap());
/// let input_dataset = Dataset::from_iter(quads);
/// let options = CanonicalizationOptions {
///     hndq_call_limit: Some(10000),
/// };
/// let canonicalized = canonicalize_with_options(&input_dataset, &options).unwrap();
///
/// assert_eq!(canonicalized, expected);
/// ```
pub fn canonicalize_with_options(
    input_dataset: &Dataset,
    options: &CanonicalizationOptions,
) -> Result<String, CanonicalizationError> {
    let issued_identifiers_map = issue_with_options(input_dataset, options)?;
    let relabeled_dataset = relabel(input_dataset, &issued_identifiers_map)?;
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
/// _:e0 <http://example.org/vocab#next> _:e1 .
/// _:e0 <http://example.org/vocab#prev> _:e2 .
/// _:e1 <http://example.org/vocab#next> _:e2 .
/// _:e1 <http://example.org/vocab#prev> _:e0 .
/// _:e2 <http://example.org/vocab#next> _:e0 .
/// _:e2 <http://example.org/vocab#prev> _:e1 .
/// "#;
/// let expected = HashMap::from([
///     ("e0".to_string(), "c14n0".to_string()),
///     ("e1".to_string(), "c14n2".to_string()),
///     ("e2".to_string(), "c14n1".to_string()),
/// ]);
///
/// let quads = NQuadsParser::new()
///     .parse_from_read(Cursor::new(input))
///     .into_iter()
///     .map(|x| x.unwrap());
/// let input_dataset = Dataset::from_iter(quads);
/// let issued_identifiers_map = issue(&input_dataset).unwrap();
///
/// assert_eq!(issued_identifiers_map, expected);
/// ```
pub fn issue(input_dataset: &Dataset) -> Result<HashMap<String, String>, CanonicalizationError> {
    let options = CanonicalizationOptions::default();
    issue_with_options(input_dataset, &options)
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
/// use rdf_canon::{issue_with_options, CanonicalizationOptions};
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
/// let expected = HashMap::from([
///     ("e0".to_string(), "c14n0".to_string()),
///     ("e1".to_string(), "c14n2".to_string()),
///     ("e2".to_string(), "c14n1".to_string()),
/// ]);
///
/// let quads = NQuadsParser::new()
///     .parse_from_read(Cursor::new(input))
///     .into_iter()
///     .map(|x| x.unwrap());
/// let input_dataset = Dataset::from_iter(quads);
/// let options = CanonicalizationOptions {
///     hndq_call_limit: Some(10000),
/// };
///
/// let issued_identifiers_map = issue_with_options(&input_dataset, &options).unwrap();
///
/// assert_eq!(issued_identifiers_map, expected);
/// ```
pub fn issue_with_options(
    input_dataset: &Dataset,
    options: &CanonicalizationOptions,
) -> Result<HashMap<String, String>, CanonicalizationError> {
    let hndq_call_counter = SimpleHndqCallCounter::new(options.hndq_call_limit);
    canonicalize_core(input_dataset, hndq_call_counter)
}

/// Re-label blank node identifiers in the input dataset according to the issued identifiers map.
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
/// let input_doc = r#"
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
/// let expected_doc = r#"
/// _:c14n0 <http://example.org/vocab#next> _:c14n2 .
/// _:c14n0 <http://example.org/vocab#prev> _:c14n1 .
/// _:c14n2 <http://example.org/vocab#next> _:c14n1 .
/// _:c14n2 <http://example.org/vocab#prev> _:c14n0 .
/// _:c14n1 <http://example.org/vocab#next> _:c14n0 .
/// _:c14n1 <http://example.org/vocab#prev> _:c14n2 .
/// "#;
///
/// let quads = NQuadsParser::new()
///     .parse_from_read(Cursor::new(input_doc))
///     .into_iter()
///     .map(|x| x.unwrap());
/// let input_dataset = Dataset::from_iter(quads);
/// let labeled_dataset = relabel(&input_dataset, &issued_identifiers_map).unwrap();
/// let quads = NQuadsParser::new()
///     .parse_from_read(Cursor::new(expected_doc))
///     .into_iter()
///     .map(|x| x.unwrap());
/// let expected = Dataset::from_iter(quads);
///
/// assert_eq!(labeled_dataset, expected);
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
