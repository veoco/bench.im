import useSWR from "swr";

export default function IndexPage({ params }) {
  const target = params.slug ? `/api/pages/${params.slug}` : `/api/pages/`;
  const { data, error, isLoading } = useSWR(target, (url) => fetch(url).then(res => res.text()))

  if (error) return <div>
    <p>网络错误</p>
  </div>
  if (isLoading) return <div>
    <p>加载中</p>
  </div>

  return (
    <div className='prose-custom' dangerouslySetInnerHTML={{ __html: data }}>
    </div>
  )
}