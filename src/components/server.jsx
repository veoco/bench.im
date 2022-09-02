import { useEffect } from "react";
import { Link, useParams } from "react-router-dom";
import { useIntl } from "react-intl";
import useSWR from 'swr'

import Searchbar from "./searchbar";

const Server = () => {
  let params = useParams();
  const intl = useIntl();
  const { data, error } = useSWR(`/api/server/?pk=${params.serverId}`)

  useEffect(() => {
    const title = intl.formatMessage({ defaultMessage: 'Server' });
    document.title = `${title} ${params.serverId} - Bench.im`;
  });

  if (error || !data) {
    return (
      <div></div>
    )
  }

  let name, host, cc;
  if (data.provider == "Ookla") {
    name = `${data.detail.sponsor} - ${data.detail.name}`;
    host = data.detail.host;
    cc = ' · ' + data.detail.cc
  } else if (data.provider == "LibreSpeed") {
    name = `${data.detail.sponsorName} - ${data.detail.name}`;
    host = data.detail.dl;
    cc = "";
  }
  const created = new Date(data.created);
  const modified = new Date(data.modified);

  return (
    <div>
      <Searchbar />
      <div className="mx-auto sm:w-2/5 text-justify">
        <div className="border border-gray-700 bg-white p-2">
          <h3><span className="before:content-['#'] px-1 mr-2 bg-sky-700 text-white">{data.pk}</span>{name}{data.editable ? <Link className="float-right" to={`/server/?pk=${data.pk}&edit=1`}>🖊️</Link> : ""}</h3>
          <div className="text-gray-400">
            {data.owner ? <span className="mr-2">{data.owner.username}</span> : ""}
            <span>{created == modified ? created.toLocaleString() : modified.toLocaleString()}</span>
          </div>
          <p>{data.provider} - {data.detail.id}{cc} - {data.detail.ipv6 ? "IPv6 · " : ""}{host}</p>
        </div>
      </div>
    </div>
  )
}

export default Server;