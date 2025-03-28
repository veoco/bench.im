import { useState } from "react";
import useSWR from "swr";

import { PingBlock } from '../components';

export default function MachinePage({ params }) {
  const [fixedY, setFixedY] = useState(false);
  const [ipv6, setIpv6] = useState(false);
  const [dateRange, setDateRange] = useState("24h")
  const { data, error, isLoading } = useSWR(`/api/machines/${params.mid}`)

  if (error) return <div className="p-2">
    <p>未找到</p>
  </div>
  if (isLoading) return <div className="p-2">
    <p>加载中</p>
  </div>

  return (
    <div>
      <div className="flex items-baseline p-2">
        <h2 className='font-bold text-2xl mr-2'>{data.nickname}</h2>
        <p className="text-neutral-500 text-sm">{data.ip}</p>
      </div>
      <div className="flex p-2 bg-neutral-100 leading-4 text-sm border-y border-neutral-400">
        <button className={`border border-neutral-600 shadow-sm px-2 py-0.5 mr-2` + (fixedY ? ` bg-neutral-500 text-white` : ` bg-white`)} type="button" onClick={() => setFixedY(!fixedY)}>对齐y轴</button>
        <button className={`border border-neutral-600 shadow-sm px-2 py-0.5 mr-2` + (ipv6 ? ` bg-neutral-500 text-white` : ` bg-white`)} type="button" onClick={() => setIpv6(!ipv6)}>IPv6</button>
        <select className="pl-2 py-0 shadow-sm" value={dateRange} onChange={(e) => setDateRange(e.target.value)}>
          <option value="24h">最近 24 小时</option>
          <option value="7d">最近 7 天</option>
        </select>
      </div>

      <div className='p-2 gap-2 grid grid-cols-1 lg:grid-cols-2 xl:grid-cols-3'>
        {data.targets.map((item) => {
          return (
            <div className='bg-white border border-neutral-400' key={item.id}>
              <h3 className='px-2 py-1 bg-neutral-100 border-b border-neutral-400 font-bold'>{item.name}{ipv6 ? " - IPv6" : ""}</h3>
              <PingBlock mid={params.mid} tid={item.id} fixedY={fixedY} dateRange={dateRange} ipv6={ipv6} />
            </div>
          )
        })}
      </div>
    </div>

  )
}