use serde::Deserialize;

#[derive(Deserialize)]
pub struct AnalyzeRequest {
    pub resume_text: String,
    pub job_description: String,
}