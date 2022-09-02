import { useEffect } from "react";
import useSWR from 'swr'
import { useNavigate, Link } from "react-router-dom";
import { FormattedMessage, useIntl } from "react-intl";

import ServerItem from "./server_item";
import ServerListItem from "./server_list_item";
import MachineItem from "./machine_item";

const My = () => {
  const { data, error } = useSWR(`/api/my/`);
  const navigate = useNavigate();
  const intl = useIntl();

  const logined = new Date(localStorage.getItem('logined'));
  const now = new Date();
  if ((now - logined) > 14 * 86400000) {
    navigate(`/login/`);
  }

  useEffect(() => {
    const title = intl.formatMessage({ defaultMessage: 'My' });
    document.title = `${title} - Bench.im`;
    const logined = new Date(localStorage.getItem('logined'));
    const now = new Date();
    if ((now - logined) > 14 * 86400000) {
      navigate(`/login/`);
    }
  });

  if (error || !data) {
    return (
      <div></div>
    )
  }

  const handleLogout = async (e) => {
    const r = await fetch("/api/logout/", {
      method: 'POST'
    })
    if (!r.ok) {
      if (r.status == 400) {
        const res = await r.json();
        const invalid = intl.formatMessage({ defaultMessage: 'Invalid' });
        alert(`${invalid}: ${res.msg}`);
        return;
      }
      const server_error = intl.formatMessage({ defaultMessage: "Server Error! Please refresh the page and try again." });
      alert(server_error);
      return;
    }
    localStorage.removeItem('logined');
    navigate(`/login/`);
  }

  return (
    <div>
      <div className="mx-auto sm:w-2/5 py-2 text-justify">
        <button className='bg-white border border-gray-700 p-2 w-full my-2' onClick={handleLogout}><FormattedMessage defaultMessage="Logout" /></button>
        <div><FormattedMessage defaultMessage="User Profile" /></div>
        <div className="my-2 border border-gray-700 bg-white p-2 leading-8">
          <p><FormattedMessage defaultMessage="Username:" /> {data.user.username}</p>
          <p><FormattedMessage defaultMessage="Email:" /> {data.user.email}</p>
          <p><FormattedMessage defaultMessage="Token:" /> {data.user.token}</p>
        </div>
        <div><FormattedMessage defaultMessage="Machine" /></div>
        {data.machine.count > 0 ? "" : <div className="my-2 p-2 border border-gray-700 bg-white">Run: ./bim -d {data.user.email}:{data.user.token}</div>}
        {data.machine.results.map((item) => {
          return (
            <MachineItem item={item} key={item.pk} />
          )
        })}
        <div><FormattedMessage defaultMessage="You have {count} server" values={{ count: data.server.count }} /><Link className="text-sm float-right bg-white w-5 text-center border border-gray-700" to="/server/">+</Link></div>
        {data.server.results.map((item) => {
          return (
            <ServerItem item={item} key={item.pk} />
          )
        })}
        <div><FormattedMessage defaultMessage="You have {count} server list" values={{ count: data.server_list.count }} /><Link className="text-sm float-right bg-white w-5 text-center border border-gray-700" to="/server_list/">+</Link></div>
        {data.server_list.results.map((item) => {
          return (
            <ServerListItem item={item} key={item.pk} />
          )
        })}
      </div>
    </div>
  )
}

export default My;