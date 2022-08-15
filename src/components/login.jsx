import { useState, useEffect } from "react";
import { useNavigate, Link } from "react-router-dom";
import { FormattedMessage } from "react-intl";

const Login = () => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const navigate = useNavigate();

  useEffect(() => {
    document.title = `Login - Bench.im`;
    const logined = new Date(localStorage.getItem('logined'));
    const now = new Date();
    if ((now - logined) < 14 * 86400000) {
      navigate(`/my/`);
    } else {
      if (localStorage.getItem('logined')) {
        localStorage.removeItem('logined');
      }
    }
  });

  const handleSubmit = async (e) => {
    e.preventDefault();
    const data = {
      "email": email,
      "password": password
    }
    const r = await fetch("/api/login/", {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data)
    })
    if (!r.ok) {
      if (r.status == 400) {
        const res = await r.json();
        alert(`Invalid: ${res.msg}`);
        return;
      }
      alert("Server Error! Please refresh the page and try again.")
      return;
    }
    const now = new Date();
    localStorage.setItem('logined', now.toUTCString());
    navigate(`/my/`);
  }

  return (
    <div className="mx-auto sm:w-2/5 text-justify">
      <div className="text-center text-2xl underline my-2"><FormattedMessage defaultMessage="Login" /></div>
      <form className="leading-8" onSubmit={handleSubmit}>
        <label><FormattedMessage defaultMessage="Email:" /></label><br />
        <input className="w-full" type="text" value={email} onChange={(e) => { setEmail(e.target.value) }} />
        <label><FormattedMessage defaultMessage="Password:" /></label><br />
        <input className="w-full" type="password" value={password} onChange={(e) => { setPassword(e.target.value) }} />
        <Link className="my-2 underline" to="/signup/"><FormattedMessage defaultMessage="Sign up" /></Link>
        <button className="float-right bg-white border border-gray-700 px-2 my-2"><FormattedMessage defaultMessage="Submit" /></button>
      </form>
    </div>
  )
}

export default Login;