import { useState, useEffect } from "react";
import { useLocation } from "wouter";

export default function EditMachinePage({ params }) {
  const [name, setName] = useState("")
  const [ip, setIp] = useState("")
  const [key, setKey] = useState("");

  const [location, setLocation] = useLocation();
  const token = sessionStorage.getItem("token");

  useEffect(() => {
    if (params.mid) {
      fetch(`/api/admin/machines/${params.mid}`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${token}`
        }
      })
        .then(res => res.json())
        .then(res => {
          setName(res.name)
          setIp(res.ip)
          setKey(res.key)
        })
    }
  }, [params.mid])

  const handleSubmit = async (e) => {
    e.preventDefault();

    const url = params.mid ? `/api/admin/machines/${params.mid}` : "/api/admin/machines/"
    const r = await fetch(url, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({ name, ip, key })
    })
    if (!r.ok) {
      alert(params.mid ? "编辑失败" : "新增失败")
      return;
    }

    setLocation("/");
  }

  return (
    <div className="flex justify-center p-2 items-center sm:h-screen">
      <form className="max-w-sm w-full mt-6 border border-neutral-400 bg-neutral-100 p-3 sm:mt-0" onSubmit={handleSubmit}>
        <h2 className="text-center text-lg font-bold py-1">{params.mid ? "编辑机器" : "新增机器"}</h2>
        <label htmlFor="name">名称：</label>
        <input className="border my-3 p-2 w-full" type="text" placeholder={name} value={name} onChange={(e) => setName(e.target.value)} />
        <label htmlFor="ip">IP:</label>
        <input className="border my-3 p-2 w-full" type="text" placeholder={ip} value={ip} onChange={(e) => setIp(e.target.value)} />
        <label htmlFor="key">密钥:</label>
        <input className="border my-3 p-2 w-full" type="text" placeholder={key} value={key} onChange={(e) => setKey(e.target.value)} />
        <button className="w-full p-2 button" type="submit">{params.mid ? "编辑" : "新增"}</button>
      </form>
    </div>
  )
}