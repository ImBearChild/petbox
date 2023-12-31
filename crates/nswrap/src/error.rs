use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unshare failed: `{0}`")]
    UnshareFailed(nix::errno::Errno),
    #[error("Clone failed: `{0}`")]
    CloneFailed(nix::errno::Errno),
    #[error("Unix API lib failed: `{0}`")]
    OsErrno(i32),
    #[error("unknown data store error")]
    Unknown,
}