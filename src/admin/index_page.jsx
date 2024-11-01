import useSWR from "swr";
import { useLocation } from "wouter";

import { MachinesBlock, TargetsBlock } from './components';
import fetchWithAuth from './utils';

export default function IndexPage() {
  const [location, setLocation] = useLocation();

  const { data, error, isLoading } = useSWR(`/api/admin/machines/`, fetchWithAuth)

  if (error) return <div>
    <p>未找到</p>
  </div>
  if (isLoading) return <div>
    <p>加载中</p>
  </div>

  return (
    <div>
      <div className="flex border-b pb-2 items-center">
        <h2 className="text-lg font-bold mr-auto">机器列表</h2>
        <button
          className="border bg-neutral-100 px-2 py-1"
          onClick={() => setLocation('/machines/new')}
        >新增机器</button>
      </div>
      <div className='w-full my-3'>
        <MachinesBlock />
      </div>
      <div className="flex border-b pb-2 items-center">
        <h2 className="text-lg font-bold mr-auto">目标列表</h2>
        <button
          className="border bg-neutral-100 px-2 py-1"
          onClick={() => setLocation('/targets/new')}
        >新增目标</button>
      </div>
      <div className='w-full my-3'>
        <TargetsBlock />
      </div>
    </div>
  )
}