import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from "@tailwindcss/vite";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  server: {
    host: "127.0.0.1",
    proxy: {
      // '/api': 'http://127.0.0.1:3000',
      '/api': {
        target: 'https://bench.im',
        changeOrigin: true,
      },
      '/content.html': {
        target: 'https://bench.im',
        changeOrigin: true,
      },
    }
  }
})
