import {MachineBlock} from './components';

import useSWR from "swr";

export default function MachinesPage() {
  const { data, error, isLoading } = useSWR(`/api/machines/latest`)

  if (error) return <div className="bg-white shadow rounded p-2 mt-3">
    <p>未找到</p>
  </div>
  if (isLoading) return <div className="bg-white shadow rounded p-2 mt-3">
    <p>加载中</p>
  </div>

  return (
    <div className='mt-3 gap-2 grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5 2xl:grid-cols-6'>
      {data.map((item)=>{
        return <MachineBlock item={item} key={item.id} />
      })}
    </div>
  )
}