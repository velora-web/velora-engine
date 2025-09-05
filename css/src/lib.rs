//! CSS parser and selector engine placeholder crate.

pub fn parse_css(_input: &str) -> Result<(), CssError> {
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum CssError {
    #[error("generic css error")]
    Generic,
}

