use std::fmt;

#[derive(Debug)]
pub enum BlockChainDemoError {
    UsageError,
    InternalError,
    GenesisError,
}
impl std::fmt::Display for BlockChainDemoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlockChainDemoError::UsageError => write!(f, "Usage error"),
            BlockChainDemoError::InternalError => write!(f, "Internal unexpected error"),
            BlockChainDemoError::GenesisError => write!(f, "Genesis block is invalid or missing"),
        }
    }
}
impl std::error::Error for BlockChainDemoError {}
