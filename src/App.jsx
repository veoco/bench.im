import { useState } from 'react'
import { Link, Router, Route, useLocation } from 'wouter'

import './app.css'

import IntroPage from './intro'
import DetailPage from './detail'
import NewPage from './new'
import TasksPage from './tasks'

function App() {
  const [token, setToken] = useState("")
  const [location, setLocation] = useLocation();

  const handleSubmit = (e) => {
    e.preventDefault();
    setLocation(`/t/${token}`);
  }

  return (
    <div className='max-w-2xl  w-full mx-auto'>
      <header className='bg-white shadow p-2 rounded-sm flex lg:my-2'>
        <h1 className='font-bold underline text-lg'><Link href="/">Bench.im</Link></h1>
        <form className='ml-auto' onSubmit={handleSubmit}>
          <input className='px-1 py-0.5 text-xs shadow' type="text" value={token} onChange={(e) => setToken(e.target.value)} />
        </form>
      </header>
      <main>
        <Router>
          <Route path="/" component={IntroPage} />
          <Route path="/t/:token/servers/new" component={NewPage} />
          <Route path="/t/:token/servers/:server/tasks" component={TasksPage} />
          <Route path="/t/:token" component={DetailPage} />
        </Router>
      </main>
    </div>
  )
}

export default App
