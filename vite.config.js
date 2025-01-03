import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
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
