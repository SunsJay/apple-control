/*
 * @Date: 2024-11-01 12:48:18
 * @LastEditors: sunsjay sunsjay0806@gmail.com
 * @LastEditTime: 2024-11-01 14:38:02
 * @FilePath: /apple-control/vite.config.ts
 * @Description: 
 */
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { resolve } from "path";
// @ts-expect-error process is a nodejs global
const host = process.env.TAURI_DEV_HOST;

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],
  css: {
  preprocessorOptions: {
    scss: { api: 'modern-compiler' },
  }
  },
  resolve: {
    alias: {
      // 这里就是需要配置resolve里的别名
      "@": resolve(__dirname,"./src") // path记得引入
    }
  },
  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host
      ? {
          protocol: "ws",
          host,
          port: 1421,
        }
      : undefined,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
}));