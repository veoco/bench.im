import { useState, useEffect } from "react";
import { useSearchParams, useNavigate, Link } from "react-router-dom";
import useSWR from 'swr'
import { FormattedMessage } from "react-intl";

import ServerListIdItems from "./server_list_id_items";

const ServerCreate = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [name, setName] = useState('');
  const [sponsorName, setSponsorName] = useState('');
  const [ipv6, setIpv6] = useState(false);
  const [dl, setDl] = useState('');
  const [ul, setUl] = useState('');

  const navigate = useNavigate();

  const logined = new Date(localStorage.getItem('logined'));
  const now = new Date();
  let isLogin = true;
  if ((now - logined) > 14 * 86400000) {
    isLogin = false;
  }

  useEffect(() => {
    if (searchParams.get("pk")) {
      document.title = `Edit server ${searchParams.get("pk")} - Bench.im`;
    } else {
      document.title = `Create server - Bench.im`;
    }

    if (!isLogin && searchParams.get("pk")) {
      navigate("/login/");
    }
  });

  let isEdit = false;
  const pk = searchParams.get("pk")
  if (pk) {
    const [changed, setChanged] = useState(false);
    const { data, error } = useSWR(`/api/server/?pk=${pk}&edit=1`);
    if (error || !data) {
      return (
        <div></div>
      )
    }
    if (!changed) {
      setName(data.detail.name);
      setSponsorName(data.detail.sponsorName);
      setIpv6(data.detail.ipv6);
      setDl(data.detail.dl);
      setUl(data.detail.ul);
      setChanged(true);
    }
    isEdit = true;
  }

  const handleSubmit = async (e) => {
    e.preventDefault();

    const data = {
      "name": name,
      "sponsor_name": sponsorName,
      "ipv6": ipv6,
      "dl": dl,
      "ul": ul
    }
    if (pk) {
      data.pk = pk;
    }
    try {
      const r = await fetch("/api/server/", {
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
          alert(`Invalid: ${msg}`);
          return;
        }
        alert("Server Error! Please refresh the page and try again.")
        return;
      }
      const res = await r.json();
      const pk = res.pk;
      navigate(`/server/${pk}/`);
    }
    catch (err) {
      alert("Network Error! Please refresh the page and try again.")
    }
  }

  return (
    <div>
      <div className="mx-auto sm:w-2/5 py-2 text-justify">
        {isLogin ? isEdit ? <p className="text-justify"><Link className="underline" to="/my/"><FormattedMessage defaultMessage="You" /></Link><FormattedMessage defaultMessage=" are editing server" /> <span className="before:content-['#'] px-1 mr-2 bg-stone-700 text-white">{pk}</span></p> : <p className="text-justify"><Link className="underline" to="/my/"><FormattedMessage defaultMessage="You" /></Link><FormattedMessage defaultMessage=" are creating a editable server" /></p> : <p className="text-justify">⚠️ <Link className="underline" to="/login/"><FormattedMessage defaultMessage="Login" /></Link><FormattedMessage defaultMessage=" to create a editable server" /></p>}
      </div>
      <div className="mx-auto sm:w-2/5 text-justify leading-8">
        <form onSubmit={handleSubmit}>
          <label><FormattedMessage defaultMessage="Name:" /><br /></label>
          <input className="w-full" type="text" value={name} onChange={(e) => setName(e.target.value)} />
          <label><FormattedMessage defaultMessage="Sponsor Name:" /><br /></label>
          <input className="w-full" type="text" value={sponsorName} onChange={(e) => setSponsorName(e.target.value)} />
          <label><FormattedMessage defaultMessage="Download Url:" /><br /></label>
          <input className="w-full" type="text" value={dl} onChange={(e) => setDl(e.target.value)} />
          <label><FormattedMessage defaultMessage="Upload Url:" /><br /></label>
          <input className="w-full" type="text" value={ul} onChange={(e) => setUl(e.target.value)} />
          <label><FormattedMessage defaultMessage="IPv6:" /></label>
        <input className="mx-2" type="checkbox" checked={ipv6} onChange={(e) => setIpv6(!ipv6)} />
          <button className="w-full border border-gray-700 bg-white my-2 py-1" type="submit"><FormattedMessage defaultMessage="Submit" /></button>
        </form>
      </div>
    </div>

  )
}

export default ServerCreate;