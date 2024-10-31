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
    <div className='mx-2 sm:mx-0'>
      <header className='w-full max-w-3xl mx-auto'>
        <h1 className='font-bold text-2xl my-3'><Link href="/">Bench.im</Link></h1>
        <nav className='flex border bg-neutral-100 my-3'>
          <Link className='px-4 py-2 hover:bg-neutral-200' href="/">首页</Link>
          <Link className='px-4 py-2 hover:bg-neutral-200 mr-auto' href="/m/">机器</Link>
          <Link className='px-4 py-2 hover:bg-neutral-200' href={isLogin ? "/admin/" : "/admin/login/"}>管理</Link>
        </nav>
      </header>
      <main className='w-full max-w-3xl mx-auto'>
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
