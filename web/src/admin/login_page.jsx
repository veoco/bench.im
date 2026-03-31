import { useState, useEffect } from "react";
import { useLocation } from "wouter";

export default function LoginPage({ isLogin, setIsLogin }) {
  const [location, setLocation] = useLocation();
  const [password, setPassword] = useState("");

  useEffect(() => {
    if (isLogin) {
      setLocation("/");
    }
  }, [isLogin]);

  const handleSubmit = async (e) => {
    e.preventDefault();

    const r = await fetch(`/api/admin/machines/`, {
      method: 'GET',
      headers: {
        'Authorization': `Bearer ${password}`,
      },
    })
    if (r.status != 200) {
      alert('密码错误')
      return;
    }

    sessionStorage.setItem("token", password);
    setIsLogin(true);
    setLocation("/");
  }

  return (
    <div className="flex justify-center p-2 items-center sm:h-screen">
      <form className="max-w-sm w-full mt-6 border border-neutral-400 bg-neutral-100 p-3 sm:mt-0" onSubmit={handleSubmit}>
        <h2 className="text-center text-lg font-bold py-1">管理登录</h2>
        <input className="border my-3 p-2 w-full" type="password" value={password} onChange={(e) => setPassword(e.target.value)} />
        <button className="w-full p-2 button" type="submit">登录</button>
      </form>
    </div>
  )
}