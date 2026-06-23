import React, { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";

import "./style.css";

type HealthResponse = {
  service: string;
  status: string;
};

function App() {
  const [health, setHealth] = useState<HealthResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    let cancelled = false;

    fetch("/api/health")
      .then((response) => {
        if (!response.ok) {
          throw new Error(`API returned ${response.status}`);
        }
        return response.json() as Promise<HealthResponse>;
      })
      .then((data) => {
        if (!cancelled) {
          setHealth(data);
        }
      })
      .catch((caught: unknown) => {
        if (!cancelled) {
          setError(caught instanceof Error ? caught.message : "API request failed");
        }
      });

    return () => {
      cancelled = true;
    };
  }, []);

  return (
    <main className="app-shell">
      <section>
        <p className="eyebrow">Full-stack scaffold</p>
        <h1>AI Coding Template</h1>
        <p className="summary">
          React, Vite, TypeScript, Gin, sqlc-ready PostgreSQL, and Redis-ready
          backend structure.
        </p>
        <dl className="status-list">
          <div>
            <dt>Frontend</dt>
            <dd>ready</dd>
          </div>
          <div>
            <dt>Backend API</dt>
            <dd>{health ? `${health.service}: ${health.status}` : error ?? "checking"}</dd>
          </div>
        </dl>
      </section>
    </main>
  );
}

createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
