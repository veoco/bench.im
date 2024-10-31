import useSWR from "swr";

import { MachineBlock } from '../components';

export default function MachinesPage() {
  const { data, error, isLoading } = useSWR(`/api/machines/`)

  if (error) return <div>
    <p>未找到</p>
  </div>
  if (isLoading) return <div>
    <p>加载中</p>
  </div>

  return (
    <div className='gap-2 grid grid-cols-2 md:grid-cols-3 lg:grid-cols-4'>
      {data.map((item) => {
        return <MachineBlock item={item} key={item.id} />
      })}
    </div>
  )
}