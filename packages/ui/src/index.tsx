import { listen } from "@tauri-apps/api/event";
import { render } from "solid-js/web";
import App from "./app/App";

import "./index.css";

render(() => <App />, document.getElementById("root")!);
