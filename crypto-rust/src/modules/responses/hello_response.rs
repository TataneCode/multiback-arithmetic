use serde::Serialize;

#[derive(Serialize)]
pub struct HelloResponse {
    pub message: String,
    pub status: u16,
    pub data: Option<String>,
}