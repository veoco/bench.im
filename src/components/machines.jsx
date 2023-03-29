import useSWR from "swr";

export default function MachinesBlock({ token }) {
  const { data, error, isLoading } = useSWR(`/api/machines/?token=${token}`)

  if (error) return <div className="bg-white shadow rounded p-2">
    <h3 className="font-bold pb-2">测速节点</h3>
    <p>未找到</p>
  </div>
  if (isLoading) return <div className="bg-white shadow rounded p-2">
    <h3 className="font-bold pb-2">测速节点</h3>
    <p>加载中</p>
  </div>

  return (
    <div className="bg-white shadow rounded p-2">
      <h3 className="font-bold pb-2">测速节点</h3>
      {data.length > 0 ? data.map((item) => {
        const modified = new Date(item.modified);
        return (
          <div className="border-t p-1" key={item.id}>
            <div className="flex items-center">
              <span className="mr-auto">{item.name}</span>
              <span>{item.ip}</span>
            </div>
            <div className="flex text-xs items-center">
              <span className="mr-auto">更新于 {modified.toLocaleString()}</span>
              <span className={"px-1 py-0.5 text-white" + (item.status == 0 ? " bg-gray-500" : "  bg-green-500")}>{item.status == 0 ? "离线" : "在线"}</span>
            </div>
          </div>
        )
      }) : <p className="border-t p-1">无</p>}
    </div>
  )
}