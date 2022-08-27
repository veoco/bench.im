import { useEffect } from "react";
import { Link } from "react-router-dom";
import { FormattedMessage } from "react-intl";
import { Chart, Geom, Axis } from "bizcharts";
import DataSet from '@antv/data-set';

import MachineTaskChart from "./machine_task_chart";

const MachineTaskItem = ({ item }) => {
  const created = new Date(item.created);
  const modified = new Date(item.modified);

  return (
    <div className="my-2 border border-gray-700 bg-white p-2">
      <h3><span className="before:content-['#'] px-1 mr-2 bg-stone-700 text-white">{item.pk}</span><Link to={`/machine/${item.machine_id}/task/${item.pk}/`}>{item.detail.server}</Link></h3>
      <div className="text-gray-400">
        <FormattedMessage defaultMessage="Last modified:" />
        <span> {modified.toLocaleString()}</span>
      </div>
      {item["3h"]?<MachineTaskChart item={item} name="3h" />:""}
      {item["30h"]?<MachineTaskChart item={item} name="30h" />:""}
      {item["10d"]?<MachineTaskChart item={item} name="10d" />:""}
      {item["360d"]?<MachineTaskChart item={item} name="360d" />:""}
    </div>
  )
}

export default MachineTaskItem;