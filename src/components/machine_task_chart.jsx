import { Chart, Geom, Axis } from "bizcharts";
import DataSet from '@antv/data-set';


const MachineTaskChart = ({item, name}) => {
  const data = [];
  const fmt = {
    "3h": "HH:mm",
    "30h": "DD HH:mm",
    "10d": "MM/DD",
    "360d": "YY MM/DD"
  }
  for (const row of item[name]) {
    const hour = new Date(row[0] * 1000);
    const upload = row[1];
    const download = row[2];
    const ping = row[3]

    const r = { "hour": hour };
    if (upload) {
      r.upload = upload;
    }
    if (download) {
      r.download = download;
    }
    if (ping) {
      r.ping = ping;
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

  const scale_speed = {
    value: {
      alias: "Speed",
      type: 'linear-strict',
      formatter: (val) => {
        return val + ' Mbps';
      },
      nice: true,
      tickCount: 5,
      min: 0
    },
    ping: {
      alias: "Latency",
      type: 'linear-strict',
      formatter: (val) => {
        return val + ' ms';
      },
      tickCount: 5,
      min: 0
    },
    hour: {
      alias: "Time",
      type: "timeCat",
      mask: fmt[name],
      nice: true
    },
  };

  return (
    <Chart height={200} data={dv.rows} scale={scale_speed} autoFit>
      <Axis name="hour" />
      <Axis name="value" />
      <Axis name="ping" />
      <Geom type="interval" position="hour*value" color={["key", ["skyblue", "lightcoral"]]} />
      <Geom type="line" position="hour*ping" size={1} color={"green"} shape={"hv"} />
    </Chart>
  )
}

export default MachineTaskChart;