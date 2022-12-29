use thiserror::Error;

#[derive(Error, Debug)]
pub enum CanonicalizationError {
    #[error("Base16 encoding failed.")]
    Base16EncodingError(base16ct::Error),
    #[error("Reference blank node identifier does not exist in the canonicalization state.")]
    QuadsNotExistError,
    
}
