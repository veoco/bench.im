import { useEffect, useRef } from "react";
import useSWR from "swr";
import { Chart } from '@antv/g2';

export default function TcpPingBlock({ mid, tid, fixedY, dateRange }) {
  const { data, error, isLoading } = useSWR(`/api/machines/${mid}/targets/${tid}/${dateRange}`)
  const imgRef = useRef(null);

  useEffect(() => {
    if (data) {
      const rect = imgRef.current.getBoundingClientRect();
      const isWide = rect.width > 480;
      const chart = new Chart({
        theme: 'classic',
        width: rect.width,
        height: rect.width / 3,
        paddingLeft: 40,
        paddingBottom: 25,
        paddingRight: 20,
        paddingTop: 10,
      });

      let index = 0;
      const nowTime = new Date();
      const extra = nowTime % (300 * 1000);
      const endTime = nowTime - extra;

      let hours = 24;
      if (dateRange == "7d") hours = 7 * 24;
      const startTime = endTime - hours * 12 * 300 * 1000;

      const array = [];
      for (let i = startTime; i <= endTime; i += 300 * 1000) {
        const current = index < data.length - 1 ? data[index] : null;
        const time = new Date(i);

        if (current && new Date(current.created) < i) {
          array.push({
            "time": time,
            "min": current.ping_min,
            "avg": current.ping_min + current.ping_jitter,
            "fail": current.ping_failed
          })

          while (index < data.length - 1) {
            index += 1;
            const ct = new Date(data[index].created);
            if (ct >= i) {
              break;
            }
          }
        } else {
          array.push({
            "time": time,
            "min": null,
            "avg": null,
            "fail": null
          })
        }
      }

      chart.interval()
        .data(array)
        .encode('x', 'time')
        .encode('y', ['min', 'avg'])
        .encode('color', (d) => {
          if (d.fail >= 3) return "#ee0000";
          if (d.fail >= 1) return "#f1c40f";
          return "#2ecc71"
        })
        .scale('color', { type: 'identity' })
        .scale('y', {
          domain: fixedY ? [0, 300] : undefined,
          tickCount: 5,
          nice: true
        })
        .tooltip({
          title: (d) => d.time.toLocaleString(),
          items: [
            {
              name: "最低延迟",
              valueFormatter: (d) => `${d.toFixed(1)}ms`,
              field: "min"
            },
            {
              name: "平均延迟",
              valueFormatter: (d) => `${d.toFixed(1)}ms`,
              field: "avg"
            },
            {
              name: "连接失败",
              valueFormatter: (d) => `${d}/20`,
              field: "fail"
            },
          ],
        })
        .axis('x', {
          title: false,
          line: true,
          label: true,
          labelFormatter: (d) => {
            if (dateRange == "7d"){
              return `${d.getDate()}-${d.getHours()}`
            };
            const t = d.toLocaleTimeString();
            return `${t.slice(0, t.length - 3)}`
          },
          labelFilter: (datum, index, data) => index % (isWide ? 4 : 8) === 0,
          tickFilter: (datum, index, data) => index % (hours / 4) === 0,
          tick: true,
          grid: true,
          style: {
            lineLineWidth: 1,
            lineStroke: "#000",
            lineStrokeOpacity: 1,
            tickLength: (datum, index, data) => index % 2 == 0 ? 6 : 3,
            tickLineWidth: 1,
            tickStroke: "#000",
            tickStrokeOpacity: 1,
            gridLineWidth: 1,
            gridLineDash: [1, 1],
            gridStroke: "#000",
            gridStrokeOpacity: 0.2,
          },
        })
        .axis('y', {
          title: false,
          line: true,
          labelAutoHide: true,
          tick: true,
          grid: true,
          style: {
            lineLineWidth: 1,
            lineStroke: "#000",
            lineStrokeOpacity: 1,
            tickLength: 3,
            tickLineWidth: 1,
            tickStroke: "#000",
            tickStrokeOpacity: 1,
            gridLineWidth: 1,
            gridLineDash: [1, 1],
            gridStroke: "#000",
            gridStrokeOpacity: 0.2,
          },
        })

      const container = chart.getContainer();tid
      imgRef.current.appendChild(container);
      chart.render();
    }
    return () => { imgRef.current && imgRef.current.firstChild ? imgRef.current.removeChild(imgRef.current.firstChild) : null };
  }, [data, fixedY, dateRange]);

  if (error) return <div className="border border-gray-400"></div>
  if (isLoading) return <div className="border border-gray-400"></div>

  return (
    <div className="border border-gray-400 font-bold w-full" ref={imgRef}></div>
  )
}