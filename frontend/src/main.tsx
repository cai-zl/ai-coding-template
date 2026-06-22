import React from "react";
import { createRoot } from "react-dom/client";

import "./style.css";

function App() {
  return (
    <main className="app-shell">
      <h1>Agents App</h1>
      <p>Minimal frontend scaffold for build verification.</p>
    </main>
  );
}

createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
