//! Display list and software raster placeholder crate.

pub fn paint() -> Result<(), PaintError> {
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum PaintError {
    #[error("generic paint error")]
    Generic,
}

