import useSWR from "swr";

import fetchWithAuth from '../utils';
import MachineBlock from './machine_block';

export default function MachinesBlock() {
  const { data, error, isLoading } = useSWR(`/api/admin/machines/`, fetchWithAuth)

  if (error) return (
    <div>
      <p>未找到</p>
    </div>
  )
  if (isLoading) return (
    <div>
      <p>加载中</p>
    </div>
  )

  return (
    <div className='gap-2 grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4'>
      {data.map((item) => {
        return <MachineBlock item={item} key={item.id} />
      })}
    </div>
  )
}