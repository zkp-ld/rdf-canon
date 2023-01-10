mod canon;
mod error;
mod nquads;
mod rdf;

#[cfg(test)]
mod tests {
    use crate::{
        canon::canonicalize,
        nquads::{parse, SerializeNQuads},
    };
    use std::{fs::File, io::Read, path::Path};
    use tracing_subscriber::fmt;

    fn init(level: tracing::Level) {
        let log_format = fmt::format()
            .with_level(false)
            .with_target(false)
            .without_time()
            .compact();
        let _ = fmt()
            .with_max_level(level)
            .event_format(log_format)
            .try_init();
    }

    #[test]
    fn test_canonicalize_unique_hash_example() {
        let input_dataset = r#"<http://example.com/#p> <http://example.com/#q> _:e0 .
<http://example.com/#p> <http://example.com/#r> _:e1 .
_:e0 <http://example.com/#s> <http://example.com/#u> .
_:e1 <http://example.com/#t> <http://example.com/#u> .
"#;
        let input_dataset = parse(input_dataset).unwrap();
        let mut canonicalized_dataset = canonicalize(&input_dataset).unwrap();
        canonicalized_dataset.sort();

        let expected_output = r#"<http://example.com/#p> <http://example.com/#q> _:c14n0 .
<http://example.com/#p> <http://example.com/#r> _:c14n1 .
_:c14n0 <http://example.com/#s> <http://example.com/#u> .
_:c14n1 <http://example.com/#t> <http://example.com/#u> .
"#;
        assert_eq!(canonicalized_dataset.serialize(), expected_output);
    }

    #[test]
    fn test_canonicalize_shared_hash_example() {
        let input_dataset = r#"<http://example.com/#p> <http://example.com/#q> _:e0 .
<http://example.com/#p> <http://example.com/#q> _:e1 .
_:e0 <http://example.com/#p> _:e2 .
_:e1 <http://example.com/#p> _:e3 .
_:e2 <http://example.com/#r> _:e3 .
"#;
        let input_dataset = parse(input_dataset).unwrap();
        let mut canonicalized_dataset = canonicalize(&input_dataset).unwrap();
        canonicalized_dataset.sort();

        let expected_output = r#"<http://example.com/#p> <http://example.com/#q> _:c14n2 .
<http://example.com/#p> <http://example.com/#q> _:c14n3 .
_:c14n0 <http://example.com/#r> _:c14n1 .
_:c14n2 <http://example.com/#p> _:c14n1 .
_:c14n3 <http://example.com/#p> _:c14n0 .
"#;
        assert_eq!(canonicalized_dataset.serialize(), expected_output);
    }

    #[test]
    fn test_canonicalize_duplicated_paths_example() {
        let input_dataset = r#"_:e0 <http://example.org/vocab#p1> _:e1 .
_:e1 <http://example.org/vocab#p2> "Foo" .
_:e2 <http://example.org/vocab#p1> _:e3 .
_:e3 <http://example.org/vocab#p2> "Foo" .
"#;
        let input_dataset = parse(input_dataset).unwrap();
        let mut canonicalized_dataset = canonicalize(&input_dataset).unwrap();
        canonicalized_dataset.sort();

        let expected_output = r#"_:c14n0 <http://example.org/vocab#p1> _:c14n1 .
_:c14n1 <http://example.org/vocab#p2> "Foo" .
_:c14n2 <http://example.org/vocab#p1> _:c14n3 .
_:c14n3 <http://example.org/vocab#p2> "Foo" .
"#;
        assert_eq!(canonicalized_dataset.serialize(), expected_output);
    }

    #[test]
    fn test_canonicalize() {
        //init(tracing::Level::DEBUG);

        const BASE_PATH: &str = "tests/urdna2015";

        fn read_nquads(path: &str) -> Option<String> {
            let path = Path::new(&path);
            let mut file = match File::open(path) {
                Err(_) => return None,
                Ok(file) => file,
            };
            let mut s = String::new();
            match file.read_to_string(&mut s) {
                Err(why) => panic!("couldn't read {}: {}", path.display(), why),
                Ok(_) => Some(s),
            }
        }

        let range = 1..=63;
        for i in range {
            let input_path = format!("{BASE_PATH}/test{:03}-in.nq", i);
            let input = match read_nquads(&input_path) {
                Some(s) => s,
                None => continue,
            };
            let output_path = format!("{BASE_PATH}/test{:03}-urdna2015.nq", i);
            let output = match read_nquads(&output_path) {
                Some(s) => s,
                None => continue,
            };

            let input_dataset = parse(&input).unwrap();
            let mut canonicalized_dataset = canonicalize(&input_dataset).unwrap();
            canonicalized_dataset.sort();

            assert_eq!(
                canonicalized_dataset.serialize(),
                output,
                "Failed: test{:03}",
                i
            );
        }
    }
}
