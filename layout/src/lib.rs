//! Basic layout crate placeholder.

use euclid::default::Rect;

pub fn layout_document(_viewport: Rect<f32>) -> Result<(), LayoutError> {
    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum LayoutError {
    #[error("generic layout error")]
    Generic,
}

