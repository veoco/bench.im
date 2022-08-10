import { useState, useEffect } from "react";
import { useSearchParams, useNavigate, Link } from "react-router-dom";
import useSWR from 'swr'

const ServerListCreate = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [name, setName] = useState('');
  const [serverIds, setServerIds] = useState('');
  const [readme, setReadme] = useState('');
  const navigate = useNavigate();

  const logined = new Date(localStorage.getItem('logined'));
  const now = new Date();
  let isLogin = true;
  if ((now - logined) > 14 * 86400000) {
    isLogin = false;
  }

  useEffect(() => {
    document.title = `Create server list - Bench.im`;
    if(!isLogin && searchParams.get("pk")){
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
    if(!changed){
      setName(data.name);
      setServerIds(data.servers.join(", "));
      setReadme(data.readme);
      setChanged(true);
    }
    isEdit = true;
  }

  const handleSubmit = async (e) => {
    e.preventDefault();
    const servers = serverIds.split(', ');

    const data = {
      "name": name,
      "readme": readme,
      "servers": servers
    }
    if(pk){
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
          alert(`Invalid: ${msg}`);
          return;
        }
        alert("Server Error! Please refresh the page and try again.")
        return;
      }
      const res = await r.json();
      const pk = res.pk;
      navigate(`/server_list/${pk}/`);
    }
    catch (err) {
      alert("Network Error! Please refresh the page and try again.")
    }
  }

  return (
    <div>
      <div className="mx-auto sm:w-2/5 py-2 text-justify">
        {isLogin ? isEdit ? <p className="text-justify"><Link className="underline" to="/my/">You</Link> are editing server list <span className="before:content-['#'] px-1 mr-2 bg-stone-700 text-white">{pk}</span></p> : <p className="text-justify"><Link className="underline" to="/my/">You</Link> are creating a editable server list.</p> : <p className="text-justify">⚠️ <Link className="underline" to="/login/">Login</Link> to create a editable server list.</p>}
      </div>
      <div className="mx-auto sm:w-2/5 text-justify leading-8">
        <form onSubmit={handleSubmit}>
          <label>Name:<br /></label>
          <input className="w-full" type="text" value={name} onChange={(e) => { setName(e.target.value) }} />
          <label>Server IDs (Separated by ", " like "1, 2"):<br /></label>
          <input className="w-full" type="text" value={serverIds} onChange={(e) => { setServerIds(e.target.value) }} />
          <label>Readme:<br /></label>
          <textarea className="w-full" rows="15" value={readme} onChange={(e) => { setReadme(e.target.value) }}></textarea>
          <button className="w-full border border-gray-700 bg-white my-2 py-1" type="submit">Submit</button>
        </form>
      </div>
    </div>

  )
}

export default ServerListCreate;