import vue from '@vitejs/plugin-vue';
import { defineConfig } from 'vite';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  clearScreen: false,
  envPrefix: ['VITE_', 'TAURI_'],
  server: {
    port: 5173,
    strictPort: true,
  },
  build: {
    outDir: './dist',
    // See https://tauri.app/v1/references/webview-versions for details
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !!!process.env.TAURI_DEBUG,
    sourcemap: !!process.env.TAURI_DEBUG,
    emptyOutDir: true,
  },
});
