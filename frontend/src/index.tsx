/* @refresh reload */
import { render } from "solid-js/web";

import "virtual:uno.css";
import "@unocss/reset/tailwind.css";

import { Router, hashIntegration } from "@solidjs/router";
import App from "./App";

const root = document.getElementById("root");

render(
    () => (
        <Router source={hashIntegration()}>
            <App />
        </Router>
    ),
    root!,
);
