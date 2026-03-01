#[derive(Debug)]
pub enum CoreUtilsError {
    ConversionError(String),
    Other(String),
}