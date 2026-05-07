use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PersonaSignalError {
    #[error("archive validation failed")]
    ArchiveValidation,
    #[error("archive deserialize failed")]
    ArchiveDeserialize,
    #[error("frame is shorter than length prefix")]
    ShortLengthPrefix,
    #[error("frame length mismatch: expected {expected} bytes, found {found} bytes")]
    LengthMismatch { expected: usize, found: usize },
}
