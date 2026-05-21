from fastapi import APIRouter
from pydantic import BaseModel

router = APIRouter()

class AnalyzeRequest(BaseModel):
    resume_text: str
    job_description: str

@router.post("/analyze")
def analyze(data: AnalyzeRequest):

    return {
        "score": 82,
        "missing_skills": [
            "Kubernetes",
            "CI/CD"
        ]
    }