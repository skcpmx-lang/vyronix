import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./index.css";
import { useAppStore } from "./lib/store";

// Load persisted settings and apply theme before first paint of the shell.
useAppStore.getState().loadSettings();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
