use serde::Serialize;

#[derive(Serialize)]
pub struct AnalyzeResponse {
    pub score: f32,
    pub missing_skills: Vec<String>,
}