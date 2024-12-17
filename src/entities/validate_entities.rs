use serde::{Serialize};


#[derive(Serialize)]
pub struct ErrorValidation {
    pub status: i32,
    pub error : bool,
    pub validate: Vec<FieldError>,
    pub message: String,
}
#[derive(Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}