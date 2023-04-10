import React from 'react'
import ReactDOM from 'react-dom/client'
import { SWRConfig } from 'swr'

import App from './App'

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <SWRConfig value={{
      fetcher: (resource, init) => fetch(resource, { method: "POST", ...init }).then(res => res.json()),
      refreshInterval: 100000
    }}>
      <App />
    </SWRConfig>
  </React.StrictMode>,
)
