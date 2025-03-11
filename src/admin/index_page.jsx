import useSWR from "swr";
import { useLocation } from "wouter";

import { MachinesBlock, TargetsBlock } from './components';
import fetchWithAuth from './utils';

export default function IndexPage() {
  const [location, setLocation] = useLocation();

  const { data, error, isLoading } = useSWR(`/api/admin/machines/`, fetchWithAuth)

  if (error) return <div className="p-2">
    <p>未找到</p>
  </div>
  if (isLoading) return <div className="p-2">
    <p>加载中</p>
  </div>

  return (
    <div>
      <div className="flex border-b border-neutral-400 bg-neutral-100 p-2 items-center">
        <h2 className="text-lg font-bold mr-3">机器列表</h2>
        <button
          className="text-sm shadow-sm border border-neutral-600 bg-white px-2 py-1"
          onClick={() => setLocation('/machines/new')}
        >新增机器</button>
      </div>
      <div className='w-full p-2'>
        <MachinesBlock />
      </div>
      <div className="flex border-y border-neutral-400 bg-neutral-100 p-2 items-center">
        <h2 className="text-lg font-bold mr-3">目标列表</h2>
        <button
          className="text-sm shadow-sm border border-neutral-600 bg-white px-2 py-1"
          onClick={() => setLocation('/targets/new')}
        >新增目标</button>
      </div>
      <div className='w-full p-2'>
        <TargetsBlock />
      </div>
    </div>
  )
}