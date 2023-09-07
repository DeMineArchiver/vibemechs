import { Component, createEffect, createSignal, lazy, onMount } from "solid-js";

import styles from "./App.module.css";
import { appWindow } from "@tauri-apps/api/window";
import { invoke } from "@tauri-apps/api";
import ToggleButton from "../lib/components/toggle-button/ToggleButton";
import HomePage from "./pages/home/Page";

const App: Component = () => {
  onMount(() => {
    appWindow.show();
  });

  return (
    <div id={styles["app"]}>
      <div id={styles["wrapper"]}>
        <nav>
          <ToggleButton>
            <span>HOME</span>
            <span>PACKS</span>
            <span>EDITOR</span>
          </ToggleButton>
        </nav>
        <HomePage />
      </div>
    </div>
  )
}

export default App;
