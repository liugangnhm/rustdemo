use std::io;
use std::io::ErrorKind;
use thiserror::Error;
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum DataStoreError {
    #[error("data store disconnected")]
    Disconnect(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

fn main() {
    println!("{:?}", DataStoreError::Disconnect(io::Error::from(ErrorKind::BrokenPipe)));
    println!("{:?}", DataStoreError::Redaction("1111111111111".into()));
    println!("{:?}", DataStoreError::InvalidHeader { expected: "1".to_string(), found: "2".to_string() });
    println!("{:?}", DataStoreError::Unknown);
}