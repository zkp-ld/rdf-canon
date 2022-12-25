/// RDF data interfaces based on
/// [RDF/JS: Data model specification](https://rdf.js.org/data-model-spec/)

// /// An abstract interface
// pub trait Term {
//     /// Returns true when called with parameter other on an object term
//     /// if all of the conditions below hold:
//     /// - other is neither null nor undefined;
//     /// - term.termType is the same string as other.termType;
//     /// - other follows the additional constraints of the specific Term
//     ///   interface implemented by term (e.g., NamedNode, Literal, …);
//     /// otherwise, it returns false.
//     fn equals(&self, other: &Self) -> bool;
// }

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct NamedNode {
    /// The IRI of the named node (example: "http://example.org/resource").
    pub value: String,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BlankNode {
    /// Blank node name as a string, without any serialization specific
    /// prefixes, e.g. when parsing, if the data was sourced from Turtle,
    /// remove "_:", if it was sourced from RDF/XML, do not change the
    /// blank node name (example: "blank3").
    pub value: String,
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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Variable {
    /// The name of the variable without leading "?" (example: "a").
    pub value: String,
}

/// An instance of DefaultGraph represents the default graph. It's only
/// allowed to assign a DefaultGraph to the graph property of a Quad.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DefaultGraph {
    // Contains an empty string as constant value.
    // NOTE: We omit the empty string `value` here.
    // _value: String,
}

/// The subject, which is a NamedNode, BlankNode, Variable or Quad.
/// NOTE: We do not currently support Quad as a subject here.
#[derive(Eq, Clone, Debug)]
pub enum Subject {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Variable(Variable),
    // Quad(Quad),
}

impl PartialEq for Subject {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NamedNode(left), Self::NamedNode(right)) => left == right,
            (Self::BlankNode(left), Self::BlankNode(right)) => left == right,
            (Self::Variable(left), Self::Variable(right)) => left == right,
            _ => false,
        }
    }
}

/// The predicate, which is a NamedNode or Variable.
#[derive(Eq, Clone, Debug)]
pub enum Predicate {
    NamedNode(NamedNode),
    Variable(Variable),
}

impl PartialEq for Predicate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NamedNode(left), Self::NamedNode(right)) => left == right,
            (Self::Variable(left), Self::Variable(right)) => left == right,
            _ => false,
        }
    }
}

/// The object, which is a NamedNode, Literal, BlankNode or Variable.
#[derive(Eq, Clone, Debug)]
pub enum Object {
    NamedNode(NamedNode),
    Literal(Literal),
    BlankNode(BlankNode),
    Variable(Variable),
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NamedNode(left), Self::NamedNode(right)) => left == right,
            (Self::Literal(left), Self::Literal(right)) => left == right,
            (Self::BlankNode(left), Self::BlankNode(right)) => left == right,
            (Self::Variable(left), Self::Variable(right)) => left == right,
            _ => false,
        }
    }
}

/// The named graph, which is a DefaultGraph, NamedNode, BlankNode or
/// Variable.
#[derive(Eq, Clone, Debug)]
pub enum Graph {
    DefaultGraph(DefaultGraph),
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Variable(Variable),
}

impl PartialEq for Graph {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::DefaultGraph(left), Self::DefaultGraph(right)) => left == right,
            (Self::NamedNode(left), Self::NamedNode(right)) => left == right,
            (Self::BlankNode(left), Self::BlankNode(right)) => left == right,
            (Self::Variable(left), Self::Variable(right)) => left == right,
            _ => false,
        }
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

#[derive(Debug)]
pub struct DataFactory {
    blank_node_prefix: String,
    blank_node_counter: usize,
}

impl DataFactory {
    const DEFAULT_BLANK_NODE_PREFIX: &str = "b";
    const DEFAULT_BLANK_NODE_COUNTER: usize = 0;

    pub fn new() -> DataFactory {
        DataFactory {
            blank_node_prefix: Self::DEFAULT_BLANK_NODE_PREFIX.to_string(),
            blank_node_counter: Self::DEFAULT_BLANK_NODE_COUNTER,
        }
    }

    /// Returns a new instance of NamedNode.
    pub fn named_node(&self, value: &str) -> NamedNode {
        NamedNode {
            value: value.to_string(),
        }
    }

    /// Returns a new instance of BlankNode. If the value parameter is undefined
    /// a new identifier for the blank node is generated for each call.
    pub fn blank_node(&mut self, value: Option<&str>) -> BlankNode {
        match value {
            Some(v) => BlankNode {
                value: v.to_string(),
            },
            None => {
                let blank_node_identifier =
                    format!("{}{}", self.blank_node_prefix, self.blank_node_counter);
                self.blank_node_counter += 1;
                BlankNode {
                    value: blank_node_identifier,
                }
            }
        }
    }

    /// Returns a new instance of Literal. If languageOrDatatype is a NamedNode, then it is used
    /// for the value of datatype. Otherwise languageOrDatatype is used for the value of language.
    /// NOTE: languageOrDatatype is split into datatype and language
    pub fn literal(
        &self,
        value: &str,
        datatype: Option<&NamedNode>,
        language: Option<&str>,
    ) -> Literal {
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
                    datatype: Some(
                        self.named_node("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString"),
                    ),
                },
                None => Literal {
                    value: value.to_string(),
                    language: None,
                    datatype: Some(self.named_node("http://www.w3.org/2001/XMLSchema#string")),
                },
            },
        }
    }

    /// Returns a new instance of Variable. This method is optional.
    pub fn variable(&self, value: &str) -> Variable {
        Variable {
            value: value.to_string(),
        }
    }

    /// Returns an instance of DefaultGraph.
    pub fn default_graph(&self) -> DefaultGraph {
        DefaultGraph {}
    }

    /// Returns a new instance of Quad. If graph is undefined or null it MUST set graph
    /// to a DefaultGraph.
    pub fn quad(
        &self,
        subject: &Subject,
        predicate: &Predicate,
        object: &Object,
        graph: Option<&Graph>,
    ) -> Quad {
        let graph = match graph {
            Some(g) => g.clone(),
            None => Graph::DefaultGraph(self.default_graph()),
        };
        Quad {
            subject: subject.clone(),
            predicate: predicate.clone(),
            object: object.clone(),
            graph,
        }
    }
}

#[test]
fn gen_named_node() {
    let df = DataFactory::new();
    let named_node1 = df.named_node("http://example.org/foo");
    let named_node2 = df.named_node("urn:example:bar");
    let named_node3 = df.named_node("http://example.org/foo");
    assert_eq!(named_node1.value, "http://example.org/foo");
    assert_eq!(named_node2.value, "urn:example:bar");
    assert_eq!(named_node3.value, "http://example.org/foo");
    assert_ne!(named_node1, named_node2);
    assert_ne!(named_node2, named_node3);
    assert_eq!(named_node3, named_node1);
}

#[test]
fn gen_blank_node() {
    let mut df = DataFactory::new();
    let blank_node1 = df.blank_node(None);
    let blank_node2 = df.blank_node(None);
    let blank_node3 = df.blank_node(Some("foo"));
    let blank_node4 = df.blank_node(Some("bar"));
    let blank_node5 = df.blank_node(Some("foo"));
    assert_eq!(blank_node1.value, "b0");
    assert_eq!(blank_node2.value, "b1");
    assert_eq!(blank_node3.value, "foo");
    assert_eq!(blank_node4.value, "bar");
    assert_eq!(blank_node5.value, "foo");
    assert_ne!(blank_node1, blank_node2);
    assert_ne!(blank_node2, blank_node3);
    assert_ne!(blank_node3, blank_node1);
    assert_ne!(blank_node3, blank_node4);
    assert_ne!(blank_node4, blank_node5);
    assert_eq!(blank_node3, blank_node5);
}

#[test]
fn gen_literal() {
    let df = DataFactory::new();
    let literal1 = df.literal("foo", None, None);
    let literal2 = df.literal("bar", None, None);
    let literal3 = df.literal("foo", None, None);
    let literal4_en = df.literal("foo", None, Some("en"));
    let literal4_ja = df.literal("あいうえお", None, Some("ja"));
    let literal5 = df.literal(
        "123",
        Some(&df.named_node("http://www.w3.org/2001/XMLSchema#integer")),
        None,
    );
    let literal6 = df.literal(
        "123",
        Some(&df.named_node("http://www.w3.org/2001/XMLSchema#integer")),
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
    let df = DataFactory::new();
    let variable1 = df.variable("foo");
    let variable2 = df.variable("bar");
    let variable3 = df.variable("foo");
    assert_eq!(variable1.value, "foo");
    assert_eq!(variable2.value, "bar");
    assert_eq!(variable3.value, "foo");
    assert_ne!(variable1, variable2);
    assert_ne!(variable2, variable3);
    assert_eq!(variable3, variable1);
}

#[test]
fn gen_default_graph() {
    let df = DataFactory::new();
    let default_graph1 = df.default_graph();
    let default_graph2 = df.default_graph();
    assert_eq!(default_graph1, default_graph2);
}

#[test]
fn gen_quad() {
    let df = DataFactory::new();

    let subject1: Subject = Subject::NamedNode(df.named_node("http://example.org/subject1"));
    let predicate1: Predicate =
        Predicate::NamedNode(df.named_node("http://example.org/predicate1"));
    let object1: Object = Object::NamedNode(df.named_node("http://example.org/object1"));
    let graph1: Graph = Graph::NamedNode(df.named_node("http://example.org/graph1"));
    let quad1: Quad = df.quad(&subject1, &predicate1, &object1, Some(&graph1));

    let subject2: Subject = Subject::NamedNode(df.named_node("http://example.org/subject2"));
    let predicate2: Predicate =
        Predicate::NamedNode(df.named_node("http://example.org/predicate2"));
    let object2: Object = Object::NamedNode(df.named_node("http://example.org/object2"));
    let graph2: Graph = Graph::NamedNode(df.named_node("http://example.org/graph2"));
    let quad2: Quad = df.quad(&subject2, &predicate2, &object2, Some(&graph2));

    let predicate3: Predicate =
        Predicate::NamedNode(df.named_node("http://example.org/predicate3"));
    let object3: Object = Object::NamedNode(df.named_node("http://example.org/object3"));
    let graph3: Graph = Graph::NamedNode(df.named_node("http://example.org/graph3"));
    let quad3: Quad = df.quad(&subject1, &predicate3, &object3, Some(&graph3));

    let predicate4: Predicate =
        Predicate::NamedNode(df.named_node("http://example.org/predicate4"));
    let object4: Object = Object::NamedNode(df.named_node("http://example.org/object4"));
    let quad4: Quad = df.quad(&subject1, &predicate4, &object4, None);

    let subject5: Subject = Subject::NamedNode(df.named_node("http://example.org/subject1"));
    let predicate5: Predicate =
        Predicate::NamedNode(df.named_node("http://example.org/predicate1"));
    let object5: Object = Object::NamedNode(df.named_node("http://example.org/object1"));
    let graph5: Graph = Graph::NamedNode(df.named_node("http://example.org/graph1"));
    let quad5: Quad = df.quad(&subject5, &predicate5, &object5, Some(&graph5));

    assert_ne!(quad1, quad2);
    assert_ne!(quad2, quad3);
    assert_ne!(quad3, quad4);
    assert_ne!(quad4, quad5);
    assert_eq!(quad1, quad5);
    assert_ne!(quad1.subject, quad2.subject);
    assert_eq!(quad1.subject, quad3.subject);
    assert_eq!(quad4.graph, Graph::DefaultGraph(df.default_graph()));
}
