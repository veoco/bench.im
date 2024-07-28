import { useLocation } from "wouter"

export default function MachineBlock({ item }) {
  const [location, setLocation] = useLocation();

  return (
    <div className='flex flex-col cursor-pointer bg-white border hover:bg-neutral-100' onClick={() => setLocation(`/m/${item.id}`)}>
      <h3 className="px-2 py-1 font-bold">{item.nickname}</h3>
      <p className="border-t p-2 bg-neutral-100 text-xs">{item.ip}</p>
    </div>
  )
}