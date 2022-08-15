import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";
import { FormattedMessage } from "react-intl";

const Signup = () => {
  const [username, setUsername] = useState('');
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [password2, setPassword2] = useState('');
  const navigate = useNavigate();

  useEffect(() => {
    document.title = `Sign up - Bench.im`;
    const logined = new Date(localStorage.getItem('logined'));
    const now = new Date();
    if ((now - logined) < 14 * 86400000) {
      navigate(`/my/`);
    }
  });

  const handleSubmit = async (e) => {
    e.preventDefault();
    if (password != password2) {
      alert("The two passwords do not match.");
      return;
    }
    const data = {
      "username": username,
      "email": email,
      "password": password
    }
    const r = await fetch("/api/signup/", {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(data)
    })
    if (!r.ok) {
      if (r.status == 400) {
        const res = await r.json();
        let msg = "";
        for (let k in res.msg) {
          msg += k + " - " + res.msg[k];
        }
        alert(`Invalid: ${msg}`);
        return;
      }
      alert("Server Error! Please refresh the page and try again.")
      return;
    }
    navigate(`/login/`);
  }

  return (
    <div className="mx-auto sm:w-2/5 text-justify">
      <div className="text-center text-2xl underline my-2">Sign up</div>
      <form className="leading-8" onSubmit={handleSubmit}>
        <label><FormattedMessage defaultMessage="Username:" /></label><br />
        <input className="w-full" type="text" value={username} onChange={(e) => { setUsername(e.target.value) }} />
        <label><FormattedMessage defaultMessage="Email:" /></label><br />
        <input className="w-full" type="text" value={email} onChange={(e) => { setEmail(e.target.value) }} />
        <label><FormattedMessage defaultMessage="Password:" /></label><br />
        <input className="w-full" type="password" value={password} onChange={(e) => { setPassword(e.target.value) }} />
        <label><FormattedMessage defaultMessage="Password Again:" /></label><br />
        <input className="w-full" type="password" value={password2} onChange={(e) => { setPassword2(e.target.value) }} />
        <button className="float-right bg-white border border-gray-700 px-2 my-2"><FormattedMessage defaultMessage="Submit" /></button>
      </form>
    </div>
  )
}

export default Signup;