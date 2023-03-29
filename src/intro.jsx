import { useLocation } from 'wouter'
import { NewMachineBlock, NewServerBlock } from './components';

export default function IntroPage({ token, setToken }) {
  const [location, setLocation] = useLocation();

  const handleSubmit = (e) => {
    e.preventDefault();
    setLocation(`/t/${token}`);
  }
  return (
    <div className='sm:px-1'>
      <h1 className="text-2xl mt-8 text-center">搜索地址</h1>
      <p className='text-center text-gray-500'>与客户端 token 参数相同</p>
      <form className='my-6' onSubmit={handleSubmit}>
        <input className='px-2 py-1 w-full shadow' type="text" value={token} placeholder={"输入地址"} onChange={(e) => setToken(e.target.value)} />
      </form>
      <NewServerBlock />
      <NewMachineBlock />
    </div>
  )
}