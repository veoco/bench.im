import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react({
    babel: {
      plugins: [
        [
          "formatjs",
          {
            "idInterpolationPattern": "[sha512:contenthash:base64:6]",
            "ast": true
          }
        ]
      ]
    }
  })],
  server: {
    proxy: {
      '/api': 'http://127.0.0.1:8000'
    }
  }
})
