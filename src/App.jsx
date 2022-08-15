import { useState, useEffect } from "react";
import { Routes, Route, Link, useSearchParams, useNavigate } from "react-router-dom";
import { IntlProvider, FormattedMessage } from "react-intl";

import Home from "./components/home"
import Search from "./components/search";
import ServerList from "./components/server_list";
import ServerListCreate from "./components/server_list_create";
import Login from "./components/login";
import Signup from "./components/signup";
import My from "./components/my";
import NotFound from "./components/404";
import ZH from "../compiled-lang/zh.json"
import EN from "../compiled-lang/en.json"

function loadLocaleData(locale) {
  switch (locale) {
    case "zh":
      return ZH
    default:
      return EN
  }
}


function App(props) {
  const [searchParams, setSearchParams] = useSearchParams();
  const [queryType, setQueryType] = useState(searchParams.get("t") ? searchParams.get("t") : "server");
  const [query, setQuery] = useState(searchParams.get("q") ? searchParams.get("q") : "");
  const [locale, setLocal] = useState(navigator.language.split('-')[0]);
  const [messages, setMessages] = useState(loadLocaleData(locale));
  const navigate = useNavigate();

  useEffect(() => {
    setMessages(loadLocaleData(locale));
  })

  const handleSearch = (e) => {
    e.preventDefault();
    navigate(`/search/?t=${queryType}&q=${query}`);
  }

  return (
    <IntlProvider
      locale={locale}
      defaultLocale="en"
      messages={messages}
    >
      <div className="bg-stone-100 min-h-screen px-1 sm:px-0">
        <h1 className="text-center text-4xl font-serif py-2 underline text-gray-600"><Link to="/">Bench.im</Link></h1>
        <nav className="text-center">
          <button className={"mx-2" + (locale == "zh" ? "" : " text-gray-400")} onClick={(e) => { setLocal("zh") }}>中文</button>
          <button className={"mx-2" + (locale == "en" ? "" : " text-gray-400")} onClick={(e) => { setLocal("en") }}>English</button>
        </nav>
        <div className="text-center my-2">
          <p><FormattedMessage defaultMessage="Download:" /></p>
          <nav className="underline">
            <Link className="mx-2" to="/dl/linux/x86_64/bim" reloadDocument>x86_64</Link>
            <Link className="mx-2" to="/dl/linux/aarch64/bim" reloadDocument>aarch64</Link>
          </nav>
        </div>
        <form className="mx-auto sm:w-2/5" onSubmit={handleSearch}>
          <select className="w-3/12 sm:w-3/12 border-r-0 ring-transparent focus:ring-0 focus:border-gray-700" value={queryType} onChange={(e) => { setQueryType(e.target.value) }}>
            <option value="server"><FormattedMessage defaultMessage="Server" /></option>
            <option value="server_list"><FormattedMessage defaultMessage="Server List" /></option>
          </select>
          <input className="w-7/12 sm:w-8/12 border-x-0 focus:ring-0 focus:border-gray-700" type="text" placeholder={queryType == "server" ? "id, country code or name" : "id, name or readme"} value={query} onChange={(e) => { setQuery(e.target.value) }} />
          <button className="w-1/6 sm:w-1/12 border border-l-0 border-gray-700 bg-white p-2" type="submit">🔍</button>
        </form>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/search/" element={<Search />} />
          <Route path="/server_list/" element={<ServerListCreate />} />
          <Route path="/server_list/:serverListId/" element={<ServerList />} />
          <Route path="/login/" element={<Login />} />
          <Route path="/signup/" element={<Signup />} />
          <Route path="/my/" element={<My />} />
          <Route path="*" element={<NotFound />} />
        </Routes>
      </div>
    </IntlProvider>
  )
}

export default App
