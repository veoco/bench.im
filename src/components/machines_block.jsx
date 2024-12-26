import useSWR from "swr";
import { Link } from 'wouter'

export default function MachinesBlock() {
  const { data, error, isLoading } = useSWR(`/api/machines/`)

  if (error) return <div className="px-4 py-2">
    <p>未找到</p>
  </div>
  if (isLoading) return <div className="px-4 py-2">
    <p>加载中</p>
  </div>

  return (
    <>
      {data.map((item) => {
        return <Link className='px-4 py-2 border-b border-neutral-500 hover:bg-neutral-200' href={`/m/${item.id}`} key={item.id}>{item.nickname}</Link>
      })}
    </>
  )
}