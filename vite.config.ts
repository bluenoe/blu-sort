import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  root: "src",
  build: {
    outDir: "../dist",
    emptyOutDir: true, // fix warning outDir not inside root
    rollupOptions: {
      external: [], // không cần externalize react/jsx-runtime
    },
  },
  plugins: [react()],
});
