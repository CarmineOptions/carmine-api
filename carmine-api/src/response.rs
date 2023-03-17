use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct AllNonExpired {
    pub status: String,
    pub data: Vec<String>,
}
