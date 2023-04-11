import { useState } from "react";
import { TcpPingBlock } from './components';

import useSWR from "swr";

export default function MachinePage({ params }) {
  const [fixedY, setFixedY] = useState(false);
  const { data, error, isLoading } = useSWR(`/api/machines/${params.mid}/`)

  if (error) return <div className="bg-white shadow rounded p-2 mt-3">
    <p>未找到</p>
  </div>
  if (isLoading) return <div className="bg-white shadow rounded p-2 mt-3">
    <p>加载中</p>
  </div>

  return (
    <div className='mt-2'>
      <div className="flex items-baseline">
        <h2 className='font-bold text-2xl mr-2'>{data.detail.name}</h2>
        <p className="text-gray-500 text-sm">{data.detail.ip}</p>
      </div>
      <div className="flex mt-2 text-sm">
        <button className={`border border-gray-600 shadow px-2 py-1`+(fixedY?` bg-gray-500 text-white`:` bg-white`)} type="button" onClick={()=>setFixedY(!fixedY)}>对齐y轴</button>
      </div>
      <div className='mt-3 gap-2 grid grid-cols-1 lg:grid-cols-2 2xl:grid-cols-3'>
        {data.targets.map((item) => {
          return (
            <div className='bg-white border shadow rounded p-2' key={item.id}>
              <h3 className='font-bold text-lg'>{item.name}</h3>
              <ul className="flex text-xs text-gray-400 items-center mb-1">
                <li className="before:content-['#'] my-1 mr-1">{item.ipv6 ? "IPv6" : "IPv4"}</li>
              </ul>
              <TcpPingBlock mid={params.mid} tid={item.id} fixedY={fixedY} />
            </div>
          )
        })}
      </div>
    </div>

  )
}