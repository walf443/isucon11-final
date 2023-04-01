#[derive(Debug, serde::Serialize)]
pub struct GetMeResponse {
    pub code: String,
    pub name: String,
    pub is_admin: bool,
}

