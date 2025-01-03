import useSWR from "swr";

export default function IndexPage() {
  const target = `/content.html`;
  const { data, error, isLoading } = useSWR(target, (url) => fetch(url).then(res => res.text()))

  if (error) return <div className="p-2">
    <p>网络错误</p>
  </div>
  if (isLoading) return <div className="p-2">
    <p>加载中</p>
  </div>

  return (
    <div className='prose-custom px-2' dangerouslySetInnerHTML={{ __html: data }}>
    </div>
  )
}