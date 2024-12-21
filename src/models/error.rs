use http::StatusCode;
use serde::ser::{Serialize, Serializer, SerializeStruct};

#[derive(Debug)]
pub struct CommonError {
    pub message: String,
    pub code: StatusCode,
}

#[derive(Debug)]
pub struct RepositoryError {
    pub message: String,
}
impl std::fmt::Display for CommonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}, Code: {}", self.message, self.code)
    }
}

impl Serialize for CommonError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CommonError", 2)?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("code", &self.code.as_u16())?;
        state.end()
    }
}
pub type RepositoryResult<T> = Result<T, RepositoryError>;
pub type ServiceResult<T> = Result<T, CommonError>;