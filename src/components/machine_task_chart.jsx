import { Chart, Axis, LineAdvance } from "bizcharts";
import DataSet from '@antv/data-set';
import { FormattedMessage, useIntl } from "react-intl";


const MachineTaskChart = ({ item, name }) => {
  const intl = useIntl();

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
  let yMax = 100;
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
      yMax = yMax < upload ? upload : yMax;
      yMax = yMax < download ? download : yMax;
      yMax = yMax < latency ? latency : yMax;
      yMax = yMax < jitter ? jitter : yMax;
    }
    data.push(r);
  }

  if (yMax < 100) {
    yMax = 100;
  } else if (yMax < 200) {
    yMax = 200;
  } else if (yMax < 500) {
    yMax = 500;
  } else if (yMax < 1000) {
    yMax = 1000;
  } else {
    yMax = 10000;
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

  return (
    <div className="border border-gray-700 px-2 mt-2">
      <h4 className="text-center my-1 text-sm text-gray-700">{title[name]}</h4>
      <Chart height={200} data={dv.rows} scale={scale} autoFit>
        <Axis name="hour" {...axisConfig} />
        <Axis name="value" {...axisConfig} />
        <LineAdvance type="interval" position="hour*value" color={["key", ["skyblue", "lightcoral", "lightgreen", "black"]]} tooltip={toolTip} area />
      </Chart>
    </div>
  )
}

export default MachineTaskChart;