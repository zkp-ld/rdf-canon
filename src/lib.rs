pub mod api;
pub mod canon;
pub mod counter;
pub mod error;
#[cfg(feature = "log")]
pub mod logger;
pub use crate::api::{
    canonicalize, canonicalize_graph, canonicalize_graph_with, canonicalize_quads,
    canonicalize_quads_with, canonicalize_with, issue, issue_graph, issue_graph_with, issue_quads,
    issue_quads_with, issue_with, relabel, relabel_graph, relabel_quads, sort, sort_graph,
    CanonicalizationOptions,
};
pub use crate::canon::serialize;
pub use crate::error::CanonicalizationError;
#[cfg(feature = "log")]
pub use crate::logger::YamlLayer;

#[cfg(test)]
mod tests {
    use crate::{
        canonicalize, canonicalize_with, issue, issue_with, CanonicalizationError,
        CanonicalizationOptions,
    };
    use oxrdf::Dataset;
    use oxttl::NQuadsParser;
    use serde::Deserialize;
    use sha2::Sha384;
    use std::{
        collections::HashMap,
        fs::File,
        io::{BufReader, Read},
    };

    #[cfg(feature = "log")]
    use crate::logger::YamlLayer;
    #[cfg(feature = "log")]
    use tracing::metadata::LevelFilter;
    #[cfg(feature = "log")]
    use tracing_subscriber::prelude::*;

    #[cfg(feature = "earl-reporting")]
    use chrono;

    #[cfg(feature = "log")]
    const INDENT_WIDTH: usize = 2;

    #[cfg(feature = "log")]
    fn init_logger(level: tracing::Level) {
        let _ = tracing_subscriber::registry()
            .with(YamlLayer::new(INDENT_WIDTH).with_filter(LevelFilter::from_level(level)))
            .try_init();
    }

    #[derive(Deserialize)]
    struct TestManifest {
        entries: Vec<TestManifestEntry>,
    }

    #[derive(Deserialize)]
    struct TestManifestEntry {
        id: String,
        r#type: String,
        name: String,
        action: String,
        result: Option<String>,
        #[serde(rename = "hashAlgorithm")]
        hash_algorithm: Option<String>,
    }

    #[test]
    fn test_canonicalize() {
        #[cfg(feature = "log")]
        init_logger(tracing::Level::INFO);
        // init_logger(tracing::Level::DEBUG);

        #[cfg(feature = "earl-reporting")]
        let (report_header, get_report) = setup_earl_reporting();
        #[cfg(feature = "earl-reporting")]
        println!("{}", report_header);

        const MANIFEST_PATH: &str = "tests/manifest.jsonld";

        let manifest_file = File::open(MANIFEST_PATH).unwrap();
        let manifest: TestManifest =
            serde_json::from_reader(BufReader::new(manifest_file)).unwrap();

        let canonicalize_with_sha384 = |input_dataset: &Dataset| {
            canonicalize_with::<Sha384>(
                input_dataset,
                &CanonicalizationOptions {
                    hndq_call_limit: None,
                },
            )
        };
        let issue_with_sha384 = |input_dataset: &Dataset| {
            issue_with::<Sha384>(
                input_dataset,
                &CanonicalizationOptions {
                    hndq_call_limit: None,
                },
            )
        };

        for entry in manifest.entries {
            let TestManifestEntry {
                r#id: test_id,
                r#type: test_type,
                name: test_name,
                action: input_path,
                result: output_path,
                hash_algorithm,
                ..
            } = entry;

            let input_file = File::open(format!("tests/{}", input_path)).unwrap();
            let input_quads = NQuadsParser::new()
                .parse_read(BufReader::new(input_file))
                .map(|x| x.unwrap());
            let input_dataset = Dataset::from_iter(input_quads);

            match test_type.as_str() {
                "rdfc:RDFC10EvalTest" => {
                    let canonicalized_document = match hash_algorithm {
                        None => canonicalize(&input_dataset).unwrap(),
                        Some(h) if h == "SHA384" => {
                            canonicalize_with_sha384(&input_dataset).unwrap()
                        }
                        Some(h) => panic!("invalid hashAlgorithm: {}", h),
                    };
                    let mut output_file =
                        File::open(format!("tests/{}", output_path.unwrap())).unwrap();
                    let mut expected_output = String::new();
                    output_file.read_to_string(&mut expected_output).unwrap();
                    assert_eq!(
                        canonicalized_document, expected_output,
                        "FAILED: {} - {}",
                        test_id, test_name
                    )
                }
                "rdfc:RDFC10MapTest" => {
                    let issued_identifiers_map = match hash_algorithm {
                        None => issue(&input_dataset).unwrap(),
                        Some(h) if h == "SHA384" => issue_with_sha384(&input_dataset).unwrap(),
                        Some(h) => panic!("invalid hashAlgorithm: {}", h),
                    };

                    let output_file =
                        File::open(format!("tests/{}", output_path.unwrap())).unwrap();
                    let expected_output: HashMap<String, String> =
                        serde_json::from_reader(BufReader::new(output_file)).unwrap();
                    assert_eq!(
                        issued_identifiers_map, expected_output,
                        "FAILED: {} - {}",
                        test_id, test_name
                    )
                }
                "rdfc:RDFC10NegativeEvalTest" => match canonicalize(&input_dataset) {
                    Err(CanonicalizationError::HndqCallLimitExceeded(_)) => {}
                    _ => panic!("FAILED: {} - {}", test_id, test_name),
                },
                _ => panic!("test type {} is not supported", test_type),
            }

            // println!("PASSED: {} - {}", test_id, test_name);

            #[cfg(feature = "earl-reporting")]
            println!("{}", get_report(test_id));
        }
    }

    #[test]
    fn use_sha384() {
        use crate::{canonicalize_with, CanonicalizationOptions};
        use oxrdf::Dataset;
        use oxttl::NQuadsParser;
        use sha2::Sha384;
        use std::io::Cursor;

        let input = r#"_:e0 <http://example.org/vocab#next> _:e1 _:g .
_:e0 <http://example.org/vocab#prev> _:e2 _:g .
_:e1 <http://example.org/vocab#next> _:e2 _:g .
_:e1 <http://example.org/vocab#prev> _:e0 _:g .
_:e2 <http://example.org/vocab#next> _:e0 _:g .
_:e2 <http://example.org/vocab#prev> _:e1 _:g .
<urn:ex:s> <urn:ex:p> "\u0008\u0009\u000a\u000b\u000c\u000d\u0022\u005c\u007f" _:g .
"#;
        let expected = r#"<urn:ex:s> <urn:ex:p> "\b\t\n\u000B\f\r\"\\\u007F" _:c14n0 .
_:c14n1 <http://example.org/vocab#next> _:c14n3 _:c14n0 .
_:c14n1 <http://example.org/vocab#prev> _:c14n2 _:c14n0 .
_:c14n2 <http://example.org/vocab#next> _:c14n1 _:c14n0 .
_:c14n2 <http://example.org/vocab#prev> _:c14n3 _:c14n0 .
_:c14n3 <http://example.org/vocab#next> _:c14n2 _:c14n0 .
_:c14n3 <http://example.org/vocab#prev> _:c14n1 _:c14n0 .
"#;

        let input_quads = NQuadsParser::new()
            .parse_read(Cursor::new(input))
            .map(|x| x.unwrap());
        let input_dataset = Dataset::from_iter(input_quads);
        let options = CanonicalizationOptions::default();
        let canonicalized = canonicalize_with::<Sha384>(&input_dataset, &options).unwrap();

        assert_eq!(canonicalized, expected);
    }

    #[cfg(feature = "earl-reporting")]
    fn setup_earl_reporting() -> (String, impl Fn(String) -> String) {
        const DEVELOPER_ID: &str = "https://github.com/yamdan";
        const DEVELOPER_NAME: &str = "Dan Yamamoto";

        const SOFTWARE_ID: &str = "https://github.com/zkp-ld/rdf-canon";
        const SOFTWARE_NAME: &str = "zkp-ld/rdf-canon";
        const SOFTWARE_CREATED: &str = "2024-02-26";
        const SOFTWARE_PROGRAMMING_LANGUAGE: &str = "Rust";

        const SOFTWARE_VERSION: &str = env!("CARGO_PKG_VERSION");
        let software_short_name_with_version = format!("{SOFTWARE_NAME}-{SOFTWARE_VERSION}");
        const SOFTWARE_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
        const SOFTWARE_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");

        let now = chrono::Utc::now();
        let now_date: String = now.format("%Y-%m-%d").to_string();
        let now_datetime: String = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();

        let report_header = format!(
            r#"@prefix rdf:  <http://www.w3.org/1999/02/22-rdf-syntax-ns#> .
@prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> .
@prefix dc:   <http://purl.org/dc/terms/> .
@prefix foaf: <http://xmlns.com/foaf/0.1/> .
@prefix doap: <http://usefulinc.com/ns/doap#> .
@prefix earl: <http://www.w3.org/ns/earl#> .
@prefix xsd:  <http://www.w3.org/2001/XMLSchema#> .

<> foaf:primaryTopic <{SOFTWARE_ID}> ;
  dc:issued "{now_date}"^^xsd:date ;
  foaf:maker <{DEVELOPER_ID}> .

<{SOFTWARE_ID}> a doap:Project ;
  doap:name                 "{SOFTWARE_NAME}" ;
  doap:release              [ doap:name     "{software_short_name_with_version}" ;
                              doap:revision "{SOFTWARE_VERSION}" ;
                              doap:created  "{SOFTWARE_CREATED}"^^xsd:date ;
                            ] ;
  doap:developer            <{DEVELOPER_ID}> ;
  doap:description          "{SOFTWARE_DESCRIPTION}"@en ;
  doap:programming-language "{SOFTWARE_PROGRAMMING_LANGUAGE}" ;
  doap:homepage             <{SOFTWARE_HOMEPAGE}> ;
  doap:implements           <https://www.w3.org/TR/rdf-canon/> .

<{DEVELOPER_ID}> a foaf:Person, earl:Assertor ;
  foaf:name "{DEVELOPER_NAME}" .
"#
        );

        let get_report = move |test_id| {
            format!(
                r#"[ a               earl:Assertion ;
  earl:assertedBy <{DEVELOPER_ID}> ;
  earl:subject    <{SOFTWARE_ID}> ;
  earl:test       <https://w3c.github.io/rdf-canon/tests/manifest{test_id}> ;
  earl:result     [ a            earl:TestResult ;
                    earl:outcome earl:passed ;
                    dc:date      "{now_datetime}"^^xsd:dateTime 
                  ] ;
  earl:mode     earl:automatic 
] .
"#
            )
        };

        return (report_header, get_report);
    }
}
