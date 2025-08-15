import { useEffect, useRef } from "react";
import useSWR from "swr";
import uPlot from "uplot";
import "uplot/dist/uPlot.min.css";

export default function PingBlock({ mid, tid, fixedY, dateRange, ipv6 }) {
  const { data, error, isLoading } = useSWR(
    `/api/machines/${mid}/targets/${tid}/${dateRange}` + (ipv6 ? "?ipv6=true" : "")
  );
  const containerRef = useRef(null);
  const plotRef = useRef(null);
  const tooltipRef = useRef(null);

  useEffect(() => {
    if (!data) return;

    const rect = containerRef.current.getBoundingClientRect();

    const nowMs = Date.now();
    const baseStepMs = 300 * 1000;
    const extra = nowMs % baseStepMs;
    const endTime = nowMs - extra + baseStepMs;

    let hours = 24;
    let step = baseStepMs;
    if (dateRange === "7d") {
      hours = 7 * 24;
      step = 6 * baseStepMs;
    }
    const startTime = endTime - hours * 12 * baseStepMs;

    const times = [];
    const mins = [];
    const avgs = [];
    const fails = [];

    let index = 0;
    for (let i = startTime; i <= endTime; i += step) {
      const current = index < data.results.length - 1 ? data.results[index] : null;
      const tSec = i / 1000;

      if (current && new Date(current[0] * 1000) < i) {
        times.push(tSec);
        mins.push(current[1]);
        avgs.push(current[2]);
        fails.push(current[3]);

        while (index < data.results.length - 1) {
          index += 1;
          const ct = new Date(data.results[index][0] * 1000);
          if (ct >= i) break;
        }
      } else {
        times.push(tSec);
        mins.push(null);
        avgs.push(null);
        fails.push(null);
      }
    }

    if (!tooltipRef.current) {
      const tt = document.createElement("div");
      tt.style.position = "absolute";
      tt.style.background = "#fff";
      tt.style.color = "#000";
      tt.style.boxShadow = "0 2px 4px rgba(0, 0, 0, 0.1)";
      tt.style.borderRadius = "4px";
      tt.style.padding = "6px 8px";
      tt.style.pointerEvents = "none";
      tt.style.fontSize = "12px";
      tt.style.whiteSpace = "nowrap";
      tt.style.display = "none";
      tt.style.zIndex = "10";
      containerRef.current.appendChild(tt);
      tooltipRef.current = tt;
    }

    const opts = {
      width: rect.width,
      height: rect.height,
      legend: { show: false },
      padding: [12, 12, -24, -6],
      scales: {
        x: { time: true },
        y: {
          range: fixedY ? [0, 300] : [],
        },
      },
      axes: [
        {
          ticks: { stroke: "#aaa", width: 1, size: 6 },
          grid: { stroke: "#aaa", width: 1, dash: [2, 2] },
          splits: (u, ai, min, max) => {
            const stepSmall = dateRange === "7d" ? 8 * 60 * 60 : 1 * 60 * 60;
            const arr = [];
            for (let t = Math.ceil(min / stepSmall) * stepSmall; t <= max; t += stepSmall) {
              arr.push(t);
            }
            return arr;
          },
          values: (u, splits) => {
            const bigStep = dateRange === "7d" ? 24 * 60 * 60 : 4 * 60 * 60;
            return splits.map(t => {
              if (t % bigStep === 0) {
                const d = new Date(t * 1000);
                return dateRange === "7d"
                  ? `${d.getDate()}-${String(d.getHours()).padStart(2, "0")}`
                  : d.toTimeString().slice(0, 5);
              }
              return "";
            });
          },
        },
        {
          ticks: { stroke: "#aaa", width: 1, size: 4 },
          grid: { stroke: "#aaa", width: 1, dash: [2, 2] },
        },
      ],
      series: [
        { label: "时间" },
        { label: "最低延迟", stroke: "transparent", fill: "transparent", points: { show: false } },
        { label: "平均延迟", stroke: "transparent", fill: "transparent", points: { show: false } },
      ],
      hooks: {
        draw: [
          (u) => {
            const ctx = u.ctx;
            const xdata = u.data[0];
            const ymin = u.data[1];
            const ymax = u.data[2];
            const umin = u.scales.y.min;
            const umax = u.scales.y.max;
            const minh = (umax - umin) / 100;

            const barW = Math.max(2, Math.ceil(rect.width / xdata.length));

            for (let i = 0; i < xdata.length; i++) {
              if (ymin[i] != null && ymax[i] != null) {
                const x = u.valToPos(xdata[i], "x", true);
                const y1 = u.valToPos(ymin[i], "y", true);
                const y2 = u.valToPos(ymax[i] + minh, "y", true);

                let color = "#2ecc71";
                if (fails[i] >= 3) color = "#ee0000";
                else if (fails[i] >= 1) color = "#f1c40f";

                ctx.strokeStyle = color;
                ctx.lineWidth = barW;
                ctx.beginPath();
                ctx.moveTo(x, y1);
                ctx.lineTo(x, y2);
                ctx.stroke();
              } else {
                const x = u.valToPos(xdata[i], "x", true);
                const y1 = u.valToPos(u.scales.y.min, "y", true);
                const y2 = u.valToPos(u.scales.y.max, "y", true);

                ctx.strokeStyle = "#ddd";
                ctx.lineWidth = barW;
                ctx.beginPath();
                ctx.moveTo(x, y1);
                ctx.lineTo(x, y2);
                ctx.stroke();
              }
            }
          },
        ],
        setCursor: [
          (u) => {
            const idx = u.cursor.idx;
            const tt = tooltipRef.current;
            if (idx == null || idx < 0 || idx >= times.length) {
              tt.style.display = "none";
              return;
            }

            const time = new Date(times[idx] * 1000);
            const min = mins[idx];
            const avg = avgs[idx];
            const fail = fails[idx];

            tt.innerHTML = `
              <div>${time.toLocaleString()}</div>
              <div>最低延迟: ${min != null ? min + "ms" : "-"}</div>
              <div>平均延迟: ${avg != null ? avg + "ms" : "-"
              }</div>
              <div>连接失败: ${fail != null ? fail + "/20" : "-"}</div>
            `;

            const offset = 16;
            const cw = containerRef.current.clientWidth;
            const ch = containerRef.current.clientHeight;
            const tw = tt.offsetWidth;
            const th = tt.offsetHeight;

            let xPos = u.cursor.left + u.bbox.left - 18 + offset;
            let yPos = u.cursor.top + u.bbox.top - 8 + offset;

            if (xPos + tw >= cw) xPos = xPos - tw - 2 * offset;
            if (yPos + th >= ch) yPos = yPos - th - 2 * offset;

            tt.style.left = `${xPos}px`;
            tt.style.top = `${yPos}px`;
            tt.style.display = "block";
          },
        ],
      },
    };

    const uplotData = [times, mins, avgs];

    if (plotRef.current) {
      plotRef.current.setData(uplotData);
      plotRef.current.setSize({ width: rect.width, height: rect.height });
    } else {
      plotRef.current = new uPlot(opts, uplotData, containerRef.current);
    }

    return () => {
      if (plotRef.current) {
        plotRef.current.destroy();
        plotRef.current = null;
      }
      if (tooltipRef.current) {
        tooltipRef.current.remove();
        tooltipRef.current = null;
      }
    };
  }, [data, fixedY, dateRange]);

  if (error) return <div className="w-full aspect-video relative"></div>;
  if (isLoading) return <div className="w-full aspect-video relative"></div>;

  return <div className="w-full aspect-video relative" ref={containerRef}></div>;
}
