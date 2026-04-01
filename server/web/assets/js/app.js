// API 请求封装
const API = {
    async fetchJSON(url, options = {}) {
        const token = sessionStorage.getItem('token');
        const res = await fetch(url, {
            ...options,
            headers: {
                'Content-Type': 'application/json',
                ...(token && { 'Authorization': `Bearer ${token}` }),
                ...options.headers,
            },
        });
        
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
    
    // 管理员按钮
    const adminBtn = document.getElementById('adminBtn');
    if (adminBtn) {
        adminBtn.addEventListener('click', function() {
            const token = sessionStorage.getItem('token');
            window.location.href = token ? '/admin/' : '/admin/login';
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
            const currentTime = Date.now();
            const updatedTime = updated * 1000;
            const diff = currentTime - updatedTime;
            
            let colorClass = 'bg-gray-400';
            if (updated > 0) {
                if (diff < 5 * 60 * 1000) colorClass = 'bg-green-500';
                else if (diff < 10 * 60 * 1000) colorClass = 'bg-yellow-500';
                else colorClass = 'bg-red-500';
            }
            
            const dot = item.querySelector('.status-dot');
            if (dot) dot.className = `w-2 h-2 rounded-full mr-2 status-dot ${colorClass}`;
        });
    } catch (e) {
        // 忽略错误
    }
}
