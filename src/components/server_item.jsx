const ServerItem = ({ item }) => {
  return (
    <div className="my-2 border border-gray-700 bg-white p-2 last:mb-0">
      <h3><span className="before:content-['#'] px-1 mr-2 bg-stone-200 text-gray-700">{item.pk}</span>{item.detail.name} - {item.detail.sponsor}</h3>
      <p className="text-gray-400">{item.provider} - {item.detail.id} · {item.detail.cc} - {item.detail.host}</p>
    </div>
  )
}

export default ServerItem;