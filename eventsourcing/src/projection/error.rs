#[derive(Debug, thiserror::Error)]
pub enum PersistenceError {
    #[error("optimistic lock error")]
    OptimisticLockError,
    #[error("{0}")]
    ConnectionError(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("{0}")]
    DeserializationError(Box<dyn std::error::Error + Send + Sync + 'static>),
    #[error("{0}")]
    UnknownError(Box<dyn std::error::Error + Send + Sync + 'static>),
}
