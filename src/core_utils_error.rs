#[derive(Debug)]
pub enum CoreUtilsError {
    ConversionError(String),
    CannotRetrieveValue,
    IncompatibleType,
    InvalidOperation,
    Other(String),
}