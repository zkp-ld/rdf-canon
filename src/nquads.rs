use crate::rdf::{
    BlankNode, DefaultGraph, Graph, GraphTrait, Literal, NamedNode, NewQuad, Object, ObjectTrait,
    Predicate, PredicateTrait, Quad, Subject, SubjectTrait, Variable,
};

pub fn serialize(quad: Quad) -> Option<String> {
    let subject = match quad.subject {
        Subject::NamedNode(n) => serialize_named_node(n),
        Subject::BlankNode(n) => serialize_blank_node(n),
        Subject::Variable(_) => return None,
    };
    let predicate = match quad.predicate {
        Predicate::NamedNode(n) => serialize_named_node(n),
        Predicate::Variable(_) => return None,
    };
    let object = match quad.object {
        Object::NamedNode(n) => serialize_named_node(n),
        Object::BlankNode(n) => serialize_blank_node(n),
        Object::Literal(n) => serialize_literal(n),
        Object::Variable(_) => return None,
    };
    let graph = match quad.graph {
        Graph::NamedNode(n) => serialize_named_node(n),
        Graph::BlankNode(n) => serialize_blank_node(n),
        Graph::DefaultGraph(_) => "".to_string(),
        Graph::Variable(_) => return None,
    };
    let result = format!("{} {} {} {}", subject, predicate, object, graph)
        .trim()
        .to_string();
    Some(format!("{} .\n", result))
}

pub fn serialize_named_node(n: NamedNode) -> String {
    format!("<{}>", n.value)
}

pub fn serialize_blank_node(n: BlankNode) -> String {
    format!("_:{}", n.value)
}

pub fn serialize_literal(l: Literal) -> String {
    // TODO: escape characters if necessary
    let value = l.value;
    match (l.language, l.datatype) {
        // If present, the language tag is preceded by a '@' (U+0040).
        (Some(lang), _) => format!("\"{}\"@{}", value, lang),
        // If there is no language tag, there may be a datatype IRI, preceded
        // by '^^' (U+005E U+005E).
        (None, Some(dt)) => format!("\"{}\"^^<{}>", value, dt.value),
        // If there is no datatype IRI and no language tag, the datatype is xsd:string.
        (None, None) => value,
    }
}

pub trait SerializeNQuads {
    fn serialize(&self) -> String;
}
impl SerializeNQuads for NamedNode {
    fn serialize(&self) -> String {
        format!("<{}>", self.value)
    }
}
impl SerializeNQuads for BlankNode {
    fn serialize(&self) -> String {
        format!("_:{}", self.value)
    }
}
impl SerializeNQuads for Literal {
    fn serialize(&self) -> String {
        // TODO: escape characters if necessary
        let value = &self.value;
        match (&self.language, &self.datatype) {
            // If present, the language tag is preceded by a '@' (U+0040).
            (Some(lang), _) => format!("\"{}\"@{}", value, lang),
            // If there is no language tag, there may be a datatype IRI, preceded
            // by '^^' (U+005E U+005E).
            (None, Some(dt)) => format!("\"{}\"^^<{}>", value, dt.value),
            // If there is no datatype IRI and no language tag, the datatype is xsd:string.
            (None, None) => value.to_string(),
        }
    }
}
impl SerializeNQuads for Variable {
    fn serialize(&self) -> String {
        format!("?{}", self.value) // TODO: fix it
    }
}
impl SerializeNQuads for DefaultGraph {
    fn serialize(&self) -> String {
        "".to_string()
    }
}

pub fn serialize_new<S, P, O, G>(quad: NewQuad<S, P, O, G>) -> Option<String>
where
    S: SubjectTrait + SerializeNQuads,
    P: PredicateTrait + SerializeNQuads,
    O: ObjectTrait + SerializeNQuads,
    G: GraphTrait + SerializeNQuads,
{
    let subject = quad.subject.serialize();
    let predicate = quad.predicate.serialize();
    let object = quad.object.serialize();
    let graph = quad.graph.serialize();
    let result = format!("{} {} {} {}", subject, predicate, object, graph)
        .trim()
        .to_string();
    Some(format!("{} .\n", result))
}

#[test]
fn test_serialize_quads() {
    let mut df = crate::rdf::DataFactory::new();
    let subject1: Subject = Subject::NamedNode(df.named_node("http://example.org/subject1"));
    let predicate1: Predicate =
        Predicate::NamedNode(df.named_node("http://example.org/predicate1"));
    let object1: Object = Object::NamedNode(df.named_node("http://example.org/object1"));
    let graph1: Graph = Graph::NamedNode(df.named_node("http://example.org/graph1"));
    let quad1: Quad = df.quad(&subject1, &predicate1, &object1, Some(&graph1));

    let subject2: Subject = Subject::BlankNode(df.blank_node(None));
    let predicate2: Predicate =
        Predicate::NamedNode(df.named_node("http://example.org/predicate2"));
    let object2: Object = Object::Literal(df.literal(
        "100",
        Some(&df.named_node("http://www.w3.org/2001/XMLSchema#integer")),
        None,
    ));
    let quad2: Quad = df.quad(&subject2, &predicate2, &object2, None);

    let subject3: Subject = Subject::BlankNode(df.blank_node(None));
    let predicate3: Predicate =
        Predicate::NamedNode(df.named_node("http://example.org/predicate3"));
    let object3: Object = Object::Literal(df.literal("あいうえお", None, Some("ja")));
    let graph3: Graph = Graph::BlankNode(df.blank_node(None));
    let quad3: Quad = df.quad(&subject3, &predicate3, &object3, Some(&graph3));

    let predicate4: Predicate =
        Predicate::NamedNode(df.named_node("http://example.org/predicate4"));
    let object4: Object = Object::NamedNode(df.named_node("http://example.org/object4"));
    let graph4: Graph = graph3;
    let quad4: Quad = df.quad(&subject1, &predicate4, &object4, Some(&graph4));

    let subject5: Subject = Subject::NamedNode(df.named_node("http://example.org/あいうえお"));
    let predicate5: Predicate =
        Predicate::NamedNode(df.named_node("http://example.org/predicate1"));
    let object5: Object = Object::NamedNode(df.named_node("http://example.org/object1"));
    let graph5: Graph = Graph::NamedNode(df.named_node("http://example.org/graph1"));
    let quad5: Quad = df.quad(&subject5, &predicate5, &object5, Some(&graph5));

    assert_eq!(serialize(quad1).unwrap(),
        "<http://example.org/subject1> <http://example.org/predicate1> <http://example.org/object1> <http://example.org/graph1> .\n".to_string());
    assert_eq!(serialize(quad2).unwrap(),
        "_:b0 <http://example.org/predicate2> \"100\"^^<http://www.w3.org/2001/XMLSchema#integer> .\n".to_string());
    assert_eq!(
        serialize(quad3).unwrap(),
        "_:b1 <http://example.org/predicate3> \"あいうえお\"@ja _:b2 .\n".to_string()
    );
    assert_eq!(serialize(quad4).unwrap(),
        "<http://example.org/subject1> <http://example.org/predicate4> <http://example.org/object4> _:b2 .\n".to_string());
    assert_eq!(serialize(quad5).unwrap(),
        "<http://example.org/あいうえお> <http://example.org/predicate1> <http://example.org/object1> <http://example.org/graph1> .\n".to_string());
}

#[test]
fn test_serialize_quads_new() {
    let subject1 = NamedNode::new("http://example.org/subject1");
    let predicate1 = NamedNode::new("http://example.org/predicate1");
    let predicate2 = NamedNode::new("http://example.org/predicate2");
    let object1 = NamedNode::new("http://example.org/object1");
    let object2 = Literal::new(
        "100",
        Some(&NamedNode::new("http://www.w3.org/2001/XMLSchema#integer")),
        None,
    );
    let graph1 = NamedNode::new("http://example.org/graph1");
    let bnode1 = BlankNode::new(None);
    let bnode2 = BlankNode::new(None);
    let default_graph = DefaultGraph::new();

    let quad1 = NewQuad::new(&subject1, &predicate1, &object1, &graph1);
    let quad2 = NewQuad::new(&bnode1, &predicate2, &bnode2, &default_graph);
    let quad3 = NewQuad::new(&bnode1, &predicate2, &object2, &default_graph);

    assert_eq!(serialize_new(quad1).unwrap(),
        "<http://example.org/subject1> <http://example.org/predicate1> <http://example.org/object1> <http://example.org/graph1> .\n".to_string());
    assert_eq!(
        serialize_new(quad2).unwrap(),
        format!(
            "{} <http://example.org/predicate2> {} .\n",
            &bnode1.serialize(),
            &bnode2.serialize(),
        )
    );
    assert_eq!(serialize_new(quad3).unwrap(),
        format!("{} <http://example.org/predicate2> \"100\"^^<http://www.w3.org/2001/XMLSchema#integer> .\n", &bnode1.serialize()));
}
