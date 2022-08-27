import { useState, useEffect } from "react";
import { useSearchParams, useParams, useNavigate, Link } from "react-router-dom";
import useSWR from 'swr'
import { FormattedMessage } from "react-intl";


const MachineTaskCreate = () => {
  const { uuid } = useParams();
  const [searchParams, setSearchParams] = useSearchParams();

  const [machineId, setMachineId] = useState(uuid);
  const [state, setState] = useState("Wait");
  const [oneshot, setOneshot] = useState(false);
  const [ipv6, setIpv6] = useState(false);
  const [thread, setThread] = useState(1);
  const [serverId, setServerId] = useState('');

  const navigate = useNavigate();

  const logined = new Date(localStorage.getItem('logined'));
  const now = new Date();
  let isLogin = true;
  if ((now - logined) > 14 * 86400000) {
    isLogin = false;
  }

  useEffect(() => {
    if (!isLogin) {
      navigate("/login/");
    }

    if (searchParams.get("pk")) {
      document.title = `Edit machine task ${searchParams.get("pk")} - Bench.im`;
    } else {
      document.title = `Create machine task - Bench.im`;
    }
  });

  let isEdit = false;
  const pk = searchParams.get("pk")
  if (pk) {
    const [changed, setChanged] = useState(false);
    const { data, error } = useSWR(`/api/machine_task/?pk=${pk}&edit=1`);
    if (error || !data) {
      return (
        <div></div>
      )
    }
    if (!changed) {
      setMachineId(data.machine_id);
      setState(data.state);
      setOneshot(data.oneshot)
      setIpv6(data.detail.ipv6)
      setThread(data.detail.ipv6)
      setServerId(data.detail.server)
      setChanged(true);
    }
    isEdit = true;
  }

  const handleSubmit = async (e) => {
    e.preventDefault();

    const data = {
      "machine_id": machineId,
      "state": state,
      "oneshot": oneshot,
      "ipv6": ipv6,
      "thread": thread,
      "server_id": serverId,
    }
    if (pk) {
      data.pk = pk;
    }
    try {
      const r = await fetch("/api/machine_task/", {
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
      navigate(`/machine/${uuid}/`);
    }
    catch (err) {
      alert("Network Error! Please refresh the page and try again.")
    }
  }

  return (
    <div>
      <div className="mx-auto sm:w-2/5 py-2 text-justify">
        {isEdit ? <p className="text-justify"><Link className="underline" to="/my/"><FormattedMessage defaultMessage="You" /></Link><FormattedMessage defaultMessage=" are editing machine task" /> <span className="before:content-['#'] px-1 mr-2 bg-stone-700 text-white">{pk}</span></p> : ""}
      </div>
      <div className="mx-auto sm:w-2/5 text-justify leading-8">
        <form onSubmit={handleSubmit}>
          <p>Machine ID: {machineId}</p>
          <label><FormattedMessage defaultMessage="State:" /><br /></label>
          <select className="w-full" value={state} onChange={(e)=> {setState(e.target.value)}}>
            <option value="Wait">Wait</option>
            <option value="Work">Work</option>
            <option value="Finish">Finish</option>
          </select>
          <label><FormattedMessage defaultMessage="Server id:" /><br /></label>
          <input className="w-full" type="text" value={serverId} onChange={(e) => setServerId(e.target.value)} />
          <label><FormattedMessage defaultMessage="Theads:" /><br /></label>
          <input className="w-full" type="text" value={thread} onChange={(e) => setThread(e.target.value)} />
          <label><FormattedMessage defaultMessage="Oneshot:" /></label>
          <input className="mx-2" type="checkbox" checked={oneshot} onChange={(e) => setOneshot(!oneshot)} />
          <label><FormattedMessage defaultMessage="IPv6:" /></label>
          <input className="mx-2" type="checkbox" checked={ipv6} onChange={(e) => setIpv6(!ipv6)} />
          <button className="w-full border border-gray-700 bg-white my-2 py-1" type="submit"><FormattedMessage defaultMessage="Submit" /></button>
        </form>
      </div>
    </div>

  )
}

export default MachineTaskCreate;