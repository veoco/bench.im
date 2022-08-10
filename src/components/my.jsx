import { useEffect } from "react";
import useSWR from 'swr'
import { useNavigate, Link } from "react-router-dom";

import ServerListItem from "./server_list_item";

const My = () => {
  const { data, error } = useSWR(`/api/my/`);
  const navigate = useNavigate();

  const logined = new Date(localStorage.getItem('logined'));
  const now = new Date();
  if ((now - logined) > 14 * 86400000) {
    navigate(`/login/`);
  }

  useEffect(() => {
    document.title = `My - Bench.im`;
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
        alert(`Invalid: ${res.msg}`);
        return;
      }
      alert("Server Error! Please refresh the page and try again.")
      return;
    }
    localStorage.removeItem('logined');
    navigate(`/login/`);
  }

  return (
    <div>
      <div className="mx-auto sm:w-2/5 py-2 text-justify">
        <p>You have {data.count} server list <Link className="text-sm float-right bg-white w-5 text-center border border-gray-700" to="/server_list/">+</Link></p>
        <button className='bg-white border border-gray-700 p-2 w-full my-2' onClick={handleLogout}>Logout</button>
        {data.results.map((item) => {
          return (
            <ServerListItem item={item} key={item.pk} />
          )
        })}
      </div>
    </div>
  )
}

export default My;