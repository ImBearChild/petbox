use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot unshare namespace: `{0}`")]
    UnshareFailed(unshare_petbox::Error),
    #[error("command exit with bad state: `{0}`")]
    CommandFailed(String),
    #[error("unknown data store error")]
    Unknown,
}