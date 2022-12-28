use crate::rdf::{
    BlankNode, DefaultGraph, Graph, Literal, NamedNode, Object, Predicate, Quad, Subject, Term,
    Variable,
};

pub trait SerializeNQuads {
    fn serialize(&self) -> String;
}
impl SerializeNQuads for NamedNode {
    fn serialize(&self) -> String {
        format!("<{}>", self.value())
    }
}
impl SerializeNQuads for BlankNode {
    fn serialize(&self) -> String {
        format!("_:{}", self.value())
    }
}
impl SerializeNQuads for Literal {
    fn serialize(&self) -> String {
        // TODO: escape characters if necessary
        let value = &self.value();
        match (&self.language, &self.datatype) {
            // If present, the language tag is preceded by a '@' (U+0040).
            (Some(lang), _) => format!("\"{}\"@{}", value, lang),
            // If there is no language tag, there may be a datatype IRI, preceded
            // by '^^' (U+005E U+005E).
            (None, Some(dt)) => format!("\"{}\"^^<{}>", value, dt.value()),
            // If there is no datatype IRI and no language tag, the datatype is xsd:string.
            (None, None) => value.to_string(),
        }
    }
}
impl SerializeNQuads for Variable {
    fn serialize(&self) -> String {
        format!("?{}", self.value()) // TODO: fix it
    }
}
impl SerializeNQuads for DefaultGraph {
    fn serialize(&self) -> String {
        "".to_string()
    }
}
impl SerializeNQuads for Subject {
    fn serialize(&self) -> String {
        match self {
            Self::NamedNode(x) => x.serialize(),
            Self::BlankNode(x) => x.serialize(),
            Self::Variable(x) => x.serialize(),
        }
    }
}
impl SerializeNQuads for Predicate {
    fn serialize(&self) -> String {
        match self {
            Self::NamedNode(x) => x.serialize(),
            Self::Variable(x) => x.serialize(),
        }
    }
}
impl SerializeNQuads for Object {
    fn serialize(&self) -> String {
        match self {
            Self::NamedNode(x) => x.serialize(),
            Self::BlankNode(x) => x.serialize(),
            Self::Literal(x) => x.serialize(),
            Self::Variable(x) => x.serialize(),
        }
    }
}
impl SerializeNQuads for Graph {
    fn serialize(&self) -> String {
        match self {
            Self::NamedNode(x) => x.serialize(),
            Self::BlankNode(x) => x.serialize(),
            Self::DefaultGraph(x) => x.serialize(),
        }
    }
}

impl SerializeNQuads for Quad {
    fn serialize(&self) -> String {
        let subject = self.subject.serialize();
        let predicate = self.predicate.serialize();
        let object = self.object.serialize();
        let graph = self.graph.serialize();
        let result = format!("{} {} {} {}", subject, predicate, object, graph)
            .trim()
            .to_string();
        format!("{} .\n", result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_quads() {
        let subject1 = NamedNode::new("http://example.org/subject1");
        let subject2 = NamedNode::new("http://example.org/あいうえお");
        let predicate1 = NamedNode::new("http://example.org/predicate1");
        let predicate2 = NamedNode::new("http://example.org/predicate2");
        let object1 = NamedNode::new("http://example.org/object1");
        let object2 = Literal::new(
            "100",
            Some(&NamedNode::new("http://www.w3.org/2001/XMLSchema#integer")),
            None,
        );
        let object3 = Literal::new("あいうえお", None, Some("ja"));
        let graph1 = NamedNode::new("http://example.org/graph1");
        let bnode1 = BlankNode::new(None);
        let bnode2 = BlankNode::new(None);
        let default_graph = DefaultGraph::new();

        let quad1 = Quad::new(
            &Subject::NamedNode(subject1.clone()),
            &Predicate::NamedNode(predicate1.clone()),
            &Object::NamedNode(object1.clone()),
            &Graph::NamedNode(graph1.clone()),
        );
        let quad2 = Quad::new(
            &Subject::BlankNode(bnode1.clone()),
            &Predicate::NamedNode(predicate2.clone()),
            &Object::BlankNode(bnode2.clone()),
            &Graph::DefaultGraph(default_graph.clone()),
        );
        let quad3 = Quad::new(
            &Subject::BlankNode(bnode1.clone()),
            &Predicate::NamedNode(predicate2.clone()),
            &Object::Literal(object2.clone()),
            &Graph::DefaultGraph(default_graph.clone()),
        );
        let quad4 = Quad::new(
            &Subject::NamedNode(subject2.clone()),
            &Predicate::NamedNode(predicate1.clone()),
            &Object::Literal(object3.clone()),
            &Graph::DefaultGraph(default_graph.clone()),
        );

        assert_eq!(quad1.serialize(),
        "<http://example.org/subject1> <http://example.org/predicate1> <http://example.org/object1> <http://example.org/graph1> .\n".to_string());
        assert_eq!(
            quad2.serialize(),
            format!(
                "{} <http://example.org/predicate2> {} .\n",
                &bnode1.serialize(),
                &bnode2.serialize(),
            )
        );
        assert_eq!(quad3.serialize(),
        format!("{} <http://example.org/predicate2> \"100\"^^<http://www.w3.org/2001/XMLSchema#integer> .\n", &bnode1.serialize()));
        assert_eq!(
            quad4.serialize(),
            "<http://example.org/あいうえお> <http://example.org/predicate1> \"あいうえお\"@ja .\n"
                .to_string()
        );
    }
}
