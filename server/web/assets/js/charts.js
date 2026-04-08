// Ping 图表类
class PingChart {
    constructor(container, mid, tid, initialData = null, autoInit = true) {
        this.container = container;
        this.mid = mid;
        this.tid = tid;
        this.plot = null;
        this.tooltip = null;
        this.fixedY = false;
        this.ipv6 = false;
        this.dateRange = '24h';
        this.initialData = initialData;
        this.skeleton = container.querySelector('.chart-skeleton');

        if (autoInit) {
            this.init();
        }
    }
    
    async init() {
        // 先创建 tooltip
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

        // 如果有初始数据，直接使用；否则从 API 加载
        if (this.initialData) {
            this.renderFromData(this.initialData);
        } else {
            await this.loadData();
        }
    }
    
    // 获取容器实际可用尺寸（减去 padding）
    getContainerSize() {
        // 使用 clientWidth/clientHeight，不包含 padding
        const style = window.getComputedStyle(this.container);
        const paddingLeft = parseFloat(style.paddingLeft) || 0;
        const paddingRight = parseFloat(style.paddingRight) || 0;
        const paddingTop = parseFloat(style.paddingTop) || 0;
        const paddingBottom = parseFloat(style.paddingBottom) || 0;
        
        return {
            width: this.container.clientWidth - paddingLeft - paddingRight,
            height: this.container.clientHeight - paddingTop - paddingBottom
        };
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
    
    // 从服务端嵌入的原始数据渲染
    renderFromData(rawData) {
        // rawData 格式: [[timestamp, min, avg, fails], ...]
        const data = {
            results: rawData.map(row => [row[0], row[1], row[2], row[3]])
        };
        this.render(data);
    }
    
    render(data) {
        if (!data || !data.results) return;

        if (this.skeleton) {
            this.skeleton.remove();
            this.skeleton = null;
        }

        const size = this.getContainerSize();
        
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
        
        const self = this;
        
        const opts = {
            width: size.width,
            height: size.height,
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
                        const stepSmall = self.dateRange === '7d' ? 8 * 60 * 60 : 1 * 60 * 60;
                        const arr = [];
                        for (let t = Math.ceil(min / stepSmall) * stepSmall; t <= max; t += stepSmall) {
                            arr.push(t);
                        }
                        return arr;
                    },
                    values: (u, splits) => {
                        const bigStep = self.dateRange === '7d' ? 24 * 60 * 60 : 4 * 60 * 60;
                        return splits.map(t => {
                            if (t % bigStep === 0) {
                                const d = new Date(t * 1000);
                                return self.dateRange === '7d'
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
                        
                        const barW = Math.max(2, Math.ceil(size.width / xdata.length));
                        
                        for (let i = 0; i < xdata.length; i++) {
                            if (ymin[i] != null && ymax[i] != null) {
                                const x = u.valToPos(xdata[i], 'x', true);
                                const y1 = u.valToPos(ymin[i], 'y', true);
                                const y2 = u.valToPos(ymax[i] + minh, 'y', true);
                                
                                let color = '#2ecc71';
                                if (self.fails[i] >= 3) color = '#ee0000';
                                else if (self.fails[i] >= 1) color = '#f1c40f';
                                
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
                        const tt = self.tooltip;
                        if (idx == null || idx < 0 || idx >= self.times.length) {
                            tt.style.display = 'none';
                            return;
                        }
                        
                        const time = new Date(self.times[idx] * 1000);
                        const min = self.mins[idx];
                        const avg = self.avgs[idx];
                        const fail = self.fails[idx];
                        
                        tt.innerHTML = `
                            <div>${time.toLocaleString()}</div>
                            <div>最低延迟: ${min != null ? min + 'ms' : '-'}</div>
                            <div>平均延迟: ${avg != null ? avg + 'ms' : '-'}</div>
                            <div>连接失败: ${fail != null ? fail + '/20' : '-'}</div>
                        `;
                        
                        const offset = 16;
                        const cw = self.container.clientWidth;
                        const ch = self.container.clientHeight;
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
            this.plot.setSize({ width: size.width, height: size.height });
            // 重新应用 Y 轴范围配置（防止被 setData 重置）
            if (this.fixedY) {
                this.plot.setScale('y', { min: 0, max: 300 });
            }
        } else {
            this.plot = new uPlot(opts, uplotData, this.container);
        }
    }
    
    setFixedY(fixed) {
        this.fixedY = fixed;
        // 使用 setScale 动态更新 Y 轴范围，不销毁图表
        if (this.plot) {
            const yRange = fixed ? [0, 300] : [];
            this.plot.setScale('y', { min: yRange[0], max: yRange[1] });
        }
    }
    
    async setIpv6(ipv6) {
        this.ipv6 = ipv6;
        // 保持旧图表可见，等待新数据到达后平滑更新
        await this.loadData();
    }
    
    async setDateRange(range) {
        this.dateRange = range;
        // 保持旧图表可见，等待新数据到达后平滑更新
        await this.loadData();
    }
}

// 用于 target 页面的图表类（按目标查看所有机器）
class TargetPingChart extends PingChart {
    async loadData() {
        try {
            const url = `/api/targets/${this.tid}/machines/${this.mid}/${this.dateRange}` + 
                       (this.ipv6 ? '?ipv6=true' : '');
            const data = await apiFetch(url);
            this.render(data);
        } catch (err) {
            console.error('Failed to load ping data:', err);
        }
    }
}

window.PingChart = PingChart;
window.TargetPingChart = TargetPingChart;
