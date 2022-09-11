pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("System call failed")]
    SysError,

    #[error("{0}")]
    IOError(#[from] std::io::Error),
}
