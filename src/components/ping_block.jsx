import { useEffect, useRef } from "react";
import useSWR from "swr";
import { Chart } from '@antv/g2';

export default function PingBlock({ mid, tid, fixedY, dateRange, ipv6 }) {
  const { data, error, isLoading } = useSWR(`/api/machines/${mid}/targets/${tid}/${dateRange}` + (ipv6 ? "?ipv6=true" : ""));
  const imgRef = useRef(null);

  useEffect(() => {
    if (data) {
      const rect = imgRef.current.getBoundingClientRect();
      const isWide = rect.width > 480;
      const chart = new Chart({
        theme: 'classic',
        width: rect.width,
        height: rect.width / 2,
        paddingLeft: 25,
        paddingBottom: 10,
        paddingRight: 0,
        paddingTop: 0,
      });

      let index = 0;
      const nowTime = new Date();
      const extra = nowTime % (300 * 1000);
      const endTime = nowTime - extra + 300 * 1000;

      let hours = 24;
      let step = 300 * 1000;
      if (dateRange == "7d") { hours = 7 * 24; step = 6 * 300 * 1000 };
      const startTime = endTime - hours * 12 * 300 * 1000;

      const array = [];
      const failedArray = [];
      for (let i = startTime; i <= endTime; i += step) {
        const current = index < data.length - 1 ? data[index] : null;
        const time = new Date(i);

        if (current && new Date(current.created * 1000) < i) {
          array.push({
            "time": time,
            "min": current.min,
            "avg": current.min + current.jitter + (fixedY ? 5 : 0.1),
            "fail": current.failed
          })

          while (index < data.length - 1) {
            index += 1;
            const ct = new Date(data[index].created * 1000);
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
          failedArray.push({
            "time": time,
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
          title: (d) => d ? d.time.toLocaleString() : undefined,
          items: [
            {
              name: "最低延迟",
              valueFormatter: (d) => d ? `${(fixedY ? d - 5 : d - 0.1).toFixed(1)}ms` : undefined,
              field: "min"
            },
            {
              name: "平均延迟",
              valueFormatter: (d) => d ? `${(fixedY ? d - 5 : d - 0.1).toFixed(1)}ms` : undefined,
              field: "avg"
            },
            {
              name: "连接失败",
              valueFormatter: (d) => d ? `${d}/20` : "0/20",
              field: "fail"
            },
          ],
        })
        .axis('x', {
          title: false,
          line: true,
          label: true,
          labelFormatter: (d) => {
            if (dateRange == "7d") {
              return `${d.getDate()}-${d.getHours()}`
            };
            const t = d.toLocaleTimeString();
            return `${t.slice(0, t.length - 3)}`
          },
          labelFilter: (datum, index, data) => index % (isWide ? 4 : 8) === 0,
          tickFilter: (datum, index, data) => index % (dateRange == "7d" ? hours / 28 : hours / 4) === 0,
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
        });

      chart.data(failedArray)
        .lineX()
        .encode('x', 'time')
        .tooltip({
          items: [
            {
              name: "数据缺失",
              valueFormatter: (d) => d ? d.toLocaleString() : undefined,
              field: "time"
            },
          ],
        })
        .style('stroke', '#000')
        .style('strokeOpacity', 0.3)
        .style('lineWidth', 1);


      const container = chart.getContainer();
      imgRef.current.appendChild(container);
      chart.render();
    }
    return () => { imgRef.current && imgRef.current.firstChild ? imgRef.current.removeChild(imgRef.current.firstChild) : null };
  }, [data, fixedY, dateRange]);

  if (error) return <div></div>
  if (isLoading) return <div></div>

  return (
    <div className="font-bold w-full" ref={imgRef}></div>
  )
}