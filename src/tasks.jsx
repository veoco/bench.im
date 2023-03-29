import { Link } from "wouter"
import useSWR from "swr";

export default function TasksPage({ params }) {
  const { data, error, isLoading } = useSWR(`/api/tasks/?token=${params.token}&server_id=${params.server}`)

  if (error) return <div>未找到</div>
  if (isLoading) return <div>加载中</div>

  const first = data.length > 0 ? data[0] : { "name": "无", "ipv6": false, "multi": false }

  return (
    <>
      <aside className="p-1 my-2">
        <div className="text-xl">地址：<Link className="underline" href={`/t/${params.token}`}>{params.token}</Link></div>
        <div className="mt-2">测速目标：{first.server.name}</div>
        <ul className="flex text-sm mt-2">
          <li className="mr-3">IPv6: {first.server.ipv6 ? "启用" : "关闭"}</li>
          <li>多线程: {first.server.multi ? "启用" : "关闭"}</li>
        </ul>
      </aside>
      <div className="bg-white shadow rounded p-2">
        <table className="w-full">
          <thead className="border-b border-gray-500">
            <tr className="text-right px-2">
              <th className="text-left">测速节点</th>
              <th>上传/Mbps</th>
              <th>下载/Mbps</th>
              <th>延迟/ms</th>
              <th>抖动/ms</th>
            </tr>
          </thead>
          <tbody>
            {data.length > 0 ? data.map((item) => {
              if (item.status == 1) {
                return (
                  <tr className="text-left px-2">
                    <td>{item.machine.name}</td>
                    <td colSpan={4}>等待中</td>
                  </tr>
                )
              } else if (item.status == 2) {
                return (
                  <tr className="text-left px-2">
                    <td>{item.machine.name}</td>
                    <td colSpan={4}>进行中</td>
                  </tr>
                )
              } else if (item.status == 4) {
                return (
                  <tr className="text-left px-2">
                    <td>{item.machine.name}</td>
                    <td colSpan={4}>超时，已取消</td>
                  </tr>
                )
              }
              return (
                <tr className="text-right px-2">
                  <td className="text-left">{item.machine.name}</td>
                  <td>{item.upload} {item.upload_status}</td>
                  <td>{item.download} {item.download_status}</td>
                  <td>{item.latency}</td>
                  <td>{item.jitter}</td>
                </tr>
              )
            }) : <tr><td>无</td></tr>}
          </tbody>
        </table>
      </div>
    </>

  )
}