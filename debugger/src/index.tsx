import "@unocss/reset/tailwind.css";
import { render } from "solid-js/web";
import "uno.css";
import App from "./App";
import "./index.css";

render(() => <App />, document.getElementById("root") as HTMLElement);
