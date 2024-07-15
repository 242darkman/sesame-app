use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub error_code: u16,
}
