import { useLocation } from "wouter"

export default function MachineBlock({ item }) {
  const [location, setLocation] = useLocation();

  return (
    <div className='flex flex-col bg-white border border-neutral-400 justify-between'>
      <h3 className="p-2 flex flex-col">
        <span className="font-bold text-sm truncate">{item.nickname}</span>
        <span className="text-neutral-500 text-xs truncate">{item.ip}</span>
      </h3>
      <ul className="flex border-t border-neutral-400 p-2 bg-neutral-100 text-xs items-center">
        <li className="mr-auto truncate">{item.name}</li>
        <li className="ml-1 shrink-0"><button className="border shadow-sm bg-sky-400 border-sky-500 text-white px-1 py-0.5 mr-1" onClick={() => setLocation(`/machines/${item.id}`)}>编辑</button></li>
        <li className="shrink-0"><button className="border shadow-sm bg-red-400 border-red-500 text-white px-1 py-0.5" onClick={() => setLocation(`/machines/${item.id}/delete`)}>删除</button></li>
      </ul>
    </div>
  )
}