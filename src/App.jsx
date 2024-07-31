import { Link, Router, Route } from 'wouter'

import './app.css'

import IndexPage from './index_page'
import MachinesPage from './machines_page'
import MachinePage from './machine_page'

function App() {

  return (
    <div className='mx-2 sm:mx-0'>
      <header className='w-full max-w-3xl mx-auto'>
        <h1 className='font-bold text-2xl my-3'><Link href="/">Bench.im</Link></h1>
        <nav className='flex border bg-neutral-100 my-3'>
          <Link className='px-4 py-2 hover:bg-neutral-200' href="/">首页</Link>
          <Link className='px-4 py-2 hover:bg-neutral-200' href="/m/">机器</Link>
        </nav>
      </header>
      <main className='w-full max-w-3xl mx-auto'>
        <Router>
          <Route path="/" component={IndexPage} />
          <Route path="/p/:slug" component={IndexPage} />
          <Route path="/m/" component={MachinesPage} />
          <Route path="/m/:mid" component={MachinePage} />
        </Router>
      </main>
    </div>
  )
}

export default App
