import { useState } from "react";
import { TcpPingBlock } from './components';

import useSWR from "swr";

export default function MachinePage({ params }) {
  const [fixedY, setFixedY] = useState(false);
  const [dateRange, setDateRange] = useState("latest")
  const { data, error, isLoading } = useSWR(`/api/machines/${params.mid}/`)

  if (error) return <div>
    <p>未找到</p>
  </div>
  if (isLoading) return <div>
    <p>加载中</p>
  </div>

  const ipv4s = data.targets.filter(obj => obj.ipv6 === false);
  ipv4s.sort((a, b) => a.name.localeCompare(b.name));
  const ipv6s = data.targets.filter(obj => obj.ipv6 === true);
  ipv6s.sort((a, b) => a.name.localeCompare(b.name));

  return (
    <div>
      <div className="border">
        <div className="flex items-baseline p-2">
          <h2 className='font-bold text-2xl mr-2'>{data.detail.nickname}</h2>
          <p className="text-neutral-500 text-sm">{data.detail.ip}</p>
        </div>
        <div className="flex p-2 bg-neutral-100 leading-4 text-sm border-t">
          <button className={`border border-neutral-600 shadow px-2 py-0.5 mr-2` + (fixedY ? ` bg-neutral-500 text-white` : ` bg-white`)} type="button" onClick={() => setFixedY(!fixedY)}>对齐y轴</button>
          <select className="pl-2 py-0" value={dateRange} onChange={(e) => setDateRange(e.target.value)}>
            <option value="latest">最近 24 小时</option>
            <option value="7d">最近 7 天</option>
          </select>
        </div>
      </div>

      <h3 className="mt-3 font-bold">IPv4</h3>
      <div className='mt-3 gap-2 grid grid-cols-1 lg:grid-cols-2'>
        {ipv4s.map((item) => {
          return (
            <div className='bg-white border' key={item.id}>
              <h3 className='px-2 py-1 bg-neutral-100 border-b font-bold'>{item.name}</h3>
              <TcpPingBlock mid={params.mid} tid={item.id} fixedY={fixedY} dateRange={dateRange} />
            </div>
          )
        })}
      </div>
      <h3 className="mt-3 font-bold">IPv6</h3>
      <div className='mt-3 gap-2 grid grid-cols-1 lg:grid-cols-2'>
        {ipv6s.map((item) => {
          return (
            <div className='bg-white border' key={item.id}>
              <h3 className='px-2 py-1 bg-neutral-100 border-b font-bold'>{item.name}</h3>
              <TcpPingBlock mid={params.mid} tid={item.id} fixedY={fixedY} dateRange={dateRange} />
            </div>
          )
        })}
      </div>
    </div>

  )
}