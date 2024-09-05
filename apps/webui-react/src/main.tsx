import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { ResizeObserver } from "@juggle/resize-observer";

if (!window.ResizeObserver) {
  window.ResizeObserver = ResizeObserver;
}

const mainElementId = "root";
const container = document.getElementById(mainElementId);


if (!container) {
  throw new Error(
    `No container '${mainElementId}' found to render application`
  );
}

ReactDOM.createRoot(container).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
