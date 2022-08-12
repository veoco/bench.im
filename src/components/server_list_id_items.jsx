import ServerItem from "./server_item"

const ServerListIdItems = ({ serverDict, setServerDict }) => {
  const handleChange = (e) => {
    setServerDict((serverD) => {
      return {
        ...serverD,
        serverId: e.target.value,
      }
    })
  }

  const handleAdd = async (e) => {
    e.preventDefault();

    if (serverDict.serverIds.indexOf(serverDict.serverId) != -1) {
      alert(`${serverDict.serverId} already exists`)
      return;
    }

    const r = await fetch(`/api/server/?pk=${serverDict.serverId}`);
    if (!r.ok) {
      alert(`Network error or check your input.`)
      return;
    }
    const res = await r.json();
    setServerDict((serverD) => {
      let d = { 
        ...serverD,
        serverId: "",
        serverIds: [...serverD.serverIds, serverDict.serverId] };
      d[res.pk] = res;
      return d
    })
  }
  return (
    <div className="mb-2">
      <label>Server IDs:<br /></label>
      <input className="w-10/12" type="text" value={serverDict.serverId} onChange={handleChange} />
      <button className="bg-white float-right w-2/12 py-1 border border-gray-700 border-l-0" onClick={handleAdd}>Add</button><br />
      <div className={serverDict.serverIds.length > 0 ? "border border-t-0 border-gray-700 bg-stone-200 p-2" : ""}>
        {serverDict.serverIds.map((sid, index) => {
          return (
            <ServerItem item={serverDict[sid]} isEdit={true} index={index} serverDict={serverDict} setServerDict={setServerDict} key={sid} />
          )
        })}
      </div>
    </div>
  )
}

export default ServerListIdItems;