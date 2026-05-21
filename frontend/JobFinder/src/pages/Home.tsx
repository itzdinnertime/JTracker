import { useState } from "react";
import { analyzeResume } from "../services/api";

export default function Home() {
  const [resume, setResume] = useState("");
  const [jobDescription, setJobDescription] = useState("");
  const [result, setResult] = useState<any>(null);

  async function handleSubmit() {
    const data = await analyzeResume(resume, jobDescription);
    setResult(data);
  }

  return (
    <div>
      <h1>AI Career Platform</h1>

      <textarea
        placeholder="Paste resume"
        onChange={(e) => setResume(e.target.value)}
      />

      <textarea
        placeholder="Paste job description"
        onChange={(e) => setJobDescription(e.target.value)}
      />

      <button onClick={handleSubmit}>
        Analyze
      </button>

      {result && (
        <div>
          <h2>ATS Score: {result.score}</h2>
        </div>
      )}
    </div>
  );
}