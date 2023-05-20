import { useLocation } from "wouter"

export default function MachineBlock({ item }) {
  const [location, setLocation] = useLocation();

  return (
    <div className='flex flex-col cursor-pointer bg-white shadow rounded p-3' onClick={() => setLocation(`/m/${item.id}`)}>
      <h3 className="font-bold mb-1">{item.nickname}</h3>
      <p className="text-xs text-gray-500">{item.ip}</p>
    </div>
  )
}