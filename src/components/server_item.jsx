import { Link } from "react-router-dom";


const ServerItem = ({ item, isEdit, index, serverDict, setServerDict }) => {
  let name, host, cc;
  if (item.provider == "Ookla") {
    name = `${item.detail.sponsor} - ${item.detail.name}`;
    host = item.detail.host;
    cc = ' · ' + item.detail.cc
  } else if (item.provider == "LibreSpeed") {
    name = `${item.detail.sponsorName} - ${item.detail.name}`;
    host = item.detail.dl;
    cc = "";
  }

  const handleDragStart = (e) => {
    if (isEdit) {
      if (index == serverDict.dragFrom) {
        return;
      }
      setServerDict((serverD) => {
        return {
          ...serverD,
          dragFrom: index
        }
      })
    }
  }

  const handleDragDrop = (e) => {
    if (isEdit) {
      if (index == serverDict.dragFrom) {
        return;
      }
      setServerDict((serverD) => {
        let sids = [...serverD.serverIds];
        let t = sids[index];
        sids[index] = serverD.serverIds[serverDict.dragFrom]
        sids[serverDict.dragFrom] = t;
        return {
          ...serverD,
          serverIds: sids
        }
      })
    }
  }

  const handleDragOver = (e) => {
    if (isEdit) {
      e.preventDefault();
    }
  }

  const handleDelete = (e) => {
    e.preventDefault();
    setServerDict((serverD) => {
      let d = [...serverD.serverIds]
      d.splice(index, 1);
      return {
        ...serverD,
        serverIds: d
      }
    })
  }
  const deleteButton = <button className="float-right invisible group-hover:visible" onClick={handleDelete}>❌</button>
  if (isEdit) {
    return (
      <div className="my-2 border border-gray-700 bg-white p-2 group last:mb-0 first:mt-0" draggable onDragStart={handleDragStart} onDragOver={handleDragOver} onDrop={handleDragDrop}>
        <h3><span className="before:content-['#'] px-1 mr-2 bg-sky-700 text-white">{item.pk}</span>{name}</h3>
        <p className="text-gray-400 text-justify">{item.provider} - {item.detail.id}{cc} - {item.detail.ipv6?"IPv6 · ":""}{host}{isEdit ? deleteButton : ""}</p>
      </div>
    )
  }
  return (
    <div className="my-2 border border-gray-700 bg-white p-2 group last:mb-0 first:mt-0">
      <h3><span className="before:content-['#'] px-1 mr-2 bg-sky-700 text-white">{item.pk}</span><Link to={`/server/${item.pk}/`}>{name}</Link></h3>
      <p className="text-gray-400 text-justify">{item.provider} - {item.detail.id}{cc} - {item.detail.ipv6?"IPv6 · ":""}{host}</p>
    </div>
  )
}

export default ServerItem;