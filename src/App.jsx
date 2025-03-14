import { useState } from 'react'
import { Link, Switch, Route, useLocation } from 'wouter'

import './app.css'

import { MachinesBlock } from './components'

import IndexPage from './pages/index_page'
import MachinePage from './pages/machine_page'

import Admin from "./admin";

function App() {
  const [isLogin, setIsLogin] = useState(false);
  const [isShow, setIsShow] = useState(false);
  const [location, setLocation] = useLocation();
  const token = sessionStorage.getItem('token');

  if (!isLogin && token) {
    setIsLogin(true);
  }

  return (
    <div className='flex flex-col sm:flex-row'>
      <header className='w-full shrink-0 z-50 sticky top-0 sm:h-screen sm:border-r sm:border-neutral-500 sm:w-56'>
        <div className='flex bg-neutral-800 px-2 py-1.5'>
          <h1 className='font-bold text-2xl text-white'><Link href="/">Bench.im</Link></h1>
          <button className='ml-auto bg-white px-1 sm:hidden' onClick={() => setIsShow(!isShow)}>
            <svg className='w-6 h-6' fill='none' stroke='currentColor' viewBox='0 0 24 24' xmlns='http://www.w3.org/2000/svg'>
              <path strokeWidth='2' d='M4 6h16M4 12h16M4 18h16'></path>
            </svg>
          </button>
        </div>
        <div className='relative sm:static'>
          <nav className={'top-0 left-0 w-full absolute flex-col bg-white sm:flex sm:static' + (isShow ? ' flex' : ' hidden')}>
            <button className='px-4 py-2 text-left border-b border-neutral-500 hover:bg-neutral-200' onClick={() => { setLocation(`/`); setIsShow(false) }}>首页</button>
            <button className='px-4 py-2 text-left border-b border-neutral-500 hover:bg-neutral-200' onClick={() => { setLocation(isLogin ? "/admin/" : "/admin/login/"); setIsShow(false) }}>管理</button>
            <MachinesBlock setIsShow={setIsShow} setLocation={setLocation} />
          </nav>
        </div>
      </header>
      <main className='w-full'>
        <Switch>
          <Route path="/" component={IndexPage} />
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
