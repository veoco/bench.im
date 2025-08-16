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
        <div className='flex bg-neutral-800 px-2 py-1.5 items-center'>
          <h1 className='font-bold text-2xl text-white'><Link href="/">Bench.im</Link></h1>
          <button className='ml-auto w-6' onClick={() => { setLocation(isLogin ? "/admin/" : "/admin/login/"); setIsShow(false) }}>
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="rgba(255,255,255,1)"><path d="M12 14V22H4C4 17.5817 7.58172 14 12 14ZM12 13C8.685 13 6 10.315 6 7C6 3.685 8.685 1 12 1C15.315 1 18 3.685 18 7C18 10.315 15.315 13 12 13ZM14.5946 18.8115C14.5327 18.5511 14.5 18.2794 14.5 18C14.5 17.7207 14.5327 17.449 14.5945 17.1886L13.6029 16.6161L14.6029 14.884L15.5952 15.4569C15.9883 15.0851 16.4676 14.8034 17 14.6449V13.5H19V14.6449C19.5324 14.8034 20.0116 15.0851 20.4047 15.4569L21.3971 14.8839L22.3972 16.616L21.4055 17.1885C21.4673 17.449 21.5 17.7207 21.5 18C21.5 18.2793 21.4673 18.551 21.4055 18.8114L22.3972 19.3839L21.3972 21.116L20.4048 20.543C20.0117 20.9149 19.5325 21.1966 19.0001 21.355V22.5H17.0001V21.3551C16.4677 21.1967 15.9884 20.915 15.5953 20.5431L14.603 21.1161L13.6029 19.384L14.5946 18.8115ZM18 17C17.4477 17 17 17.4477 17 18C17 18.5523 17.4477 19 18 19C18.5523 19 19 18.5523 19 18C19 17.4477 18.5523 17 18 17Z"></path></svg>
          </button>
          <button className='ml-3 bg-white px-1 sm:hidden' onClick={() => setIsShow(!isShow)}>
            <svg className='w-4 h-6' fill='none' stroke='currentColor' viewBox='0 0 24 24' xmlns='http://www.w3.org/2000/svg'>
              <path strokeWidth='2' d='M4 6h16M4 12h16M4 18h16'></path>
            </svg>
          </button>
        </div>
        <div className='relative sm:static'>
          <nav className={'top-0 left-0 w-full absolute flex-col bg-white sm:flex sm:static' + (isShow ? ' flex' : ' hidden')}>
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
