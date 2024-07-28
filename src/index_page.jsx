import useSWR from "swr";

export default function IndexPage({ params }) {
  const { data, error, isLoading } = useSWR(`/api/pages/index/`, (url) => fetch(url).then(res => res.text()))

  if (error) return <div className="bg-white shadow rounded p-2 mt-3">
    <p>网络错误</p>
  </div>
  if (isLoading) return <div className="bg-white shadow rounded p-2 mt-3">
    <p>加载中</p>
  </div>

  return (
    <div className='mt-2 prose lg:prose-xl' dangerouslySetInnerHTML={{ __html: data }}>
    </div>
  )
}