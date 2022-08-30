import { Chart, Geom, Axis, LineAdvance } from "bizcharts";
import DataSet from '@antv/data-set';


const MachineTaskChart = ({ item, name }) => {
  const data = [];
  const fmt = {
    "3h": "HH:mm",
    "30h": "DD HH:mm",
    "10d": "MM/DD",
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
      speed_max = speed_max < 10000 && download > 1000 ? 10000: speed_max;
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
      tickCount: 5,
      max: speed_max,
      min: 0
    },
    ping: {
      alias: "Latency",
      type: 'linear-strict',
      formatter: (val) => {
        return val + ' ms';
      },
      tickCount: 5,
      max: ping_max,
      min: 0
    },
    hour: {
      alias: "Time",
      type: "timeCat",
      mask: fmt[name],
      nice: true
    },
  };

  const axisConfig = {
    line: {
      style: {
        stroke: '#ccc',
        lineDash: [3, 3],
      }
    },
    grid: {
      line: {
        style: {
          stroke: '#ccc',
          lineDash: [3, 3],
        },
      }
    },
  }

  return (
    <Chart className="border border-gray-700 p-2 pt-2 pb-0 mt-2" height={200} data={dv.rows} scale={scale} autoFit>
      <Axis name="hour" {...axisConfig} />
      <Axis name="value" {...axisConfig} />
      <Axis name="ping" {...axisConfig} />
      <LineAdvance type="interval" position="hour*value" color={["key", ["skyblue", "lightcoral"]]} area />
      <LineAdvance type="line" position="hour*ping" size={1} color={"green"} shape={"hv"} />
    </Chart>
  )
}

export default MachineTaskChart;