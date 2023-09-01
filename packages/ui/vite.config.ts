import { defineConfig } from "vite";
import solid from "vite-plugin-solid";

export default defineConfig({
  server: {
    port: 5181,
    strictPort: true
  },
  clearScreen: false,
  envPrefix: [
    "VITE_",
    "TAURI_"
  ],
  plugins: [solid()],
});
