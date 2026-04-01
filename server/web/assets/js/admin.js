// 管理后台 JavaScript

// 认证检查
document.addEventListener('DOMContentLoaded', function() {
    // 如果不是登录页，检查认证
    if (!window.location.pathname.includes('/login')) {
        const token = sessionStorage.getItem('token');
        if (!token) {
            window.location.href = '/admin/login';
        }
    }
});

// 管理员登出功能
window.logout = function() {
    sessionStorage.removeItem('token');
    window.location.href = '/admin/login';
};
