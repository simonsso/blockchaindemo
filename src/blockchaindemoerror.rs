use std::fmt;

/// Errors originating in blockchaindemo application
/// errors originating in libraries are propagated unmodified
#[derive(Debug)]
pub enum BlockChainDemoError {
    /// Error caused by abuse to user interface
    UsageError,
    /// Should not be possible to reach end user, generated by the code when a function returns an Option but is already checked elsewhere
    InternalError,
    /// Initial block in chain is corrupt
    GenesisError,
    /// Verification failed, return to user if the chain have been compromised and verify() fails
    VerifyError,
}
impl std::fmt::Display for BlockChainDemoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlockChainDemoError::UsageError => write!(f, "Usage error"),
            BlockChainDemoError::InternalError => write!(f, "Internal unexpected error"),
            BlockChainDemoError::GenesisError => write!(f, "Genesis block is invalid or missing"),
            BlockChainDemoError::VerifyError => write!(f, "Block chain verification failed"),
        }
    }
}
impl std::error::Error for BlockChainDemoError {}
