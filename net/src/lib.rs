//! Networking crate placeholder.

use url::Url;

pub fn fetch(_url: &Url) -> Result<(), NetError> {
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum NetError {
    #[error("generic network error")]
    Generic,
}

