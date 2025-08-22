import useSWR from "swr";

import LightBlock from "./light_block";

export default function MachinesBlock({ setIsShow, setLocation }) {
  const { data, error, isLoading } = useSWR(`/api/machines/`)

  if (error) return <div className="px-4 py-2">
    <p>未找到</p>
  </div>
  if (isLoading) return <div className="px-4 py-2">
    <p>加载中</p>
  </div>

  return (
    <div className="m-2 flex flex-col bg-white border border-neutral-500">
      {data.map((item) => {
        return (
          <button className='p-2 flex border-b border-neutral-500 hover:bg-neutral-200 last:border-0' onClick={() => { setLocation(`/m/${item.id}`); setIsShow(false) }} key={item.id}>
            <svg className="w-4 mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M4 3H20C20.5523 3 21 3.44772 21 4V11H3V4C3 3.44772 3.44772 3 4 3ZM3 13H21V20C21 20.5523 20.5523 21 20 21H4C3.44772 21 3 20.5523 3 20V13ZM7 16V18H10V16H7ZM7 6V8H10V6H7Z"></path></svg>
            {item.name}
            <LightBlock updated={item.updated} />
          </button>)
      })}
    </div>
  )
}