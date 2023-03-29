import { useState } from 'react'

export default function NewServerBlock({ token }) {
  const [name, setName] = useState("")

  return (
    <div className="bg-white shadow rounded p-2 my-3">
      <h3 className="font-bold pb-2 border-b">增加目标服务器</h3>
      <p className='my-2'>服务器名称（16 字符以内，留空自动使用 IP 地址）：</p>
      <input className='px-2 py-1 shadow w-full' type="text" value={name} placeholder={"输入名称"} onChange={(e) => setName(e.target.value)} />
      <p className='my-2'>测试使用（默认 x86_64，<a className='underline' href="https://github.com/veoco/bim" target="_blank">其他</a>）：</p>
      <div className="bg-gray-100 p-2 font-mono">
        <p>wget https://bench.im/dl/bim -O bim && chmod +x bim</p>
        <p>./bim {token ? token : "tester"} {name ? `-n ${name}` : ""}</p>
      </div>
    </div>
  )
}