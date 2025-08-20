import useSWR from "swr";

import { LightBlock } from "../components";

export default function IndexPage() {
  const { data, error, isLoading } = useSWR(`/api/targets/`)

  if (error) return <div className="p-2">
    <p>网络错误</p>
  </div>
  if (isLoading) return <div className="p-2">
    <p>加载中</p>
  </div>

  return (
    <div className="m-2 grid grid-cols-2 gap-2 sm:grid-cols-4 lg:grid-cols-6">
      {data.map((item) => {
        return (
          <div className='p-2 flex border bg-neutral-100 border-neutral-500' key={item.id}>
            <svg className="w-4 mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor"><path d="M5 11H19V5H5V11ZM21 4V20C21 20.5523 20.5523 21 20 21H4C3.44772 21 3 20.5523 3 20V4C3 3.44772 3.44772 3 4 3H20C20.5523 3 21 3.44772 21 4ZM19 13H5V19H19V13ZM7 15H10V17H7V15ZM7 7H10V9H7V7Z"></path></svg>
            {item.name}
            <LightBlock updated={item.updated} />
          </div>)
      })}
    </div>
  )
}