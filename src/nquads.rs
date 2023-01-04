use crate::rdf::{
    BlankNode, DefaultGraph, Graph, Literal, NamedNode, Object, Predicate, Quad, Subject, Term,
    Variable,
};
use nom::branch::{alt, permutation};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, line_ending, none_of, space0, space1};
use nom::combinator::{map, opt};
use nom::multi::many0;
use nom::sequence::{delimited, preceded, tuple};
use nom::IResult;

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

impl SerializeNQuads for Vec<Quad> {
    fn serialize(&self) -> String {
        self.iter().map(|q| q.serialize()).collect::<String>()
    }
}

fn langtag(input: &str) -> IResult<&str, String> {
    map(
        permutation((tag("@"), alpha1, many0(tuple((tag("-"), alphanumeric1))))),
        |(_, langtag, subtags)| -> String {
            format!(
                "{}{}",
                langtag,
                subtags
                    .into_iter()
                    .map(|(l, r)| format!("{}{}", l, r))
                    .collect::<Vec<String>>()
                    .join("")
            )
        },
    )(input)
}

fn iriref(input: &str) -> IResult<&str, NamedNode> {
    // TODO: exclude #x00-#x20
    // TODO: include UCHAR
    let (input, value) = delimited(char('<'), many0(none_of("<>\"{}|^`\\")), char('>'))(input)?;
    let value: &str = &value.iter().collect::<String>();
    Ok((input, NamedNode::new(value)))
}

fn string_literal_quote(input: &str) -> IResult<&str, String> {
    // TODO: include ECHAR and UCHAR
    let (input, value) = delimited(char('\"'), many0(none_of("\"\\\n\r")), char('\"'))(input)?;
    let value: String = value.iter().collect::<String>();
    Ok((input, value))
}

fn blank_node_label(input: &str) -> IResult<&str, BlankNode> {
    let (input, _) = tag("_:")(input)?;
    // TODO: use PN_CHARS_U etc. instead of alphanumeric1
    let (input, value) = alphanumeric1(input)?;
    Ok((input, BlankNode::new(Some(value))))
}

fn literal(input: &str) -> IResult<&str, Literal> {
    alt((
        map(
            permutation((string_literal_quote, preceded(tag("^^"), iriref))),
            |(value, d)| -> Literal { Literal::new(&value, Some(&d), None) },
        ),
        map(
            permutation((string_literal_quote, opt(langtag))),
            |(value, l)| -> Literal { Literal::new(&value, None, l.as_deref()) },
        ),
    ))(input)
}

fn subject(input: &str) -> IResult<&str, Subject> {
    alt((
        map(iriref, Subject::NamedNode),
        map(blank_node_label, Subject::BlankNode),
    ))(input)
}

fn predicate(input: &str) -> IResult<&str, Predicate> {
    map(iriref, Predicate::NamedNode)(input)
}

fn object(input: &str) -> IResult<&str, Object> {
    alt((
        map(iriref, Object::NamedNode),
        map(blank_node_label, Object::BlankNode),
        map(literal, Object::Literal),
    ))(input)
}

fn empty(input: &str) -> IResult<&str, ()> {
    Ok((input, ()))
}

fn graph(input: &str) -> IResult<&str, Graph> {
    alt((
        map(iriref, Graph::NamedNode),
        map(blank_node_label, Graph::BlankNode),
        map(empty, |()| Graph::DefaultGraph(DefaultGraph::new())),
    ))(input)
}

fn statement(input: &str) -> IResult<&str, Quad> {
    map(
        permutation((
            subject,
            space1,
            predicate,
            space1,
            object,
            space1,
            graph,
            space0,
            char('.'),
        )),
        |(s, _, p, _, o, _, g, _, _)| Quad::new(&s, &p, &o, &g),
    )(input)
}

fn nquads_doc(input: &str) -> IResult<&str, Vec<Quad>> {
    map(
        permutation((
            opt(statement),
            many0(preceded(line_ending, statement)),
            opt(line_ending),
        )),
        |(first_statement, rest_statements, _)| {
            let mut quads = Vec::<Quad>::new();
            if let Some(q) = first_statement {
                quads.push(q);
            }
            for q in rest_statements {
                quads.push(q);
            }
            quads
        },
    )(input)
}

pub fn parse(input: &str) -> Result<Vec<Quad>, nom::Err<nom::error::Error<&str>>> {
    match nquads_doc(input) {
        Ok((_, quads)) => Ok(quads),
        Err(e) => Err(e),
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

    #[test]
    fn parse_iri() {
        assert_eq!(
            iriref("<http://example.org/#1>"),
            Ok(("", NamedNode::new("http://example.org/#1")))
        );
        assert_eq!(
            iriref("<http://example.org/#1> <p> <o> ."),
            Ok((" <p> <o> .", NamedNode::new("http://example.org/#1")))
        );
        assert_eq!(
            iriref("<http://example.org/#1 <p> <o> ."),
            Err(nom::Err::Error(nom::error::Error::new(
                "<p> <o> .",
                nom::error::ErrorKind::Char
            )))
        );
        assert_eq!(
            iriref("<http://example.}org/#1> <p> <o> ."),
            Err(nom::Err::Error(nom::error::Error::new(
                "}org/#1> <p> <o> .",
                nom::error::ErrorKind::Char
            )))
        );
    }

    #[test]
    fn parse_blanknode() {
        assert_eq!(
            blank_node_label("_:b0"),
            Ok(("", BlankNode::new(Some("b0"))))
        );
        assert_eq!(
            blank_node_label("_:c14n9  "),
            Ok(("  ", BlankNode::new(Some("c14n9"))))
        );
        assert_eq!(
            blank_node_label("_:-b0"),
            Err(nom::Err::Error(nom::error::Error::new(
                "-b0",
                nom::error::ErrorKind::AlphaNumeric
            )))
        );
        assert_eq!(
            blank_node_label("_:_b0"),
            Err(nom::Err::Error(nom::error::Error::new(
                "_b0",
                nom::error::ErrorKind::AlphaNumeric
            ))) // Ok(("", BlankNode::new(Some("_b0"))))  // TODO: should have been accepted
        );
    }

    #[test]
    fn parse_langtag() {
        assert_eq!(langtag("@ja"), Ok(("", "ja".to_string())));
        assert_eq!(langtag("@ja-Hira"), Ok(("", "ja-Hira".to_string())));
        assert_eq!(langtag("@ja-Hira"), Ok(("", "ja-Hira".to_string())));
        assert_eq!(
            langtag("ja"),
            Err(nom::Err::Error(nom::error::Error::new(
                "",
                nom::error::ErrorKind::Tag
            )))
        );
        assert_eq!(langtag("@ja--"), Ok(("--", "ja".to_string())));
    }

    #[test]
    fn parse_literal() {
        assert_eq!(
            literal("\"abc\""),
            Ok(("", Literal::new("abc", None, None)))
        );
        assert_eq!(
            literal("\"abc\"^^<http://www.w3.org/2001/XMLSchema#string>"),
            Ok((
                "",
                Literal::new(
                    "abc",
                    Some(&NamedNode::new("http://www.w3.org/2001/XMLSchema#string")),
                    None
                )
            ))
        );
        assert_eq!(
            literal("\"abc\"@en"),
            Ok(("", Literal::new("abc", None, Some("en"))))
        );
        assert_eq!(
            literal("\"あいうえお\"@ja-Hira"),
            Ok(("", Literal::new("あいうえお", None, Some("ja-Hira"))))
        );
        assert_eq!(
            literal("\"  abc  \"^^<http://www.w3.org/2001/XMLSchema#string>"),
            Ok((
                "",
                Literal::new(
                    "  abc  ",
                    Some(&NamedNode::new("http://www.w3.org/2001/XMLSchema#string")),
                    None
                )
            ))
        );
        assert_eq!(
            literal("\"12345\"^^<http://www.w3.org/2001/XMLSchema#integer>"),
            Ok((
                "",
                Literal::new(
                    "12345",
                    Some(&NamedNode::new("http://www.w3.org/2001/XMLSchema#integer")),
                    None
                )
            ))
        );
        assert_eq!(
            literal("\"12345\"^^foo"),
            Ok(("^^foo", Literal::new("12345", None, None)))
        );
    }

    #[test]
    fn parse_subject() {
        assert_eq!(
            subject("<http://example.org/#1>"),
            Ok((
                "",
                Subject::NamedNode(NamedNode::new("http://example.org/#1"))
            ))
        );
        assert_eq!(
            subject("_:b0"),
            Ok(("", Subject::BlankNode(BlankNode::new(Some("b0")))))
        );
    }

    #[test]
    fn parse_statement() {
        assert_eq!(
            statement("<http://example.org/#s> <http://example.org/#p> <http://example.org/#o> <http://example.org/#g> ."),
            Ok((
                "",
                Quad::new(
                    &Subject::NamedNode(NamedNode::new("http://example.org/#s")),
                    &Predicate::NamedNode(NamedNode::new("http://example.org/#p")),
                    &Object::NamedNode(NamedNode::new("http://example.org/#o")),
                    &Graph::NamedNode(NamedNode::new("http://example.org/#g")),
                )
            ))
        );
        assert_eq!(
            statement("<http://example.org/#s> <http://example.org/#p> <http://example.org/#o> ."),
            Ok((
                "",
                Quad::new(
                    &Subject::NamedNode(NamedNode::new("http://example.org/#s")),
                    &Predicate::NamedNode(NamedNode::new("http://example.org/#p")),
                    &Object::NamedNode(NamedNode::new("http://example.org/#o")),
                    &Graph::DefaultGraph(DefaultGraph::new()),
                )
            ))
        );
        assert_eq!(
            statement("_:b0 <http://example.org/#p> _:b1 <http://example.org/#g> ."),
            Ok((
                "",
                Quad::new(
                    &Subject::BlankNode(BlankNode::new(Some("b0"))),
                    &Predicate::NamedNode(NamedNode::new("http://example.org/#p")),
                    &Object::BlankNode(BlankNode::new(Some("b1"))),
                    &Graph::NamedNode(NamedNode::new("http://example.org/#g")),
                )
            ))
        );
        assert_eq!(
            statement("_:b0 _:b1 _:b2 _:b3 ."),
            Err(nom::Err::Error(nom::error::Error::new(
                "_:b3 .",
                nom::error::ErrorKind::Char
            )))
        );
    }

    #[test]
    fn parse_test() {
        assert_eq!(
            parse(
                r#"
<http://example.org/#s>  <http://example.org/#p>  <http://example.org/#o>  <http://example.org/#g>  .
<http://example.org/#s2> <http://example.org/#p2> <http://example.org/#o2> <http://example.org/#g2> .
_:b0 <http://example.org/#p> _:b1 <http://example.org/#g> .
"#
            ).unwrap(),
            vec![
                Quad::new(
                    &Subject::NamedNode(NamedNode::new("http://example.org/#s")),
                    &Predicate::NamedNode(NamedNode::new("http://example.org/#p")),
                    &Object::NamedNode(NamedNode::new("http://example.org/#o")),
                    &Graph::NamedNode(NamedNode::new("http://example.org/#g")),
                ),
                Quad::new(
                    &Subject::NamedNode(NamedNode::new("http://example.org/#s2")),
                    &Predicate::NamedNode(NamedNode::new("http://example.org/#p2")),
                    &Object::NamedNode(NamedNode::new("http://example.org/#o2")),
                    &Graph::NamedNode(NamedNode::new("http://example.org/#g2")),
                ),
                Quad::new(
                    &Subject::BlankNode(BlankNode::new(Some("b0"))),
                    &Predicate::NamedNode(NamedNode::new("http://example.org/#p")),
                    &Object::BlankNode(BlankNode::new(Some("b1"))),
                    &Graph::NamedNode(NamedNode::new("http://example.org/#g")),
                ),
            ]
        );
    }
}
