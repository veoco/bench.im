import { useState } from 'react'
import { useLocation } from 'wouter'

import { serversList } from './servers';

export default function NewPage({ params }) {
  const [location, setLocation] = useLocation();

  const [name, setName] = useState("")
  const [download, setDownload] = useState("")
  const [upload, setUpload] = useState("")
  const [ipv6, setIpv6] = useState(false)
  const [multi, setMulti] = useState(false)

  const serverKeys = Object.keys(serversList);
  const [serverKey, setSeverKey] = useState(serverKeys[0])
  const [quickIpv6, setQuickIpv6] = useState(false)
  const [quickMulti, setQuickMulti] = useState(false)

  const handleQuickSubmit = async (e) => {
    e.preventDefault();

    const server = serversList[serverKey];

    const data = {
      token: params.token,
      name: serverKey,
      download_url: server.download_url,
      upload_url: server.upload_url,
      ipv6: quickIpv6,
      multi: quickMulti,
    }
    const r = await fetch("/api/servers/", {
      method: "POST",
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data)
    })
    if (!r.ok) {
      alert("创建失败");
      return
    }
    setLocation(`/t/${params.token}`);
  }

  const handleCustomSubmit = async (e) => {
    e.preventDefault();

    const data = {
      token: params.token,
      name: name,
      download_url: download,
      upload_url: upload,
      ipv6: ipv6,
      multi: multi,
    }
    const r = await fetch("/api/servers/", {
      method: "POST",
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data)
    })
    if (!r.ok) {
      alert("创建失败");
      return
    }
    setLocation(`/t/${params.token}`);
  }

  return (
    <>
      <form className='flex flex-col shadow bg-white rounded p-2 my-3' onSubmit={handleQuickSubmit}>
        <h3 className='font-bold border-b pb-2'>快速新增目标服务器</h3>
        <select value={serverKey} onChange={(e) => setSeverKey(e.target.value)}>
          {serverKeys.map((item) => {
            return (
              <option value={item} key={item}>{item}</option>
            )
          })}
        </select>
        <div className='flex my-3'>
          <button className={'border border-gray-500 px-2 py-1 mr-2' + (quickIpv6 ? " bg-gray-400 text-white" : " bg-white")} type='button' onClick={() => setQuickIpv6(!quickIpv6)}>IPv6</button>
          <button className={'border border-gray-500 px-2 py-1 mr-2' + (quickMulti ? " bg-gray-400 text-white" : " bg-white")} type='button' onClick={() => setQuickMulti(!quickMulti)}>多线程</button>
        </div>
        <button className='border border-gray-500 px-2 py-2 my-2' type='submit'>创建</button>
      </form>
      <form className='flex flex-col shadow bg-white rounded p-2' onSubmit={handleCustomSubmit}>
        <h3 className='font-bold border-b pb-2'>自定义新增目标服务器</h3>
        <label className='my-2' htmlFor="name">名称：</label>
        <input type="text" name="name" value={name} onChange={(e) => setName(e.target.value)} />
        <label className='my-2' htmlFor="download">下载网址：</label>
        <input type="text" name="download" value={download} onChange={(e) => setDownload(e.target.value)} />
        <label className='my-2' htmlFor="upload">上传网址：</label>
        <input type="text" name="upload" value={upload} onChange={(e) => setUpload(e.target.value)} />
        <div className='flex my-3'>
          <button className={'border border-gray-500 px-2 py-1 mr-2' + (ipv6 ? " bg-gray-400 text-white" : " bg-white")} type='button' onClick={() => setIpv6(!ipv6)}>IPv6</button>
          <button className={'border border-gray-500 px-2 py-1 mr-2' + (multi ? " bg-gray-400 text-white" : " bg-white")} type='button' onClick={() => setMulti(!multi)}>多线程</button>
        </div>
        <button className='border border-gray-500 px-2 py-2 my-2' type='submit'>创建</button>
      </form>
    </>

  )
}