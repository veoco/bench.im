import { useLocation } from "wouter";
import useSWR from "swr";

import fetchWithAuth from "./utils";

export default function DeleteMachinePage({ params }) {
  const { data, error, isLoading } = useSWR(`/api/admin/machines/${params.mid}`, fetchWithAuth)
  const token = sessionStorage.getItem("token");

  const [location, setLocation] = useLocation();

  const handleSubmit = async (e) => {
    e.preventDefault();

    const r = await fetch(`/api/admin/machines/${params.mid}`, {
      method: 'DELETE',
      headers: {
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      },
    })
    if (!r.ok) {
      alert("删除失败")
      return;
    }

    setLocation("/");
  }

  if (error) return <div>
    <p>未找到</p>
  </div>
  if (isLoading) return <div>
    <p>加载中</p>
  </div>

  return (
    <div className="flex justify-center p-2 items-center sm:h-screen">
      <form className="max-w-sm w-full mt-6 border border-neutral-400 bg-neutral-100 p-3 sm:mt-0" onSubmit={handleSubmit}>
        <h2 className="text-center text-lg font-bold py-1">删除机器</h2>
        <table className="w-full my-3 bg-white">
          <tbody>
            <tr>
              <td className="border border-neutral-400 px-2 py-1">名称</td>
              <td className="border border-neutral-400 px-2 py-1">{data.name}</td>
            </tr>
            <tr>
              <td className="border border-neutral-400 px-2 py-1">IP</td>
              <td className="border border-neutral-400 px-2 py-1">{data.ip}</td>
            </tr>
            <tr>
              <td className="border border-neutral-400 px-2 py-1">昵称</td>
              <td className="border border-neutral-400 px-2 py-1">{data.nickname}</td>
            </tr>
          </tbody>
        </table>
        <button className="w-full p-2 button" type="submit">确认删除</button>
      </form>
    </div>
  )
}