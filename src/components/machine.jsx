import { useEffect } from "react";
import { useParams, Link } from 'react-router-dom';
import { FormattedMessage } from "react-intl";
import useSWR from 'swr'

import MachineItem from "./machine_item";
import MachineTaskItem from "./machine_task_item";

const Machine = () => {
  const { uuid } = useParams();
  const { data, error } = useSWR(`/api/machine/?pk=${uuid}`);

  useEffect(() => {
    document.title = `Machine - Bench.im`;
  });

  if (error || !data) {
    return (
      <div></div>
    )
  }

  return (
    <div className="mx-auto sm:w-2/5 py-2 text-justify">
      <MachineItem item={data} />
      <h3><FormattedMessage defaultMessage="Machine Tasks" /> <Link className="text-sm float-right bg-white w-5 text-center border border-gray-700" to={`/machine/${uuid}/task/`}>+</Link></h3>
      {data.tasks.map((item) => {
        return (
          <MachineTaskItem item={item} key={item.pk} />
        )
      })}
    </div>
  )
}

export default Machine;