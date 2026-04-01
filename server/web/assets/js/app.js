// Bench.im 核心 JavaScript

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
    
    // 加载机器列表
    loadMachinesList();
});

// 加载侧边栏机器列表
async function loadMachinesList() {
    const container = document.getElementById('machinesList');
    if (!container) return;
    
    try {
        const machines = await API.fetchJSON('/api/machines/');
        
        if (!machines || machines.length === 0) {
            container.innerHTML = '<div class="px-4 py-2 text-neutral-500">暂无机器</div>';
            return;
        }
        
        container.innerHTML = machines.map(m => {
            const updated = m.updated || 0;
            const currentTime = Date.now();
            const updatedTime = updated * 1000;
            const diff = currentTime - updatedTime;
            
            let colorClass = 'bg-gray-400';
            if (updated > 0) {
                if (diff < 5 * 60 * 1000) colorClass = 'bg-green-500';
                else if (diff < 10 * 60 * 1000) colorClass = 'bg-yellow-500';
                else colorClass = 'bg-red-500';
            }
            
            return `
                <a href="/m/${m.id}" class="p-2 flex items-center border-b border-neutral-500 hover:bg-neutral-200 last:border-0">
                    <svg class="w-4 mr-2" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M4 3H20C20.5523 3 21 3.44772 21 4V11H3V4C3 3.44772 3.44772 3 4 3ZM3 13H21V20C21 20.5523 20.5523 21 20 21H4C3.44772 21 3 20.5523 3 20V13ZM7 16V18H10V16H7ZM7 6V8H10V6H7Z"></path>
                    </svg>
                    ${escapeHtml(m.name)}
                    <span class="flex items-center ml-auto">
                        <span class="w-2 h-2 rounded-full mr-2 ${colorClass}"></span>
                    </span>
                </a>
            `;
        }).join('');
        
        // 定期更新状态灯
        setInterval(updateMachineStatusLights, 30000);
        
    } catch (err) {
        container.innerHTML = '<div class="px-4 py-2 text-red-500">加载失败</div>';
    }
}

// 更新机器列表状态灯
function updateMachineStatusLights() {
    const links = document.querySelectorAll('#machinesList a');
    links.forEach(async link => {
        const mid = link.href.match(/\/m\/(\d+)/)?.[1];
        if (!mid) return;
        
        try {
            const machine = await API.fetchJSON(`/api/machines/${mid}`);
            const updated = machine.updated || 0;
            const currentTime = Date.now();
            const updatedTime = updated * 1000;
            const diff = currentTime - updatedTime;
            
            let colorClass = 'bg-gray-400';
            if (updated > 0) {
                if (diff < 5 * 60 * 1000) colorClass = 'bg-green-500';
                else if (diff < 10 * 60 * 1000) colorClass = 'bg-yellow-500';
                else colorClass = 'bg-red-500';
            }
            
            const dot = link.querySelector('.w-2');
            if (dot) dot.className = `w-2 h-2 rounded-full mr-2 ${colorClass}`;
        } catch (e) {
            // 忽略错误
        }
    });
}
