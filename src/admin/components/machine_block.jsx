import { useLocation } from "wouter"

export default function MachineBlock({ item }) {
  const [location, setLocation] = useLocation();

  return (
    <div className='flex flex-col bg-white border justify-center'>
      <h3 className="p-2 flex flex-col">
        <span className="font-bold text-sm">{item.nickname}</span>
        <span className="text-neutral-500 text-xs">{item.name}</span>
      </h3>
      <ul className="flex border-t p-2 bg-neutral-100 text-xs items-center">
        <li className="mr-auto">{item.ip}</li>
        <li><button className="border bg-sky-400 border-sky-500 text-white px-1 py-0.5 mr-1" onClick={() => setLocation(`/machines/${item.id}`)}>编辑</button></li>
        <li><button className="border bg-red-400 border-red-500 text-white px-1 py-0.5" onClick={() => setLocation(`/machines/${item.id}/delete`)}>删除</button></li>
      </ul>
    </div>
  )
}