import { Link } from "react-router-dom";
import { FormattedMessage } from "react-intl";
import { useSWRConfig } from 'swr'

import MachineTaskChart from "./machine_task_chart";

const MachineTaskItem = ({ item }) => {
  const { mutate } = useSWRConfig()

  const modified = new Date(item.modified);
  const colors = {
    "Wait": "bg-amber-400",
    "Work": "bg-emerald-400",
    "Finish": "bg-red-400"
  }

  const handleStop = async (e) => {
    e.preventDefault();
    const data = {
      "pk": item.pk
    }
    try {
      const r = await fetch("/api/machine_task/", {
        method: 'PUT',
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
      mutate(`/api/machine/?pk=${item.machine_id}`);
    }
    catch (err) {
      alert("Network Error! Please refresh the page and try again.")
    }
  }

  return (
    <div className="my-2 border border-gray-700 bg-white p-2 group">
      <h3>
        <span className={`before:content-['#'] px-1 mr-2 text-white ${colors[item.state]}`}>{item.pk}</span>
        <Link to={`/machine/${item.machine_id}/task/${item.pk}/`}>{item.detail.server}</Link>
        {item.state == "Finish" ? "" : <button className="float-right invisible group-hover:visible" onClick={handleStop}>🛑</button>}
      </h3>
      <div className="text-gray-400">
        <FormattedMessage defaultMessage="Last modified:" />
        <span> {modified.toLocaleString()}</span>
      </div>
      {item["3h"] ? <MachineTaskChart item={item} name="3h" /> : ""}
      {item["30h"] ? <MachineTaskChart item={item} name="30h" /> : ""}
      {item["10d"] ? <MachineTaskChart item={item} name="10d" /> : ""}
      {item["360d"] ? <MachineTaskChart item={item} name="360d" /> : ""}
    </div>
  )
}

export default MachineTaskItem;