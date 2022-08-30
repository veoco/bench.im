import { Chart, Geom, Axis, LineAdvance } from "bizcharts";
import DataSet from '@antv/data-set';
import { FormattedMessage } from "react-intl";


const MachineTaskChart = ({ item, name }) => {
  const data = [];
  const title = {
    "30h": <FormattedMessage defaultMessage="Last 30 hours" />,
    "10d": <FormattedMessage defaultMessage="Last 10 days" />,
    "360d": <FormattedMessage defaultMessage="Last 360 days" />
  }
  const fmt = {
    "30h": "DD HH:mm",
    "10d": "MM/DD HH:mm",
    "360d": "YY MM/DD"
  }
  let speed_max = 100;
  let ping_max = 300;
  for (const row of item[name]) {
    const hour = new Date(row[0] * 1000);
    const upload = row[1];
    const download = row[2];
    const ping = row[3]

    const r = { "hour": hour };
    if (upload) {
      r.upload = upload;
      speed_max = speed_max < 1000 && upload > 100 ? 1000 : speed_max;
      speed_max = speed_max < 10000 && upload > 1000 ? 10000 : speed_max;
    }
    if (download) {
      r.download = download;
      speed_max = speed_max < 1000 && download > 100 ? 1000 : speed_max;
      speed_max = speed_max < 10000 && download > 1000 ? 10000 : speed_max;
    }
    if (ping) {
      r.ping = ping;
      ping_max = ping_max < 1000 && ping > 300 ? 1000 : ping_max;
    }
    data.push(r);
  }

  const dv = new DataSet.View().source(data);
  dv.transform({
    type: "fold",
    fields: ["upload", "download"],
    key: "key",
    value: "value"
  });

  const scale = {
    value: {
      alias: "Speed",
      type: 'linear-strict',
      formatter: (val) => {
        return val + ' Mbps';
      },
      nice: true,
      tickCount: 12,
      max: speed_max,
      min: 0
    },
    ping: {
      alias: "Latency",
      type: 'linear-strict',
      formatter: (val) => {
        return val + ' ms';
      },
      tickCount: 12,
      max: ping_max,
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

  return (
    <div className="border border-gray-700 px-2 mt-2">
      <h4 className="text-center my-1 text-sm text-gray-700">{title[name]}</h4>
      <Chart height={200} data={dv.rows} scale={scale} autoFit>
        <Axis name="hour" {...axisConfig} />
        <Axis name="value" {...axisConfig} />
        <Axis name="ping" {...axisConfig} />
        <LineAdvance type="interval" position="hour*value" color={["key", ["skyblue", "lightcoral"]]} area />
        <LineAdvance type="line" position="hour*ping" size={1} color={"green"} shape={"hv"} />
      </Chart>
    </div>
  )
}

export default MachineTaskChart;