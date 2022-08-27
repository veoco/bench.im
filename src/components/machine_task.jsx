import { useEffect } from "react";
import { useParams } from 'react-router-dom';
import useSWR from 'swr'

import MachineTaskItem from "./machine_task_item";


const MachineTask = () => {
  const { uuid, taskId } = useParams();
  const { data, error } = useSWR(`/api/machine_task/?pk=${taskId}`);

  useEffect(() => {
    document.title = `Machine Task - Bench.im`;
  });

  if (error || !data) {
    return (
      <div></div>
    )
  }

  return (
    <div className="mx-auto sm:w-2/5 py-2 text-justify">
      <MachineTaskItem item={data} />
    </div>
  )
}

export default MachineTask;