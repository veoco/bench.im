import { useState, useEffect } from "react";
import { useSearchParams, useNavigate, Link } from "react-router-dom";
import useSWR from 'swr'
import { FormattedMessage, useIntl } from "react-intl";

import ServerListIdItems from "./server_list_id_items";

const ServerListCreate = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [name, setName] = useState('');
  const [readme, setReadme] = useState('');

  const [serverDict, setServerDict] = useState({
    "serverId": "",
    "serverIds": [],
    "dragFrom": -1
  })
  const navigate = useNavigate();
  const intl = useIntl();

  const logined = new Date(localStorage.getItem('logined'));
  const now = new Date();
  let isLogin = true;
  if ((now - logined) > 14 * 86400000) {
    isLogin = false;
  }

  useEffect(() => {
    if (searchParams.get("pk")) {
      const title = intl.formatMessage({ defaultMessage: 'Edit server list' });
      document.title = `${title} ${searchParams.get("pk")} - Bench.im`;
    } else {
      const title = intl.formatMessage({ defaultMessage: 'Create server list' });
      document.title = `${title} - Bench.im`;
    }

    if (!isLogin && searchParams.get("pk")) {
      navigate("/login/");
    }
  });

  let isEdit = false;
  const pk = searchParams.get("pk")
  if (pk) {
    const [changed, setChanged] = useState(false);
    const { data, error } = useSWR(`/api/server_list/?pk=${pk}&edit=1`);
    if (error || !data) {
      return (
        <div></div>
      )
    }
    if (!changed) {
      setName(data.name);
      setServerDict((serverD) => {
        let d = {
          ...serverD,
          "serverIds": data.server_ids,
        }
        for (let s of data.servers) {
          d[s.pk] = s
        }
        return d;
      });
      setReadme(data.readme);
      setChanged(true);
    }
    isEdit = true;
  }

  const handleSubmit = async (e) => {
    e.preventDefault();

    const data = {
      "name": name,
      "readme": readme,
      "servers": serverDict.serverIds
    }
    if (pk) {
      data.pk = pk;
    }
    try {
      const r = await fetch("/api/server_list/", {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(data)
      })
      if (!r.ok) {
        if (r.status == 400) {
          const res = await r.json();
          let msg = "";
          for (let k in res.msg) {
            msg += k + " - " + res.msg[k];
          }
          const invalid = intl.formatMessage({ defaultMessage: 'Invalid' });
          alert(`${invalid} ${msg}`);
          return;
        }
        const server_error = intl.formatMessage({ defaultMessage: "Server Error! Please refresh the page and try again." });
        alert(server_error);
        return;
      }
      const res = await r.json();
      const pk = res.pk;
      navigate(`/server_list/${pk}/`);
    }
    catch (err) {
      const network_error = intl.formatMessage({ defaultMessage: "Network Error! Please refresh the page and try again." });
      alert(network_error)
    }
  }

  return (
    <div>
      <div className="mx-auto sm:w-2/5 py-2 text-justify">
        {isLogin ? isEdit ? <p className="text-justify"><Link className="underline" to="/my/"><FormattedMessage defaultMessage="You" /></Link><FormattedMessage defaultMessage=" are editing server list" /> <span className="before:content-['#'] px-1 mr-2 bg-stone-700 text-white">{pk}</span></p> : <p className="text-justify"><Link className="underline" to="/my/"><FormattedMessage defaultMessage="You" /></Link><FormattedMessage defaultMessage=" are creating a editable server list" /></p> : <p className="text-justify">?????? <Link className="underline" to="/login/"><FormattedMessage defaultMessage="Login" /></Link><FormattedMessage defaultMessage=" to create a editable server list" /></p>}
      </div>
      <div className="mx-auto sm:w-2/5 text-justify leading-8">
        <form onSubmit={handleSubmit}>
          <label><FormattedMessage defaultMessage="Name:" /><br /></label>
          <input className="w-full" type="text" value={name} onChange={(e) => setName(e.target.value)} />
          <label><FormattedMessage defaultMessage="Readme:" /><br /></label>
          <textarea className="w-full" rows="15" value={readme} onChange={(e) => setReadme(e.target.value)}></textarea>
          <ServerListIdItems serverDict={serverDict} setServerDict={setServerDict} />
          <button className="w-full border border-gray-700 bg-white my-2 py-1" type="submit"><FormattedMessage defaultMessage="Submit" /></button>
        </form>
      </div>
    </div>

  )
}

export default ServerListCreate;