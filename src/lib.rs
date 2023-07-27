pub mod api;
pub mod canon;
pub mod counter;
pub mod error;
#[cfg(feature = "log")]
pub mod logger;
pub use crate::api::{
    canonicalize, canonicalize_quads, canonicalize_quads_with_options, canonicalize_with_options,
    issue, issue_quads, issue_quads_with_options, issue_with_options, relabel, relabel_quads,
    CanonicalizationOptions,
};
pub use crate::canon::serialize;
pub use crate::error::CanonicalizationError;
#[cfg(feature = "log")]
pub use crate::logger::YamlLayer;

#[cfg(test)]
mod tests {
    use crate::{canonicalize, issue, CanonicalizationError};
    use oxrdf::Dataset;
    use oxttl::NQuadsParser;
    use serde::Deserialize;
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
    }

    #[test]
    fn test_canonicalize() {
        #[cfg(feature = "log")]
        init_logger(tracing::Level::INFO);
        // init_logger(tracing::Level::DEBUG);

        const MANIFEST_PATH: &str = "tests/manifest.jsonld";

        let manifest_file = File::open(MANIFEST_PATH).unwrap();
        let manifest: TestManifest =
            serde_json::from_reader(BufReader::new(manifest_file)).unwrap();

        for entry in manifest.entries {
            let TestManifestEntry {
                r#id: test_id,
                r#type: test_type,
                name: test_name,
                action: input_path,
                result: output_path,
                ..
            } = entry;

            let input_file = File::open(format!("tests/{}", input_path)).unwrap();
            let input_quads = NQuadsParser::new()
                .parse_from_read(BufReader::new(input_file))
                .into_iter()
                .map(|x| x.unwrap());
            let input_dataset = Dataset::from_iter(input_quads);

            match test_type.as_str() {
                "rdfc:RDFC10EvalTest" => {
                    let canonicalized_document = canonicalize(&input_dataset).unwrap();
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
                    let issued_identifiers_map = issue(&input_dataset).unwrap();
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

            println!("PASSED: {} - {}", test_id, test_name);
        }
    }
}
