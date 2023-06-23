use oxrdf::BlankNodeIdParseError;
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum CanonicalizationError {
    #[error("Base16 encoding failed.")]
    Base16EncodingFailed(base16ct::Error),
    #[error("Reference blank node identifier does not exist in the canonicalization state.")]
    QuadsNotExist,
    #[error("Canonical identifier does not exist for the given blank node.")]
    CanonicalIdentifierNotExist,
    #[error("Parsing blank node identifier failed.")]
    BlankNodeIdParseError,
    #[error("The number of calls to the Hash N-degree Quads algorithm have exceeded the limit of {0}.")]
    HndqCallLimitExceeded(usize),
}

impl From<BlankNodeIdParseError> for CanonicalizationError {
    fn from(_: BlankNodeIdParseError) -> Self {
        Self::BlankNodeIdParseError
    }
}
