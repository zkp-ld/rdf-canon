/// RDF data interfaces based on
/// [RDF/JS: Data model specification](https://rdf.js.org/data-model-spec/)

/// An abstract interface
pub trait Term {
    /// Returns true when called with parameter other on an object term
    /// if all of the conditions below hold:
    /// - other is neither null nor undefined;
    /// - term.termType is the same string as other.termType;
    /// - other follows the additional constraints of the specific Term
    ///   interface implemented by term (e.g., NamedNode, Literal, …);
    /// otherwise, it returns false.
    fn equals(&self, other: &Self) -> bool;
}

#[derive(Clone, Debug)]
pub struct NamedNode {
    /// The IRI of the named node (example: "http://example.org/resource").
    value: String,
}

impl Term for NamedNode {
    /// Returns true if all general Term.equals conditions hold and
    /// term.value is the same string as other.value; otherwise, it
    /// returns false.
    fn equals(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Clone, Debug)]
pub struct BlankNode {
    /// Blank node name as a string, without any serialization specific
    /// prefixes, e.g. when parsing, if the data was sourced from Turtle,
    /// remove "_:", if it was sourced from RDF/XML, do not change the
    /// blank node name (example: "blank3").
    value: String,
}

impl Term for BlankNode {
    /// Returns true if all general Term.equals conditions hold and
    /// term.value is the same string as other.value; otherwise, it
    /// returns false.
    fn equals(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

#[derive(Clone, Debug)]
pub struct Literal {
    /// The text value, unescaped, without language or type (example: "Brad
    /// Pitt").
    value: String,
    /// The language as lowercase BCP-47 [BCP47] string (examples: "en",
    /// "en-gb") or an empty string if the literal has no language.
    language: String,
    /// A NamedNode whose IRI represents the datatype of the literal.
    datatype: NamedNode,
}

impl Term for Literal {
    /// Returns true if all general Term.equals conditions hold, term.value
    /// is the same string as other.value, term.language is the same string
    /// as other.language, and term.datatype.equals(other.datatype) evaluates
    /// to true; otherwise, it returns false.
    fn equals(&self, other: &Self) -> bool {
        self.value == other.value
            && self.language == other.language
            && self.datatype.equals(&other.datatype)
    }
}

#[derive(Clone, Debug)]
pub struct Variable {
    /// The name of the variable without leading "?" (example: "a").
    value: String,
}

impl Term for Variable {
    /// Returns true if all general Term.equals conditions hold and
    /// term.value is the same string as other.value; otherwise, it
    /// returns false.
    fn equals(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

/// An instance of DefaultGraph represents the default graph. It's only
/// allowed to assign a DefaultGraph to the graph property of a Quad.
#[derive(Clone, Debug)]
pub struct DefaultGraph {
    // Contains an empty string as constant value.
    // NOTE: We omit the empty string `value` here.
    // _value: String,
}

impl Term for DefaultGraph {
    /// Returns true if all general Term.equals conditions hold;
    /// otherwise, it returns false.
    fn equals(&self, _other: &Self) -> bool {
        true
    }
}

/// The subject, which is a NamedNode, BlankNode, Variable or Quad.
/// NOTE: We do not currently support Quad as a subject here.
#[derive(Clone, Debug)]
pub enum Subject {
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Variable(Variable),
    // Quad(Quad),
}

impl Term for Subject {
    fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NamedNode(left), Self::NamedNode(right)) => left.equals(right),
            (Self::BlankNode(left), Self::BlankNode(right)) => left.equals(right),
            (Self::Variable(left), Self::Variable(right)) => left.equals(right),
            _ => false,
        }
    }
}

/// The predicate, which is a NamedNode or Variable.
#[derive(Clone, Debug)]
pub enum Predicate {
    NamedNode(NamedNode),
    Variable(Variable),
}

impl Term for Predicate {
    fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NamedNode(left), Self::NamedNode(right)) => left.equals(right),
            (Self::Variable(left), Self::Variable(right)) => left.equals(right),
            _ => false,
        }
    }
}

/// The object, which is a NamedNode, Literal, BlankNode or Variable.
#[derive(Clone, Debug)]
pub enum Object {
    NamedNode(NamedNode),
    Literal(Literal),
    BlankNode(BlankNode),
    Variable(Variable),
}

impl Term for Object {
    fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NamedNode(left), Self::NamedNode(right)) => left.equals(right),
            (Self::Literal(left), Self::Literal(right)) => left.equals(right),
            (Self::BlankNode(left), Self::BlankNode(right)) => left.equals(right),
            (Self::Variable(left), Self::Variable(right)) => left.equals(right),
            _ => false,
        }
    }
}

/// The named graph, which is a DefaultGraph, NamedNode, BlankNode or
/// Variable.
#[derive(Clone, Debug)]
pub enum Graph {
    DefaultGraph(DefaultGraph),
    NamedNode(NamedNode),
    BlankNode(BlankNode),
    Variable(Variable),
}

impl Term for Graph {
    fn equals(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::DefaultGraph(left), Self::DefaultGraph(right)) => left.equals(right),
            (Self::NamedNode(left), Self::NamedNode(right)) => left.equals(right),
            (Self::BlankNode(left), Self::BlankNode(right)) => left.equals(right),
            (Self::Variable(left), Self::Variable(right)) => left.equals(right),
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Quad {
    // Contains an empty string as constant value.
    // NOTE: We omit the empty string `value` here.
    // value: String,
    /// The subject, which is a NamedNode, BlankNode, Variable or Quad.
    /// NOTE: We do not currently support Quad as a subject here
    subject: Subject,
    /// The predicate, which is a NamedNode or Variable.
    predicate: Predicate,
    /// The object, which is a NamedNode, Literal, BlankNode or Variable.
    object: Object,
    /// The named graph, which is a DefaultGraph, NamedNode, BlankNode or
    /// Variable.
    graph: Graph,
}

impl Term for Quad {
    /// Returns true when called with parameter other on an object quad if all of the conditions below hold:
    /// - other is neither null nor undefined;
    /// - quad.subject.equals(other.subject) evaluates to true;
    /// - quad.predicate.equals(other.predicate) evaluates to true;
    /// - quad.object.equals(other.object) evaluates to a true;
    /// - quad.graph.equals(other.graph) evaluates to a true;
    /// otherwise, it returns false.
    fn equals(&self, other: &Self) -> bool {
        self.subject.equals(&other.subject)
            && self.predicate.equals(&other.predicate)
            && self.object.equals(&other.object)
            && self.graph.equals(&other.graph)
    }
}

const DEFAULT_BLANK_NODE_PREFIX: &str = "b";
const DEFAULT_BLANK_NODE_COUNTER: usize = 0;

#[derive(Debug)]
pub struct DataFactory {
    blank_node_prefix: String,
    blank_node_counter: usize,
}

impl DataFactory {
    pub fn new() -> DataFactory {
        DataFactory {
            blank_node_prefix: DEFAULT_BLANK_NODE_PREFIX.to_string(),
            blank_node_counter: DEFAULT_BLANK_NODE_COUNTER,
        }
    }

    /// Returns a new instance of NamedNode.
    fn named_node(&self, value: &str) -> NamedNode {
        NamedNode {
            value: value.to_string(),
        }
    }

    /// Returns a new instance of BlankNode. If the value parameter is undefined
    /// a new identifier for the blank node is generated for each call.
    fn blank_node(&mut self, value: Option<&str>) -> BlankNode {
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
    fn literal(
        &self,
        value: &str,
        datatype: Option<&NamedNode>,
        language: Option<&str>,
    ) -> Literal {
        match datatype {
            Some(datatype) => Literal {
                value: value.to_string(),
                datatype: datatype.clone(),
                language: "".to_string(),
            },
            None => match language {
                Some(language) => Literal {
                    value: value.to_string(),
                    language: language.to_string(),
                    datatype: self
                        .named_node("http://www.w3.org/1999/02/22-rdf-syntax-ns#langString"),
                },
                None => Literal {
                    value: value.to_string(),
                    language: "".to_string(),
                    datatype: self.named_node("http://www.w3.org/2001/XMLSchema#string"),
                },
            },
        }
    }

    /// Returns a new instance of Variable. This method is optional.
    fn variable(&self, value: &str) -> Variable {
        Variable {
            value: value.to_string(),
        }
    }

    /// Returns an instance of DefaultGraph.
    fn default_graph(&self) -> DefaultGraph {
        DefaultGraph {}
    }

    /// Returns a new instance of Quad. If graph is undefined or null it MUST set graph
    /// to a DefaultGraph.
    fn quad(
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
    assert!(!named_node1.equals(&named_node2));
    assert!(!named_node2.equals(&named_node3));
    assert!(named_node3.equals(&named_node1));
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
    assert!(!blank_node1.equals(&blank_node2));
    assert!(!blank_node2.equals(&blank_node3));
    assert!(!blank_node3.equals(&blank_node1));
    assert!(!blank_node3.equals(&blank_node4));
    assert!(!blank_node4.equals(&blank_node5));
    assert!(blank_node3.equals(&blank_node5));
}

#[test]
fn gen_literal() {
    let df = DataFactory::new();
    let literal1 = df.literal("foo", None, None);
    let literal2 = df.literal("bar", None, None);
    let literal3 = df.literal("foo", None, None);
    let literal4_en = df.literal("foo", None, Some("en"));
    let literal4_ja = df.literal("あいうえお", None, Some("ja"));
    let literal5_en = df.literal(
        "123",
        Some(&df.named_node("http://www.w3.org/2001/XMLSchema#integer")),
        None,
    );
    assert_eq!(literal1.value, "foo");
    assert_eq!(literal2.value, "bar");
    assert_eq!(literal3.value, "foo");
    assert_eq!(literal4_en.value, "foo");
    assert_eq!(literal4_ja.value, "あいうえお");
    assert_eq!(literal5_en.value, "123");
    assert!(!literal1.equals(&literal2));
    assert!(!literal2.equals(&literal3));
    assert!(literal3.equals(&literal1));
    assert!(!literal4_en.equals(&literal4_ja));
    assert!(!literal4_en.equals(&literal4_ja));
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
    assert!(!variable1.equals(&variable2));
    assert!(!variable2.equals(&variable3));
    assert!(variable3.equals(&variable1));
}

#[test]
fn gen_default_graph() {
    let df = DataFactory::new();
    let default_graph1 = df.default_graph();
    let default_graph2 = df.default_graph();
    assert!(default_graph1.equals(&default_graph2));
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
    let quad4: Quad = df.quad(&subject1, &predicate3, &object3, None);

    let subject5: Subject = Subject::NamedNode(df.named_node("http://example.org/subject1"));
    let predicate5: Predicate =
        Predicate::NamedNode(df.named_node("http://example.org/predicate1"));
    let object5: Object = Object::NamedNode(df.named_node("http://example.org/object1"));
    let graph5: Graph = Graph::NamedNode(df.named_node("http://example.org/graph1"));
    let quad5: Quad = df.quad(&subject5, &predicate5, &object5, Some(&graph5));

    assert!(!quad1.equals(&quad2));
    assert!(!quad2.equals(&quad3));
    assert!(!quad3.equals(&quad4));
    assert!(!quad4.equals(&quad5));
    assert!(quad1.equals(&quad5));
    assert!(!quad1.subject.equals(&quad2.subject));
    assert!(quad1.subject.equals(&quad3.subject));
    assert!(quad4.graph.equals(&Graph::DefaultGraph(df.default_graph())));
}
