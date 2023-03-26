import { Link } from "wouter"
import {ServersBlock, MachinesBlock} from "./components"

export default function DetailPage({ params }) {
  return (
    <>
      <aside className="flex p-1 my-2 items-center">
        <div className="text-xl">地址：<Link className="underline" href={`/t/${params.token}`}>{params.token}</Link></div>
      </aside>
      <ServersBlock token={params.token} />
      <MachinesBlock token={params.token} />
    </>
  )
}