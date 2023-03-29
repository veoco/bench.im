import { Link } from "wouter"
import {ServersBlock, MachinesBlock, NewMachineBlock, NewServerBlock} from "./components"

export default function DetailPage({ params }) {
  return (
    <>
      <aside className="flex p-1 my-2 items-center">
        <div className="text-xl">地址：<Link className="underline" href={`/t/${params.token}`}>{params.token}</Link></div>
      </aside>
      <ServersBlock token={params.token} />
      <NewServerBlock token={params.token} />
      <MachinesBlock token={params.token} />
      <NewMachineBlock token={params.token} />
    </>
  )
}