// API 请求封装
const API = {
    async fetchJSON(url, options = {}) {
        // 只在有 body 时才设置 Content-Type
        const headers = { ...options.headers };
        if (options.body) {
            headers['Content-Type'] = 'application/json';
        }

        const res = await fetch(url, {
            ...options,
            headers,
        });

        // 处理 401 未授权 - 重定向到登录页
        if (res.status === 401) {
            window.location.href = '/admin/login';
            return null;
        }

        if (!res.ok) {
            const err = new Error(res.statusText);
            err.status = res.status;
            throw err;
        }

        if (res.status === 204) return null;
        return res.json();
    }
};

// 全局 API 函数（供内联脚本使用）
window.apiFetch = API.fetchJSON;

// HTML 转义
window.escapeHtml = function(text) {
    if (!text) return '';
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
};

// 状态灯工具函数

/**
 * 根据时间差计算状态
 * @param {number} updated - 更新时间戳（秒）
 * @returns {string} - 状态标识: 'online' | 'warning' | 'offline' | 'unknown'
 */
function getStatusFromTime(updated) {
    if (!updated || updated <= 0) return 'unknown';

    const diff = Date.now() - (updated * 1000);
    if (diff < 5 * 60 * 1000) return 'online';
    if (diff < 10 * 60 * 1000) return 'warning';
    return 'offline';
}

/**
 * 设置状态灯样式（带缓存，避免不必要的 DOM 更新）
 * @param {HTMLElement} element - 状态灯元素
 * @param {string} status - 状态: 'online' | 'warning' | 'offline' | 'unknown'
 */
function setStatusLight(element, status) {
    const colorMap = {
        online: { color: 'bg-green-500', pulse: 'status-pulse-green' },
        warning: { color: 'bg-yellow-500', pulse: '' },
        offline: { color: 'bg-red-500', pulse: '' },
        unknown: { color: 'bg-neutral-300', pulse: '' }
    };

    const config = colorMap[status] || colorMap.unknown;
    const currentStatus = element.dataset.status;

    // 只有状态变化时才更新 DOM
    if (currentStatus !== status) {
        const className = `w-2 h-2 rounded-full transition-colors duration-300 ${config.color} ${config.pulse}`.trim();
        element.className = className;
        element.dataset.status = status;
    }
}

// 全局导出供内联脚本使用
window.getStatusFromTime = getStatusFromTime;
window.setStatusLight = setStatusLight;

// 导航控制
document.addEventListener('DOMContentLoaded', function() {
    // 移动端菜单切换
    const menuBtn = document.getElementById('menuBtn');
    const mainNav = document.getElementById('mainNav');

    if (menuBtn && mainNav) {
        menuBtn.addEventListener('click', function() {
            mainNav.classList.toggle('hidden');
            mainNav.classList.toggle('flex');
        });
    }

    // 管理员按钮 - 始终跳转到管理后台，由后端判断是否需要登录
    const adminBtn = document.getElementById('adminBtn');
    if (adminBtn) {
        adminBtn.addEventListener('click', function() {
            window.location.href = '/admin/';
        });
    }

    // 更新机器列表状态灯
    updateMachineStatusLights();
    setInterval(updateMachineStatusLights, 30000);
});

// 更新机器列表状态灯
async function updateMachineStatusLights() {
    try {
        const machines = await API.fetchJSON('/api/machines/');
        if (!machines) return;

        const machineMap = new Map();
        machines.forEach(m => {
            machineMap.set(m.id, m.updated || 0);
        });

        const items = document.querySelectorAll('#machinesList .machine-item');
        items.forEach(item => {
            const href = item.getAttribute('href');
            const midMatch = href.match(/\/m\/(\d+)/);
            if (!midMatch) return;

            const mid = parseInt(midMatch[1]);
            const updated = machineMap.get(mid) || 0;
            const status = getStatusFromTime(updated);

            const dot = item.querySelector('.status-light span');
            if (dot) setStatusLight(dot, status);
        });
    } catch (e) {
        // 忽略错误
    }
}
