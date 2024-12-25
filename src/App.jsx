import { useState } from 'react'
import { Link, Switch, Route } from 'wouter'

import './app.css'

import IndexPage from './pages/index_page'
import MachinesPage from './pages/machines_page'
import MachinePage from './pages/machine_page'

import Admin from "./admin";

function App() {
  const [isLogin, setIsLogin] = useState(false);
  const token = sessionStorage.getItem('token');

  if (!isLogin && token) {
    setIsLogin(true);
  }

  return (
    <div className='flex flex-col sm:flex-row'>
      <header className='w-full flex-shrink-0 sm:h-screen sm:border-r sm:border-neutral-500 sm:w-56 sm:sticky sm:top-0'>
        <h1 className='font-bold text-2xl text-white bg-neutral-800 px-2 py-1.5'><Link href="/">Bench.im</Link></h1>
        <nav className='flex flex-col bg-white'>
          <Link className='px-4 py-2 border-b border-neutral-500 hover:bg-neutral-200' href="/">首页</Link>
          <Link className='px-4 py-2 border-b border-neutral-500 hover:bg-neutral-200' href="/m/">机器</Link>
          <Link className='px-4 py-2 border-b border-neutral-500 hover:bg-neutral-200' href={isLogin ? "/admin/" : "/admin/login/"}>管理</Link>
        </nav>
      </header>
      <main className='w-full'>
        <Switch>
          <Route path="/" component={IndexPage} />
          <Route path="/m/" component={MachinesPage} />
          <Route path="/m/:mid" component={MachinePage} />
          <Route path="/admin" nest>
            <Admin isLogin={isLogin} setIsLogin={setIsLogin} />
          </Route>
        </Switch>
      </main>
    </div>
  )
}

export default App
