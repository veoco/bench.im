// 管理后台 JavaScript

// 管理员登出功能
window.logout = async function() {
    try {
        // 调用后端登出 API 清除 HttpOnly Cookie
        await fetch('/admin/logout', { method: 'POST' });
    } catch (err) {
        // 忽略错误，继续跳转
    }
    window.location.href = '/admin/login';
};
