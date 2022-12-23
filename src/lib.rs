use base16ct::lower::encode_str;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
mod canon;
mod nquads;
mod rdf;

#[cfg(test)]
mod tests {
    use super::*;
    use canon::{issue_identifier, IdentifierIssuer};

    #[test]
    fn test_hash() {
        let hash = Sha256::digest(b"Hello world!");
        const HASH_LEN: usize = 32;
        const HASH_BUF_LEN: usize = HASH_LEN * 2;
        let mut buf = [0u8; HASH_BUF_LEN];
        let hex_hash = encode_str(&hash, &mut buf).unwrap();
        assert_eq!(
            hex_hash,
            "c0535e4be2b79ffd93291305436bf889314e4a3faec05ecffcbb7df31ad9e51a"
        );
    }
}
