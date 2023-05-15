pub mod canon;
pub mod error;
pub use crate::canon::{canonicalize, serialize};
pub use crate::error::CanonicalizationError;

#[cfg(test)]
mod tests {
    use crate::canon::{canonicalize, serialize};
    use oxigraph::io::{DatasetFormat, DatasetParser};
    use oxrdf::Dataset;
    use std::{
        fs::File,
        io::{BufReader, Read},
        path::Path,
    };

    #[cfg(feature = "log")]
    use tracing::metadata::LevelFilter;
    #[cfg(feature = "log")]
    use tracing_subscriber::{fmt, prelude::*};
    #[cfg(feature = "log")]
    mod logger;
    #[cfg(feature = "log")]
    use logger::CustomLayer;

    #[cfg(feature = "log")]
    const INDENT_WIDTH: usize = 2;

    #[cfg(feature = "log")]
    fn _init(level: tracing::Level) {
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
    #[cfg(feature = "log")]
    fn init(level: tracing::Level) {
        let _ = tracing_subscriber::registry()
            .with(CustomLayer::new(INDENT_WIDTH).with_filter(LevelFilter::from_level(level)))
            .try_init();
    }

    #[test]
    fn test_canonicalize() {
        #[cfg(feature = "log")]
        init(tracing::Level::INFO);
        // init(tracing::Level::DEBUG);

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

        let range = 1..=69;
        for i in range {
            let input_path = format!("{BASE_PATH}/test{:03}-in.nq", i);
            let parser = DatasetParser::from_format(DatasetFormat::NQuads);
            let file = BufReader::new(File::open(input_path).unwrap());
            let input_dataset =
                Dataset::from_iter(parser.read_quads(file).unwrap().map(|x| x.unwrap()));

            let normalized_dataset = canonicalize(&input_dataset).unwrap();
            let canonicalized_document = serialize(normalized_dataset);

            let output_path = format!("{BASE_PATH}/test{:03}-urdna2015.nq", i);
            let expected_output = match read_nquads(&output_path) {
                Some(s) => s,
                None => continue,
            };

            assert_eq!(
                canonicalized_document, expected_output,
                "Failed: test{:03}",
                i
            );
        }
    }
}
