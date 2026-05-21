const API_BASE = "http://localhost:3000";

export async function analyzeResume(
  resumeText: string,
  jobDescription: string
) {
  const response = await fetch(`${API_BASE}/analyze`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      resume_text: resumeText,
      job_description: jobDescription,
    }),
  });

  return response.json();
}