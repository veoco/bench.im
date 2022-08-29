import { useState } from "react";
import { FormattedMessage } from "react-intl";
import { useSWRConfig } from 'swr'


const MachineTaskCreate = ({ uuid, setShow }) => {
  const { mutate } = useSWRConfig()

  const [machineId, setMachineId] = useState(uuid);
  const [ipv6, setIpv6] = useState(false);
  const [thread, setThread] = useState(1);
  const [serverId, setServerId] = useState('');

  const handleSubmit = async (e) => {
    e.preventDefault();

    const data = {
      "machine_id": machineId,
      "ipv6": ipv6,
      "thread": thread,
      "server_id": serverId,
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
      setShow(false)
      mutate(`/api/machine/?pk=${uuid}`);
    }
    catch (err) {
      alert("Network Error! Please refresh the page and try again.")
    }
  }

  return (
    <div className="my-2">
      <form onSubmit={handleSubmit}>
        <label><FormattedMessage defaultMessage="Server id:" /><br /></label>
        <input className="w-full" type="text" value={serverId} onChange={(e) => setServerId(e.target.value)} />
        <label><FormattedMessage defaultMessage="Theads:" /><br /></label>
        <input className="w-full" type="text" value={thread} onChange={(e) => setThread(e.target.value)} />
        <label><FormattedMessage defaultMessage="IPv6:" /></label>
        <input className="mx-2" type="checkbox" checked={ipv6} onChange={(e) => setIpv6(!ipv6)} />
        <button className="w-full border border-gray-700 bg-white my-2 py-1.5" type="submit"><FormattedMessage defaultMessage="Submit" /></button>
      </form>
    </div>

  )
}

export default MachineTaskCreate;