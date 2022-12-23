use crate::rdf::{BlankNode, Graph, Literal, NamedNode, Object, Predicate, Quad, Subject};

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
