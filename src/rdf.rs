/// RDF data interfaces based on
/// [RDF/JS: Data model specification](https://rdf.js.org/data-model-spec/)
// to generate blank node identifiers as short uuids
use nanoid::nanoid;

/// The subject, which is a NamedNode, BlankNode, Variable or Quad.
/// NOTE: We do not currently support Quad as a subject here.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Subject {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Variable(Variable),
    // Quad(Quad),
}

/// The predicate, which is a NamedNode or Variable.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Predicate {
    NamedNode(NamedNode),
    Variable(Variable),
}

/// The object, which is a NamedNode, Literal, BlankNode or Variable.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Object {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Literal(Literal),
    Variable(Variable),
}

/// The named graph, which is a DefaultGraph, NamedNode, BlankNode or
/// Variable.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Graph {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    DefaultGraph(DefaultGraph),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct NamedNode {
    /// The IRI of the named node (example: "http://example.org/resource").
    pub value: String,
}

impl NamedNode {
    /// Returns a new instance of NamedNode.
    pub fn new(value: &str) -> NamedNode {
        NamedNode {
            value: value.to_string(),
        }
    }
}

impl PartialEq<BlankNode> for NamedNode {
    fn eq(&self, _other: &BlankNode) -> bool {
        false
    }
}
impl PartialEq<Literal> for NamedNode {
    fn eq(&self, _other: &Literal) -> bool {
        false
    }
}
impl PartialEq<Variable> for NamedNode {
    fn eq(&self, _other: &Variable) -> bool {
        false
    }
}
impl PartialEq<DefaultGraph> for NamedNode {
    fn eq(&self, _other: &DefaultGraph) -> bool {
        false
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BlankNode {
    /// Blank node name as a string, without any serialization specific
    /// prefixes, e.g. when parsing, if the data was sourced from Turtle,
    /// remove "_:", if it was sourced from RDF/XML, do not change the
    /// blank node name (example: "blank3").
    pub value: String,
}

impl BlankNode {
    const BLANK_NODE_ID_LENGTH: usize = 10;

    /// Returns a new instance of BlankNode. If the value parameter is undefined
    /// a new identifier for the blank node is generated for each call.
    pub fn new(value: Option<&str>) -> BlankNode {
        let len = Self::BLANK_NODE_ID_LENGTH;
        match value {
            Some(v) => BlankNode {
                value: v.to_string(),
            },
            None => BlankNode {
                value: nanoid!(len),
            },
        }
    }
}

impl PartialEq<NamedNode> for BlankNode {
    fn eq(&self, _other: &NamedNode) -> bool {
        false
    }
}
impl PartialEq<Literal> for BlankNode {
    fn eq(&self, _other: &Literal) -> bool {
        false
    }
}
impl PartialEq<Variable> for BlankNode {
    fn eq(&self, _other: &Variable) -> bool {
        false
    }
}
impl PartialEq<DefaultGraph> for BlankNode {
    fn eq(&self, _other: &DefaultGraph) -> bool {
        false
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Literal {
    /// The text value, unescaped, without language or type (example: "Brad
    /// Pitt").
    pub value: String,
    /// The language as lowercase BCP-47 [BCP47] string (examples: "en",
    /// "en-gb") or an empty string if the literal has no language.
    pub language: Option<String>,
    /// A NamedNode whose IRI represents the datatype of the literal.
    pub datatype: Option<NamedNode>,
}

impl Literal {
    /// Returns a new instance of Literal. If languageOrDatatype is a NamedNode, then it is used
    /// for the value of datatype. Otherwise languageOrDatatype is used for the value of language.
    /// NOTE: languageOrDatatype is split into datatype and language
    pub fn new(value: &str, datatype: Option<&NamedNode>, language: Option<&str>) -> Literal {
        match datatype {
            Some(datatype) => Literal {
                value: value.to_string(),
                datatype: Some(datatype.clone()),
                language: None,
            },
            None => match language {
                Some(language) => Literal {
                    value: value.to_string(),
                    language: Some(language.to_string()),
                    datatype: Some(NamedNode::new(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#langString",
                    )),
                },
                None => Literal {
                    value: value.to_string(),
                    language: None,
                    datatype: Some(NamedNode::new("http://www.w3.org/2001/XMLSchema#string")),
                },
            },
        }
    }
}

impl PartialEq<NamedNode> for Literal {
    fn eq(&self, _other: &NamedNode) -> bool {
        false
    }
}
impl PartialEq<BlankNode> for Literal {
    fn eq(&self, _other: &BlankNode) -> bool {
        false
    }
}
impl PartialEq<Variable> for Literal {
    fn eq(&self, _other: &Variable) -> bool {
        false
    }
}
impl PartialEq<DefaultGraph> for Literal {
    fn eq(&self, _other: &DefaultGraph) -> bool {
        false
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Variable {
    /// The name of the variable without leading "?" (example: "a").
    pub value: String,
}

impl Variable {
    /// Returns a new instance of Variable. This method is optional.
    pub fn new(value: &str) -> Variable {
        Variable {
            value: value.to_string(),
        }
    }
}

impl PartialEq<NamedNode> for Variable {
    fn eq(&self, _other: &NamedNode) -> bool {
        false
    }
}
impl PartialEq<BlankNode> for Variable {
    fn eq(&self, _other: &BlankNode) -> bool {
        false
    }
}
impl PartialEq<Literal> for Variable {
    fn eq(&self, _other: &Literal) -> bool {
        false
    }
}
impl PartialEq<DefaultGraph> for Variable {
    fn eq(&self, _other: &DefaultGraph) -> bool {
        false
    }
}

/// An instance of DefaultGraph represents the default graph. It's only
/// allowed to assign a DefaultGraph to the graph property of a Quad.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DefaultGraph {
    // Contains an empty string as constant value.
    // NOTE: We omit the empty string `value` here.
    // _value: String,
}

impl DefaultGraph {
    pub fn new() -> DefaultGraph {
        DefaultGraph {}
    }
}

impl PartialEq<NamedNode> for DefaultGraph {
    fn eq(&self, _other: &NamedNode) -> bool {
        false
    }
}
impl PartialEq<BlankNode> for DefaultGraph {
    fn eq(&self, _other: &BlankNode) -> bool {
        false
    }
}
impl PartialEq<Literal> for DefaultGraph {
    fn eq(&self, _other: &Literal) -> bool {
        false
    }
}
impl PartialEq<Variable> for DefaultGraph {
    fn eq(&self, _other: &Variable) -> bool {
        false
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Quad {
    // Contains an empty string as constant value.
    // NOTE: We omit the empty string `value` here.
    // value: String,
    /// The subject, which is a NamedNode, BlankNode, Variable or Quad.
    /// NOTE: We do not currently support Quad as a subject here
    pub subject: Subject,
    /// The predicate, which is a NamedNode or Variable.
    pub predicate: Predicate,
    /// The object, which is a NamedNode, Literal, BlankNode or Variable.
    pub object: Object,
    /// The named graph, which is a DefaultGraph, NamedNode, BlankNode or
    /// Variable.
    pub graph: Graph,
}

impl Quad {
    /// Returns a new instance of Quad. If graph is undefined or null it MUST set graph
    /// to a DefaultGraph.
    pub fn new(subject: &Subject, predicate: &Predicate, object: &Object, graph: &Graph) -> Quad {
        Quad {
            subject: subject.clone(),
            predicate: predicate.clone(),
            object: object.clone(),
            graph: graph.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_named_node() {
        let named_node1 = NamedNode::new("http://example.org/foo");
        let named_node2 = NamedNode::new("urn:example:bar");
        let named_node3 = NamedNode::new("http://example.org/foo");
        assert_eq!(named_node1.value, "http://example.org/foo");
        assert_eq!(named_node2.value, "urn:example:bar");
        assert_eq!(named_node3.value, "http://example.org/foo");
        assert_ne!(named_node1, named_node2);
        assert_ne!(named_node2, named_node3);
        assert_eq!(named_node3, named_node1);
    }

    #[test]
    fn gen_blank_node() {
        let blank_node1 = BlankNode::new(None);
        let blank_node2 = BlankNode::new(None);
        let blank_node3 = BlankNode::new(Some(&blank_node1.value));
        assert_ne!(blank_node1, blank_node2);
        assert_ne!(blank_node2, blank_node3);
        assert_eq!(blank_node3, blank_node1);
    }

    #[test]
    fn gen_literal() {
        let literal1 = Literal::new("foo", None, None);
        let literal2 = Literal::new("bar", None, None);
        let literal3 = Literal::new("foo", None, None);
        let literal4_en = Literal::new("foo", None, Some("en"));
        let literal4_ja = Literal::new("あいうえお", None, Some("ja"));
        let literal5 = Literal::new(
            "123",
            Some(&NamedNode::new("http://www.w3.org/2001/XMLSchema#integer")),
            None,
        );
        let literal6 = Literal::new(
            "123",
            Some(&NamedNode::new("http://www.w3.org/2001/XMLSchema#integer")),
            None,
        );
        assert_eq!(literal1.value, "foo");
        assert_eq!(literal2.value, "bar");
        assert_eq!(literal3.value, "foo");
        assert_eq!(literal4_en.value, "foo");
        assert_eq!(literal4_ja.value, "あいうえお");
        assert_eq!(literal5.value, "123");
        assert_ne!(literal1, literal2);
        assert_ne!(literal2, literal3);
        assert_eq!(literal3, literal1);
        assert_ne!(literal4_en, literal4_ja);
        assert_ne!(literal4_en, literal4_ja);
        assert_eq!(literal5, literal6);
    }

    #[test]
    fn gen_variable() {
        let variable1 = Variable::new("foo");
        let variable2 = Variable::new("bar");
        let variable3 = Variable::new("foo");
        assert_eq!(variable1.value, "foo");
        assert_eq!(variable2.value, "bar");
        assert_eq!(variable3.value, "foo");
        assert_ne!(variable1, variable2);
        assert_ne!(variable2, variable3);
        assert_eq!(variable3, variable1);
    }

    #[test]
    fn gen_default_graph() {
        let default_graph1 = DefaultGraph::new();
        let default_graph2 = DefaultGraph::new();
        assert_eq!(default_graph1, default_graph2);
    }

    #[test]
    fn gen_quad() {
        let subject1 = NamedNode::new("http://example.org/subject1");
        let predicate1 = NamedNode::new("http://example.org/predicate1");
        let predicate3 = NamedNode::new("http://example.org/predicate3");
        let object1 = NamedNode::new("http://example.org/object1");
        let graph1 = NamedNode::new("http://example.org/graph1");
        let bnode1 = BlankNode::new(None);
        let bnode2 = BlankNode::new(None);

        let quad1 = Quad::new(
            &Subject::NamedNode(subject1.clone()),
            &Predicate::NamedNode(predicate1.clone()),
            &Object::NamedNode(object1.clone()),
            &Graph::NamedNode(graph1.clone()),
        );
        let quad2 = Quad::new(
            &Subject::BlankNode(bnode1.clone()),
            &Predicate::NamedNode(predicate1.clone()),
            &Object::BlankNode(bnode2.clone()),
            &Graph::NamedNode(graph1.clone()),
        );
        assert_ne!(quad1, quad2);
        assert_ne!(quad2, quad1);

        let quad21 = Quad::new(
            &Subject::BlankNode(bnode1.clone()),
            &Predicate::NamedNode(predicate1.clone()),
            &Object::BlankNode(bnode2.clone()),
            &Graph::NamedNode(graph1.clone()),
        );
        assert_eq!(quad2, quad21);
        assert_eq!(quad21, quad2);

        let quad22 = Quad::new(
            &Subject::BlankNode(bnode2.clone()),
            &Predicate::NamedNode(predicate1.clone()),
            &Object::BlankNode(bnode2.clone()),
            &Graph::NamedNode(graph1.clone()),
        );
        assert_ne!(quad2, quad22);
        assert_ne!(quad22, quad2);

        let quad3 = Quad::new(
            &Subject::NamedNode(subject1.clone()),
            &Predicate::NamedNode(predicate3.clone()),
            &Object::NamedNode(NamedNode::new("http://example.org/object3")),
            &Graph::NamedNode(NamedNode::new("http://example.org/graph3")),
        );
        assert_ne!(quad2, quad3);
        assert_ne!(quad3, quad2);

        let quad4 = Quad::new(
            &Subject::NamedNode(subject1.clone()),
            &Predicate::NamedNode(predicate3.clone()),
            &Object::NamedNode(NamedNode::new("http://example.org/object3")),
            &Graph::DefaultGraph(DefaultGraph::new()),
        );
        assert_ne!(quad3, quad4);
        assert_ne!(quad4, quad3);

        let quad5 = Quad::new(
            &Subject::NamedNode(NamedNode::new("http://example.org/subject1")),
            &Predicate::NamedNode(NamedNode::new("http://example.org/predicate1")),
            &Object::NamedNode(NamedNode::new("http://example.org/object1")),
            &Graph::NamedNode(NamedNode::new("http://example.org/graph1")),
        );
        assert_eq!(quad1, quad5);
        assert_eq!(quad5, quad1);
        assert_ne!(quad4, quad5);
        assert_ne!(quad5, quad4);

        let quad6 = Quad::new(
            &Subject::NamedNode(subject1.clone()),
            &Predicate::NamedNode(predicate1.clone()),
            &Object::Literal(Literal::new("object6", None, None)),
            &Graph::DefaultGraph(DefaultGraph::new()),
        );
        assert_ne!(quad1, quad6);
        assert_ne!(quad6, quad1);

        let quad7 = Quad::new(
            &Subject::NamedNode(subject1.clone()),
            &Predicate::NamedNode(predicate1.clone()),
            &Object::Literal(Literal::new(
                "object6",
                Some(&NamedNode::new("http://www.w3.org/2001/XMLSchema#string")),
                None,
            )),
            &Graph::DefaultGraph(DefaultGraph::new()),
        );
        assert_eq!(quad6, quad7);
        assert_eq!(quad7, quad6);

        assert_ne!(quad1.subject, quad2.subject);
        assert_eq!(quad1.subject, quad3.subject);
    }
}
