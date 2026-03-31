export default function LightBlock({ updated }) {
  const currentTime = Date.now();
  const updatedTime = updated ? new Date(updated * 1000).getTime() : null;
  return (
    <div className="flex items-center ml-auto">
      <span className={`w-2 h-2 rounded-full mr-2 ${!updated ? 'bg-gray-400' :
        (currentTime - updatedTime < 5 * 60 * 1000 ? 'bg-green-500' :
          (currentTime - updatedTime < 10 * 60 * 1000 ? 'bg-yellow-500' : 'bg-red-500'))}`}
        title={updated ? `最后更新: ${new Date(updatedTime).toLocaleString()}` : '未上线'}>
      </span>
    </div>
  )
}