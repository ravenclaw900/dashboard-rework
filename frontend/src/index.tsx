/* @refresh reload */
import { render } from "solid-js/web";

import "virtual:uno.css";
import "@unocss/reset/tailwind.css";

import "vite/modulepreload-polyfill";

import App from "./App";

const root = document.getElementById("root");

render(() => <App />, root!);
