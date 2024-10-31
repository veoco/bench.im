import { useState, useEffect } from "react";
import { useLocation } from "wouter";

export default function EditTargetPage({ params }) {
  const [name, setName] = useState("")
  const [domain, setDomain] = useState("")
  const [ipv4, setIpv4] = useState("")
  const [ipv6, setIpv6] = useState("")

  const [location, setLocation] = useLocation();
  const token = sessionStorage.getItem("token");

  useEffect(() => {
    if (params.tid) {
      fetch(`/api/admin/targets/${params.tid}`, {
        method: 'GET',
        headers: {
          'Authorization': `Bearer ${token}`
        }
      })
        .then(res => res.json())
        .then(res => {
          setName(res.name)
          setDomain(res.domain)
          setIpv4(res.ipv4)
          setIpv6(res.ipv6)
        })
    }
  }, [params.tid])

  const handleSubmit = async (e) => {
    e.preventDefault();

    const data = { name }
    if (domain && domain.length > 0) data.domain = domain
    if (ipv4 && ipv4.length > 0) data.ipv4 = ipv4
    if (ipv6 && ipv6.length > 0) data.ipv6 = ipv6

    const url = params.tid ? `/api/admin/targets/${params.tid}` : "/api/admin/targets/"
    const r = await fetch(url, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(data)
    })
    if (!r.ok) {
      alert(params.tid ? "编辑失败" : "新增失败")
      return;
    }

    setLocation("/");
  }

  return (
    <div className="flex justify-center">
      <form className="max-w-sm w-full mt-6 border bg-neutral-100 p-3" onSubmit={handleSubmit}>
        <h2 className="text-center text-lg font-bold py-1">{params.tid ? "编辑目标" : "新增目标"}</h2>
        <label htmlFor="name">名称：</label>
        <input className="border my-3 p-2 w-full" type="text" placeholder={name} value={name} onChange={(e) => setName(e.target.value)} />
        <label htmlFor="domain">域名:</label>
        <input className="border my-3 p-2 w-full" type="text" placeholder={domain} value={domain} onChange={(e) => setDomain(e.target.value)} />
        <label htmlFor="ip">IPv4:</label>
        <input className="border my-3 p-2 w-full" type="text" placeholder={ipv4} value={ipv4} onChange={(e) => setIpv4(e.target.value)} />
        <label htmlFor="ip">IPv6:</label>
        <input className="border my-3 p-2 w-full" type="text" placeholder={ipv6} value={ipv6} onChange={(e) => setIpv6(e.target.value)} />
        <button className="w-full p-2 button" type="submit">{params.tid ? "编辑" : "新增"}</button>
      </form>
    </div>
  )
}