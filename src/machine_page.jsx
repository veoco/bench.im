import { useState } from "react";
import { TcpPingBlock } from './components';

import useSWR from "swr";

export default function MachinePage({ params }) {
  const [fixedY, setFixedY] = useState(false);
  const [dateRange, setDateRange] = useState("latest")
  const { data, error, isLoading } = useSWR(`/api/machines/${params.mid}/`)

  if (error) return <div className="bg-white shadow rounded p-2 mt-3">
    <p>未找到</p>
  </div>
  if (isLoading) return <div className="bg-white shadow rounded p-2 mt-3">
    <p>加载中</p>
  </div>

  const ipv4s = data.targets.filter(obj => obj.ipv6 === false);
  ipv4s.sort((a, b) => a.name.localeCompare(b.name));
  const ipv6s = data.targets.filter(obj => obj.ipv6 === true);
  ipv6s.sort((a, b) => a.name.localeCompare(b.name));

  return (
    <div className='mt-2'>
      <div className="flex items-baseline">
        <h2 className='font-bold text-2xl mr-2'>{data.detail.nickname}</h2>
        <p className="text-gray-500 text-sm">{data.detail.ip}</p>
      </div>
      <div className="flex mt-2 text-sm">
        <button className={`border border-gray-600 shadow px-2 py-1 mr-2` + (fixedY ? ` bg-gray-500 text-white` : ` bg-white`)} type="button" onClick={() => setFixedY(!fixedY)}>对齐y轴</button>
        <select className="pl-2 py-1" value={dateRange} onChange={(e) => setDateRange(e.target.value)}>
          <option value="latest">最近 24 小时</option>
          <option value="7d">最近 7 天</option>
        </select>
      </div>
      <h3 className="mt-2 p-2 border-b font-bold border-gray-600">IPv4</h3>
      <div className='mt-3 gap-2 grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3'>
        {ipv4s.map((item) => {
          return (
            <div className='bg-white border shadow rounded p-2' key={item.id}>
              <h3 className='font-bold text-lg'>{item.name}</h3>
              <TcpPingBlock mid={params.mid} tid={item.id} fixedY={fixedY} dateRange={dateRange} />
            </div>
          )
        })}
      </div>
      <h3 className="mt-2 p-2 border-b font-bold border-gray-600">IPv6</h3>
      <div className='mt-3 gap-2 grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3'>
        {ipv6s.map((item) => {
          return (
            <div className='bg-white border shadow rounded p-2' key={item.id}>
              <h3 className='font-bold text-lg'>{item.name}</h3>
              <TcpPingBlock mid={params.mid} tid={item.id} fixedY={fixedY} dateRange={dateRange} />
            </div>
          )
        })}
      </div>
    </div>

  )
}