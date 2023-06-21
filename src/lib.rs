pub mod canon;
pub mod error;
#[cfg(feature = "log")]
pub mod logger;
pub use crate::canon::{canonicalize, serialize};
pub use crate::error::CanonicalizationError;
#[cfg(feature = "log")]
pub use crate::logger::YamlLayer;

#[cfg(test)]
mod tests {
    use crate::canon::{canonicalize, serialize};
    use oxrdf::Dataset;
    use oxttl::NQuadsParser;
    use std::{
        fs::File,
        io::{BufReader, Read},
        path::Path,
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

    #[test]
    fn test_canonicalize() {
        #[cfg(feature = "log")]
        init_logger(tracing::Level::INFO);
        // init_logger(tracing::Level::DEBUG);

        const BASE_PATH: &str = "tests/rdfc10";

        fn read_nquads_as_string(path: &str) -> Option<String> {
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

        let range = 1..=73;
        for i in range {
            let input_path = format!("{BASE_PATH}/test{:03}-in.nq", i);
            let Ok(input_file) = File::open(input_path) else {
                println!("test{:03} not found", i);
                continue;
            };
            let input_quads = NQuadsParser::new()
                .parse_from_read(BufReader::new(input_file))
                .into_iter()
                .map(|x| x.unwrap());
            let input_dataset = Dataset::from_iter(input_quads);

            let normalized_dataset = canonicalize(&input_dataset).unwrap();
            let canonicalized_document = serialize(normalized_dataset);

            let output_path = format!("{BASE_PATH}/test{:03}-rdfc10.nq", i);
            let expected_output = match read_nquads_as_string(&output_path) {
                Some(s) => s,
                None => continue,
            };

            assert_eq!(
                canonicalized_document, expected_output,
                "test{:03} failed",
                i
            );

            println!("test{:03} passed", i);
        }
    }
}
