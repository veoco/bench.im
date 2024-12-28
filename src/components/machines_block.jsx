import useSWR from "swr";

export default function MachinesBlock({ setIsShow, setLocation }) {
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
        return <button className='px-4 py-2 text-left border-b border-neutral-500 hover:bg-neutral-200' onClick={() => { setLocation(`/m/${item.id}`); setIsShow(false) }} key={item.id}>{item.nickname}</button>
      })}
    </>
  )
}