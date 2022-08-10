import React from 'react'
import ReactDOM from 'react-dom/client'
import { BrowserRouter } from "react-router-dom";
import { SWRConfig } from 'swr'
import App from './App'
import './index.css'

const fetcher = async url => {
  const res = await fetch(url)

  if (!res.ok) {
    const error = new Error('An error occurred while fetching the data.')
    error.info = await res.json()
    error.status = res.status
    throw error
  }

  return res.json()
}

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <BrowserRouter>
      <SWRConfig value={{ fetcher: fetcher }}>
        <App />
      </SWRConfig>
    </BrowserRouter>
  </React.StrictMode>
)
