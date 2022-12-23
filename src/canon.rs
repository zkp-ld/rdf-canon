use crate::nquads::serialize;
use crate::rdf::{BlankNode, Graph, Object, Quad, Subject};
use std::collections::{BTreeMap, HashMap};

pub type BnodeID = String;
pub type HexHash = String;

/// **4.3 Canonicalization State**
pub struct CanonicalizationState {
    /// **blank node to quads map**
    ///   A map that relates a blank node identifier to the quads
    ///   in which they appear in the input dataset.
    blank_node_to_quads_map: HashMap<BnodeID, Vec<Quad>>,

    /// **hash to blank nodes map**
    ///   A map that relates a hash to a list of blank node identifiers.
    hash_to_blank_node_map: HashMap<String, Vec<BnodeID>>,

    /// **canonical issuer**
    ///   An identifier issuer, initialized with the prefix c14n, for
    ///   issuing canonical blank node identifiers.
    canonical_issuer: IdentifierIssuer,
}

const DEFAULT_CANONICAL_IDENTIFER_PREFIX: &str = "c14n";

impl CanonicalizationState {
    pub fn new() -> CanonicalizationState {
        CanonicalizationState {
            blank_node_to_quads_map: HashMap::new(),
            hash_to_blank_node_map: HashMap::new(),
            canonical_issuer: IdentifierIssuer::new(DEFAULT_CANONICAL_IDENTIFER_PREFIX),
        }
    }
}

/// **4.4 Blank Node Identifier Issuer State**
pub struct IdentifierIssuer {
    /// **identifier prefix**
    ///   The identifier prefix is a string that is used at the
    ///   beginning of an blank node identifier. It should be initialized
    ///   to a string that is specified by the canonicalization algorithm.
    ///   When generating a new blank node identifier, the prefix is
    ///   concatenated with a identifier counter. For example, c14n is a
    ///   proper initial value for the identifier prefix that would produce
    ///   blank node identifiers like c14n1.
    identifier_prefix: String,

    /// **identifier counter**
    ///   A counter that is appended to the identifier prefix to create an
    ///   blank node identifier. It is initialized to 0.
    identifier_counter: usize,

    /// **issued identifiers map**
    ///   An ordered map that relates existing identifiers to issued
    ///   identifiers, to prevent issuance of more than one new identifier
    ///   per existing identifier, and to allow blank nodes to be
    ///   reassigned identifiers some time after issuance.
    issued_identifiers_map: BTreeMap<BnodeID, BnodeID>,
}

impl IdentifierIssuer {
    pub fn new(identifier_prefix: &str) -> IdentifierIssuer {
        let issued_identifiers_map: BTreeMap<BnodeID, BnodeID> = BTreeMap::new();
        IdentifierIssuer {
            identifier_prefix: identifier_prefix.to_string(),
            identifier_counter: 0,
            issued_identifiers_map,
        }
    }

    pub fn increment(&mut self) {
        self.identifier_counter += 1
    }
}

/// **4.6 Issue Identifier Algorithm**
///   This algorithm issues a new blank node identifier for a given existing
///   blank node identifier. It also updates state information that tracks
///   the order in which new blank node identifiers were issued. The order
///   of issuance is important for canonically labeling blank nodes that are
///   isomorphic to others in the dataset.
pub fn issue_identifier(
    identifier_issuer: &mut IdentifierIssuer,
    existing_identifier: BnodeID,
) -> String {
    // 1) If there is a map entry for existing identifier in issued identifiers
    // map of I, return it.
    if let Some(issued_identifier) = identifier_issuer
        .issued_identifiers_map
        .get(&existing_identifier)
    {
        return issued_identifier.clone();
    }

    // 2) Generate issued identifier by concatenating identifier prefix with
    // the string value of identifier counter.
    let issued_identifier = format!(
        "{}{}",
        identifier_issuer.identifier_prefix, identifier_issuer.identifier_counter
    );

    // 3) Add an entry mapping existing identifier to issued identifier to
    // the issued identifiers map of I.
    identifier_issuer
        .issued_identifiers_map
        .insert(existing_identifier, issued_identifier.clone());

    // 4) Increment identifier counter.
    identifier_issuer.increment();

    // 5) Return issued identifier.
    issued_identifier
}

/// **4.7 Hash First Degree Quads**
///   This algorithm calculates a hash for a given blank node across the
///   quads in a dataset in which that blank node is a component. If the
///   hash uniquely identifies that blank node, no further examination is
///   necessary. Otherwise, a hash will be created for the blank node using
///   the algorithm in 4.9 Hash N-Degree Quads invoked via
///   4.5 Canonicalization Algorithm.
pub fn hash_first_degree_quads(
    canonicalization_state: CanonicalizationState,
    reference_blank_node_identifier: BnodeID,
) -> Option<HexHash> {
    // 1) Initialize nquads to an empty list. It will be used to store
    // quads in canonical n-quads form.
    // let nquads: Vec<String> = Vec::new();

    // 2) Get the list of quads quads from the map entry for reference
    // blank node identifier in the blank node to quads map.
    let quads = canonicalization_state
        .blank_node_to_quads_map
        .get(&reference_blank_node_identifier)?;

    // 3) For each quad quad in quads:
    let mut nquads = quads
        .iter()
        .map(|quad| {
            // 3.1) Serialize the quad in canonical n-quads form with the following special rule:
            // 3.1.1) If any component in quad is an blank node, then serialize it using a special
            // identifier as follows:
            let subject = match &quad.subject {
                Subject::BlankNode(n) => {
                    // 3.1.1.1) If the blank node's existing blank node identifier matches the reference
                    // blank node identifier then use the blank node identifier a, otherwise, use the blank
                    // node identifier z.
                    Subject::BlankNode(if n.value == reference_blank_node_identifier {
                        BlankNode {
                            value: "a".to_string(),
                        }
                    } else {
                        BlankNode {
                            value: "z".to_string(),
                        }
                    })
                }
                s => s.clone(),
            };
            // 3.1.1) If any component in quad is an blank node, then serialize it using a special
            // identifier as follows:
            let object = match &quad.object {
                Object::BlankNode(n) => {
                    // 3.1.1.1) If the blank node's existing blank node identifier matches the reference
                    // blank node identifier then use the blank node identifier a, otherwise, use the blank
                    // node identifier z.
                    Object::BlankNode(if n.value == reference_blank_node_identifier {
                        BlankNode {
                            value: "a".to_string(),
                        }
                    } else {
                        BlankNode {
                            value: "z".to_string(),
                        }
                    })
                }
                s => s.clone(),
            };
            // 3.1.1) If any component in quad is an blank node, then serialize it using a special
            // identifier as follows:
            let graph = match &quad.graph {
                Graph::BlankNode(n) => {
                    // 3.1.1.1) If the blank node's existing blank node identifier matches the reference
                    // blank node identifier then use the blank node identifier a, otherwise, use the blank
                    // node identifier z.
                    Graph::BlankNode(if n.value == reference_blank_node_identifier {
                        BlankNode {
                            value: "a".to_string(),
                        }
                    } else {
                        BlankNode {
                            value: "z".to_string(),
                        }
                    })
                }
                s => s.clone(),
            };
            let predicate = quad.predicate.clone();

            serialize(Quad {
                subject,
                predicate,
                object,
                graph,
            })
        })
        .collect::<Option<Vec<String>>>()?;

    // 4) Sort nquads in Unicode code point order.
    // TODO: check if it is actually Unicode code point order
    nquads.sort();

    // Dummy
    Some(nquads.join("\n"))
}

#[test]
fn test_issue_identifier() {
    let mut canonical_issuer = IdentifierIssuer::new("c14n");
    assert_eq!(
        issue_identifier(&mut canonical_issuer, "b0".to_string()),
        "c14n0".to_string()
    );
    assert_eq!(
        issue_identifier(&mut canonical_issuer, "b1".to_string()),
        "c14n1".to_string()
    );
    assert_eq!(
        issue_identifier(&mut canonical_issuer, "b99".to_string()),
        "c14n2".to_string()
    );
    assert_eq!(
        issue_identifier(&mut canonical_issuer, "xyz".to_string()),
        "c14n3".to_string()
    );
    assert_eq!(
        issue_identifier(&mut canonical_issuer, "xyz".to_string()),
        "c14n3".to_string()
    );
    assert_eq!(
        issue_identifier(&mut canonical_issuer, "b99".to_string()),
        "c14n2".to_string()
    );
    assert_eq!(
        issue_identifier(&mut canonical_issuer, "b1".to_string()),
        "c14n1".to_string()
    );
    assert_eq!(
        issue_identifier(&mut canonical_issuer, "b0".to_string()),
        "c14n0".to_string()
    );
}

#[test]
fn test_hash_first_degree_quads() {
    let state = CanonicalizationState::new();
    // TODO
}
