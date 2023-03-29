import { useState } from 'react'

export default function NewMachineBlock({ token }) {
  const [name, setName] = useState("")

  return (
    <div className="bg-white shadow rounded p-2 my-3">
      <h3 className="font-bold pb-2 border-b">增加测速节点</h3>
      <p className='my-2'>节点名称（16 字符以内，留空自动使用 IP 地址）：</p>
      <input className='px-2 py-1 shadow w-full' type="text" value={name} placeholder={"输入名称"} onChange={(e) => setName(e.target.value)} />
      <p className='my-2'>测试使用（默认 x86_64，<a className='underline' href="https://github.com/veoco/bim" target="_blank">其他</a>）：</p>
      <div className="bg-gray-100 p-2 font-mono">
        <p>wget https://bench.im/dl/bimc -O bimc</p>
        <p>chmod +x bimc</p>
        <p>./bimc {token ? token : "tester"} {name ? `-n ${name}` : ""}</p>
      </div>

      <p className='my-2'>或创建 /etc/systemd/system/bimc.service：</p>
      <div className="bg-gray-100 p-2 font-mono">
        <p>[Unit]</p>
        <p>Description=bimc</p>
        <p>After=network.target</p>
        <p className='h-4'></p>
        <p>[Service]</p>
        <p>Type=simple</p>
        <p>ExecStart=/opt/bim/bimc {token ? token : "tester"} {name ? `-n ${name}` : ""}</p>
        <p>User=bim</p>
        <p>Group=bim</p>
        <p className='h-4'></p>
        <p>[Install]</p>
        <p>WantedBy=multi-user.target</p>
      </div>
      <p className='my-2'>注意需要创建 bim 用户和用户组：</p>
      <div className="bg-gray-100 p-2 font-mono">
        <p>groupadd bim </p>
        <p>useradd -s /sbin/nologin -M -g bim bim</p>
      </div>
    </div>
  )
}