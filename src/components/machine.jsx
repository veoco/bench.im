import { useState, useEffect } from "react";
import { useParams } from 'react-router-dom';
import { FormattedMessage, useIntl } from "react-intl";
import useSWR from 'swr'

import MachineItem from "./machine_item";
import MachineTaskItem from "./machine_task_item";
import MachineTaskCreate from "./machine_task_create";

const Machine = () => {
  const { uuid } = useParams();
  const { data, error } = useSWR(`/api/machine/?pk=${uuid}`);
  const [show, setShow] = useState(false)
  const intl = useIntl();

  useEffect(() => {
    const title = intl.formatMessage({ defaultMessage: 'Machine' });
    document.title = `${title} - Bench.im`;
  });

  if (error || !data) {
    return (
      <div></div>
    )
  }

  return (
    <div className="mx-auto sm:w-2/5 py-2 text-justify">
      <MachineItem item={data} />
      <h3>
        <FormattedMessage defaultMessage="Machine Tasks" />
        <button className="text-sm float-right bg-white w-5 text-center border border-gray-700" onClick={() => { setShow(!show) }}>+</button>
      </h3>
      {show ? <MachineTaskCreate uuid={uuid} setShow={setShow} /> : ""}
      {data.tasks.map((item) => {
        return (
          <MachineTaskItem item={item} key={item.pk} />
        )
      })}
    </div>
  )
}

export default Machine;