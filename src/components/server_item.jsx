const ServerItem = ({ item }) => {
  let name, host, cc;
  if(item.provider == "Ookla"){
    name = `${item.detail.sponsor} - ${item.detail.name}`;
    host = item.detail.host;
    cc = ' · ' + item.detail.cc
  } else if (item.provider == "LibreSpeed"){
    name = `${item.detail.sponsorName} - ${item.detail.name}`;
    host = item.detail.server;
    cc = "";
  }
  return (
    <div className="my-2 border border-gray-700 bg-white p-2 last:mb-0">
      <h3><span className="before:content-['#'] px-1 mr-2 bg-stone-200 text-gray-700">{item.pk}</span>{name}</h3>
      <p className="text-gray-400">{item.provider} - {item.detail.id}{cc} - {host}</p>
    </div>
  )
}

export default ServerItem;