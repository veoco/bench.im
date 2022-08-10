import { Link } from "react-router-dom";

const ServerListItem = ({ item }) => {
  const created = new Date(item.created);
  const modified = new Date(item.modified);
  const readme = {"__html": item.readme}

  return (
    <div className="my-2 border border-gray-700 bg-white p-2">
      <h3><span className="before:content-['#'] px-1 mr-2 bg-stone-700 text-white">{item.pk}</span><Link to={`/server_list/${item.pk}/`}>{item.name}</Link></h3>
      <div className="text-gray-400">
        {item.owner ? <span className="mr-2">{item.owner.username}</span> : ""}
        <span>{created == modified ? created.toLocaleString() : modified.toLocaleString()}</span>
      </div>
      <div className="truncate" dangerouslySetInnerHTML={readme}>
      </div>
    </div>
  )
}

export default ServerListItem;