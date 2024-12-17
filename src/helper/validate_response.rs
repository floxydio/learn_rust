use validator::ValidationErrors;
use crate::entities::FieldError;

pub fn format_validation_errors(errors: ValidationErrors) -> Vec<FieldError> {
    let mut field_errors = Vec::new();

    for (field, errors) in errors.field_errors() {
        for error in errors {
            field_errors.push(FieldError {
                field: field.to_string(),
                message: error.message.clone().unwrap_or_default().to_string(),
            });
        }
    }
    field_errors
}