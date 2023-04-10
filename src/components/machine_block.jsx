import { Link } from "wouter"

export default function MachineBlock({item}) {
  return (
    <div className='flex flex-col bg-white shadow rounded p-3'>
      <h3 className="font-bold underline"><Link href={`/m/${item.id}`}>{item.name}</Link></h3>
      <p>{item.ip}</p>
    </div>
  )
}