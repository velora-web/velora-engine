//! HTML5 tokenizer/state machine placeholder crate.
//! Provides parsing entry points.

/// Parses the provided HTML input and returns a placeholder result.
pub fn parse_html(_input: &str) -> Result<(), ParseError> {
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum ParseError {
    #[error("generic parse error")]
    Generic,
}

