import { useLocation } from "wouter"

export default function TargetBlock({ item }) {
  const [location, setLocation] = useLocation();

  return (
    <div className='flex flex-col bg-white border border-neutral-400 justify-between'>
      <h3 className="p-2 flex flex-col">
        <span className="font-bold text-sm truncate">{item.name}</span>
        <span className="text-neutral-500 text-xs truncate">{item.ipv4}</span>
        <span className="text-neutral-500 text-xs truncate">{item.ipv6}</span>
      </h3>
      <ul className="flex border-t border-neutral-400 p-2 bg-neutral-100 text-xs items-center">
        <li className="mr-auto truncate">{item.domain}</li>
        <li className="ml-1 shrink-0"><button className="border shadow-sm bg-sky-400 border-sky-500 text-white px-1 py-0.5 mr-1" onClick={() => setLocation(`/targets/${item.id}`)}>编辑</button></li>
        <li className="shrink-0"><button className="border shadow-sm bg-red-400 border-red-500 text-white px-1 py-0.5" onClick={() => setLocation(`/targets/${item.id}/delete`)}>删除</button></li>
      </ul>
    </div>
  )
}