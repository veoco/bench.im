import { useEffect } from "react";
import { Link, useParams } from "react-router-dom";
import useSWR from 'swr'
import { useIntl } from "react-intl";

import Searchbar from "./searchbar";
import ServerItem from "./server_item";

const ServerList = () => {
  let params = useParams();
  const intl = useIntl();
  const { data, error } = useSWR(`/api/server_list/?pk=${params.serverListId}`)

  useEffect(() => {
    const title = intl.formatMessage({ defaultMessage: 'Server list' });
    document.title = `${title} ${params.serverListId} - Bench.im`;
  });

  if (error || !data) {
    return (
      <div></div>
    )
  }

  const created = new Date(data.created);
  const modified = new Date(data.modified);
  const readme = { "__html": data.readme }

  return (
    <div>
      <Searchbar />
      <div className="mx-auto sm:w-2/5 text-justify">
        <div className="border border-gray-700 bg-white p-2">
          <h3><span className="before:content-['#'] px-1 mr-2 bg-pink-700 text-white">{data.pk}</span>{data.name}{data.editable ? <Link className="float-right" to={`/server_list/?pk=${data.pk}&edit=1`}>🖊️</Link> : ""}</h3>
          <div className="text-gray-400 mb-1">
            {data.owner ? <span className="mr-2">{data.owner.username}</span> : ""}
            <span>{created == modified ? created.toLocaleString() : modified.toLocaleString()}</span>
          </div>
          <div className="break-all prose max-w-none prose-p:my-2 prose-pre:my-2" dangerouslySetInnerHTML={readme}>
          </div>
          <div>
            {data.servers.map((item) => {
              return (
                <ServerItem item={item} key={item.pk} />
              )
            })}
          </div>
        </div>
      </div>
    </div>
  )
}

export default ServerList;