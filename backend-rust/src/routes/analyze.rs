use axum::{Json};
use crate::models::request::AnalyzeRequest;
use crate::models::response::AnalyzeResponse;

pub async fn analyze_resume(
    Json(payload): Json<AnalyzeRequest>,
) -> Json<AnalyzeResponse> {

    let response = AnalyzeResponse {
        score: 78.0,
        missing_skills: vec![
            "Docker".to_string(),
            "AWS".to_string(),
        ],
    };

    Json(response)
}