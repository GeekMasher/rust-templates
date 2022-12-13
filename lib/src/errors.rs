
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LibErrors {
    #[error("Unknown error")]
    Unknown
}


