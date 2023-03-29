import useSWR from "swr";
import { Link } from "wouter";

export default function ServersBlock({ token }) {
  const { data, error, isLoading } = useSWR(`/api/servers/?token=${token}`)

  if (error) return <div className="bg-white shadow rounded p-2 mb-3">
    <h3 className="font-bold pb-2 flex">
      目标服务器
      <Link className="ml-auto text-xs border border-gray-500 px-2 py-1" href={`/t/${token}/servers/new`}>新增</Link>
    </h3>
    <p>未找到</p>
  </div>
  if (isLoading) return <div className="bg-white shadow rounded p-2 mb-3">
    <h3 className="font-bold pb-2 flex">
      目标服务器
      <Link className="ml-auto text-xs border border-gray-500 px-2 py-1" href={`/t/${token}/servers/new`}>新增</Link>
    </h3>
    <p>加载中</p>
  </div>

  return (
    <div className="bg-white shadow rounded p-2 mb-3">
      <h3 className="font-bold pb-2 flex">
        目标服务器
        <Link className="ml-auto text-xs border border-gray-500 px-2 py-1" href={`/t/${token}/servers/new`}>新增</Link>
      </h3>
      {data.length > 0 ? data.map((item) => {
        const created = new Date(item.created);
        return (
          <div className="border-t p-1 flex items-center" key={item.id}>
            <div>
              <div className="flex items-center">
                <span className="mr-auto">{item.name}</span>
                <span>{item.ipv6}</span>
              </div>
              <div className="flex text-xs items-center">
                <span className="mr-auto">创建于 {created.toLocaleString()}</span>
              </div>
            </div>
            <div className="ml-auto">
              <Link className="border border-gray-500 px-2 py-1" href={`/t/${token}/servers/${item.id}/tasks/`}>查看详情</Link>
            </div>
          </div>
        )
      }) : <p className="border-t p-1">无</p>}
    </div>
  )
}