import { useState } from 'react'
import { Link, Router, Route, useLocation } from 'wouter'

import './app.css'

import IntroPage from './intro'
import DetailPage from './detail'
import NewPage from './new'
import TasksPage from './tasks'

function App() {
  const [token, setToken] = useState("")
 
  return (
    <div className='max-w-2xl  w-full mx-auto'>
      <header className='bg-white shadow p-2 rounded-sm flex lg:my-2'>
        <h1 className='font-bold underline text-lg'><Link href="/">Bench.im</Link></h1>
      </header>
      <main>
        <Router>
          <Route path="/">
            <IntroPage token={token} setToken={setToken} />
          </Route>
          <Route path="/t/:token/servers/new" component={NewPage} />
          <Route path="/t/:token/servers/:server/tasks" component={TasksPage} />
          <Route path="/t/:token" component={DetailPage} />
        </Router>
      </main>
    </div>
  )
}

export default App
