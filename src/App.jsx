import { Link, Router, Route } from 'wouter'

import './app.css'

import IndexPage from './index_page'
import MachinesPage from './machines_page'
import MachinePage from './machine_page'

function App() {

  return (
    <div>
      <header className='p-2 bg-white shadow rounded-sm flex'>
        <h1 className='font-bold text-lg'><Link href="/">Bench.im</Link></h1>
      </header>
      <main className='w-full mx-auto p-2'>
        <Router>
          <Route path="/" component={IndexPage} />
          <Route path="/m/" component={MachinesPage} />
          <Route path="/m/:mid" component={MachinePage} />
        </Router>
      </main>
    </div>
  )
}

export default App
