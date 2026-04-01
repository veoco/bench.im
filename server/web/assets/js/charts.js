// Ping 图表类
class PingChart {
    constructor(container, mid, tid) {
        this.container = container;
        this.mid = mid;
        this.tid = tid;
        this.plot = null;
        this.tooltip = null;
        this.fixedY = false;
        this.ipv6 = false;
        this.dateRange = '24h';
        
        this.init();
    }
    
    async init() {
        await this.loadData();
        
        // 创建 tooltip
        this.tooltip = document.createElement('div');
        this.tooltip.className = 'ping-tooltip';
        this.tooltip.style.cssText = `
            position: absolute;
            background: #fff;
            color: #000;
            box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
            border-radius: 4px;
            padding: 6px 8px;
            pointer-events: none;
            font-size: 12px;
            white-space: nowrap;
            display: none;
            z-index: 10;
        `;
        this.container.appendChild(this.tooltip);
    }
    
    async loadData() {
        try {
            const url = `/api/machines/${this.mid}/targets/${this.tid}/${this.dateRange}` + 
                       (this.ipv6 ? '?ipv6=true' : '');
            const data = await apiFetch(url);
            this.render(data);
        } catch (err) {
            console.error('Failed to load ping data:', err);
        }
    }
    
    render(data) {
        if (!data || !data.results) return;
        
        const rect = this.container.getBoundingClientRect();
        
        const nowMs = Date.now();
        const baseStepMs = 300 * 1000;
        const extra = nowMs % baseStepMs;
        const endTime = nowMs - extra - baseStepMs;
        
        let hours = 24;
        let step = baseStepMs;
        if (this.dateRange === '7d') {
            hours = 7 * 24;
            step = 6 * baseStepMs;
        }
        const startTime = endTime - hours * 12 * baseStepMs + 3 * baseStepMs;
        
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
        
        this.times = times;
        this.mins = mins;
        this.avgs = avgs;
        this.fails = fails;
        
        const opts = {
            width: rect.width,
            height: rect.height,
            legend: { show: false },
            padding: [12, 12, 0, 0],
            scales: {
                x: { time: true },
                y: {
                    range: this.fixedY ? [0, 300] : [],
                },
            },
            axes: [
                {
                    size: 26,
                    ticks: { stroke: '#aaa', width: 1, size: 6 },
                    grid: { stroke: '#aaa', width: 1, dash: [2, 2] },
                    splits: (u, ai, min, max) => {
                        const stepSmall = this.dateRange === '7d' ? 8 * 60 * 60 : 1 * 60 * 60;
                        const arr = [];
                        for (let t = Math.ceil(min / stepSmall) * stepSmall; t <= max; t += stepSmall) {
                            arr.push(t);
                        }
                        return arr;
                    },
                    values: (u, splits) => {
                        const bigStep = this.dateRange === '7d' ? 24 * 60 * 60 : 4 * 60 * 60;
                        return splits.map(t => {
                            if (t % bigStep === 0) {
                                const d = new Date(t * 1000);
                                return this.dateRange === '7d'
                                    ? `${d.getDate()}-${String(d.getHours()).padStart(2, '0')}`
                                    : d.toTimeString().slice(0, 5);
                            }
                            return '';
                        });
                    },
                },
                {
                    size: 44,
                    ticks: { stroke: '#aaa', width: 1, size: 4 },
                    grid: { stroke: '#aaa', width: 1, dash: [2, 2] },
                },
            ],
            series: [
                { label: '时间' },
                { label: '最低延迟', stroke: 'transparent', fill: 'transparent', points: { show: false } },
                { label: '平均延迟', stroke: 'transparent', fill: 'transparent', points: { show: false } },
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
                                const x = u.valToPos(xdata[i], 'x', true);
                                const y1 = u.valToPos(ymin[i], 'y', true);
                                const y2 = u.valToPos(ymax[i] + minh, 'y', true);
                                
                                let color = '#2ecc71';
                                if (this.fails[i] >= 3) color = '#ee0000';
                                else if (this.fails[i] >= 1) color = '#f1c40f';
                                
                                ctx.strokeStyle = color;
                                ctx.lineWidth = barW;
                                ctx.beginPath();
                                ctx.moveTo(x, y1);
                                ctx.lineTo(x, y2);
                                ctx.stroke();
                            } else {
                                const x = u.valToPos(xdata[i], 'x', true);
                                const y1 = u.valToPos(u.scales.y.min, 'y', true);
                                const y2 = u.valToPos(u.scales.y.max, 'y', true);
                                
                                ctx.strokeStyle = '#999';
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
                        const tt = this.tooltip;
                        if (idx == null || idx < 0 || idx >= this.times.length) {
                            tt.style.display = 'none';
                            return;
                        }
                        
                        const time = new Date(this.times[idx] * 1000);
                        const min = this.mins[idx];
                        const avg = this.avgs[idx];
                        const fail = this.fails[idx];
                        
                        tt.innerHTML = `
                            <div>${time.toLocaleString()}</div>
                            <div>最低延迟: ${min != null ? min + 'ms' : '-'}</div>
                            <div>平均延迟: ${avg != null ? avg + 'ms' : '-'}</div>
                            <div>连接失败: ${fail != null ? fail + '/20' : '-'}</div>
                        `;
                        
                        const offset = 16;
                        const cw = this.container.clientWidth;
                        const ch = this.container.clientHeight;
                        const tw = tt.offsetWidth;
                        const th = tt.offsetHeight;
                        
                        let xPos = u.cursor.left + u.over.offsetLeft + offset;
                        let yPos = u.cursor.top + u.over.offsetTop + offset;
                        
                        if (xPos + tw >= cw) xPos = xPos - tw - 2 * offset;
                        if (yPos + th >= ch) yPos = yPos - th - 2 * offset;
                        
                        tt.style.left = `${xPos}px`;
                        tt.style.top = `${yPos}px`;
                        tt.style.display = 'block';
                    },
                ],
            },
        };
        
        const uplotData = [times, mins, avgs];
        
        if (this.plot) {
            this.plot.setData(uplotData);
            this.plot.setSize({ width: rect.width, height: rect.height });
        } else {
            this.plot = new uPlot(opts, uplotData, this.container);
        }
    }
    
    setFixedY(fixed) {
        this.fixedY = fixed;
        this.loadData();
    }
    
    setIpv6(ipv6) {
        this.ipv6 = ipv6;
        this.loadData();
    }
    
    setDateRange(range) {
        this.dateRange = range;
        this.loadData();
    }
}

window.PingChart = PingChart;
