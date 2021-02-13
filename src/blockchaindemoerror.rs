use std::fmt;

#[derive(Debug)]
pub enum BlockChainDemoError {
    UsageError,
    InternalError,
}
impl std::fmt::Display for BlockChainDemoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlockChainDemoError::UsageError => write!(f, "Usage error"),
            BlockChainDemoError::InternalError => write!(f, "Internal unexpected error"),
        }
    }
}
impl std::error::Error for BlockChainDemoError {}
