export default function LightBlock({ updated }) {
  return (
    <div className="flex items-center ml-auto">
      <span className={`w-2 h-2 rounded-full mr-2 ${!updated ? 'bg-gray-400' :
        (Date.now() - new Date(updated).getTime() < 5 * 60 * 1000 ? 'bg-green-500' :
          (Date.now() - new Date(updated).getTime() < 10 * 60 * 1000 ? 'bg-yellow-500' : 'bg-red-500'))}`}
        title={updated ? `最后更新: ${new Date(updated).toLocaleString()}` : '未上线'}>
      </span>
    </div>
  )
}