import { Chart, Axis, LineAdvance } from "bizcharts";
import DataSet from '@antv/data-set';
import { FormattedMessage, useIntl } from "react-intl";
import { useState } from "react";


const MachineTaskChart = ({ item, name }) => {
  const intl = useIntl();
  const [yMax, setYMax] = useState(500);

  const data = [];
  const title = {
    "30h": <FormattedMessage defaultMessage="Last 30 hours" />,
    "10d": <FormattedMessage defaultMessage="Last 10 days" />,
    "360d": <FormattedMessage defaultMessage="Last 360 days" />
  }
  const fields = {
    "upload": intl.formatMessage({ defaultMessage: 'Upload' }),
    "download": intl.formatMessage({ defaultMessage: 'Download' }),
    "latency": intl.formatMessage({ defaultMessage: 'Latency' }),
    "jitter": intl.formatMessage({ defaultMessage: 'Jitter' })
  }
  const fmt = {
    "30h": "DD HH:mm",
    "10d": "MM/DD HH:mm",
    "360d": "YY MM/DD"
  }

  for (const row of item[name]) {
    const hour = new Date(row[0] * 1000);
    const upload = row[1];
    const download = row[2];
    const latency = row[3]
    const jitter = row[4]

    const r = { "hour": hour };
    if (latency) {
      r[fields["upload"]] = upload;
      r[fields["download"]] = download;
      r[fields["latency"]] = latency;
      r[fields["jitter"]] = jitter;
    }
    data.push(r);
  }

  const dv = new DataSet.View().source(data);
  dv.transform({
    type: "fold",
    fields: [fields["upload"], fields["download"], fields["latency"], fields["jitter"]],
    key: "key",
    value: "value"
  });

  const scale = {
    value: {
      alias: "Speed",
      type: 'linear-strict',
      nice: true,
      tickCount: 11,
      max: yMax,
      min: 0
    },
    hour: {
      alias: "Time",
      type: "time",
      tickCount: 30,
      mask: fmt[name],
      nice: true
    },
  };

  const axisConfig = {
    line: {
      style: {
        stroke: '#c3c3c3',
        lineDash: [1, 1],
      }
    },
    grid: {
      line: {
        style: {
          stroke: '#c3c3c3',
          lineDash: [1, 1],
        },
      }
    },
  }

  const toolTip = ['hour*value*key', (hour, value, key) => {
    return {
      title: `${hour.toLocaleString()}`,
      name: `${key}`,
      value: key == fields["latency"] || key == fields["jitter"] ? `${value} ms` : `${value} Mbps`
    }
  }]

  const handleSetYMax = (e) => {
    e.preventDefault();

    switch (yMax) {
      case 500: setYMax(1000); break;
      case 1000: setYMax(10000); break;
      case 10000: setYMax(100); break;
      case 100: setYMax(200); break;
      case 200: setYMax(500); break;
    }
  }

  return (
    <div className="border border-gray-700 px-2 mt-2">
      <div className="flex my-1 group">
        <h4 className="text-sm mx-auto text-gray-700">{title[name]}</h4>
        <button className="text-xs px-1 invisible group-hover:visible" onClick={handleSetYMax}>🎯</button>
      </div>
      <Chart height={200} data={dv.rows} scale={scale} autoFit>
        <Axis name="hour" {...axisConfig} />
        <Axis name="value" {...axisConfig} />
        <LineAdvance type="interval" position="hour*value" color={["key", ["skyblue", "lightcoral", "lightgreen", "black"]]} tooltip={toolTip} area />
      </Chart>
    </div>
  )
}

export default MachineTaskChart;