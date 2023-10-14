import React from "react";
import ReactDOM from "react-dom/client";
import { AllRoutes } from "./App";
import "./input.css";
import { RouterProvider } from "react-router-dom";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <RouterProvider router={AllRoutes} />
  </React.StrictMode>,
);
