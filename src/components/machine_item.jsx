import { Link } from "react-router-dom";
import { FormattedMessage } from "react-intl";

const MachineItem = ({ item }) => {
  const created = new Date(item.created);

  return (
    <div className="my-2 border border-gray-700 bg-white p-2">
      <h3><span className="before:content-['#'] px-1 mr-2 bg-stone-700 text-white">{item.pk.slice(0, 4)}</span><Link to={`/machine/${item.pk}/`}>{item.ip}</Link></h3>
      <div className="text-gray-400">
        <FormattedMessage defaultMessage="Created: " />
        <span>{created.toLocaleString()}</span>
        <span> - <FormattedMessage defaultMessage='Running {count} tasks' values={{ count: item.task_count }} /></span>
      </div>
    </div>
  )
}

export default MachineItem;