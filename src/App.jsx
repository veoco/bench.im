import { useState, useEffect } from "react";
import { Routes, Route, Link } from "react-router-dom";
import { IntlProvider, FormattedMessage } from "react-intl";

import Home from "./components/home"
import Search from "./components/search";
import Server from "./components/server";
import ServerCreate from "./components/server_create";
import ServerList from "./components/server_list";
import ServerListCreate from "./components/server_list_create";
import Login from "./components/login";
import Signup from "./components/signup";
import My from "./components/my";
import Machine from "./components/machine";
import MachineTaskCreate from "./components/machine_task_create";
import MachineTask from "./components/machine_task";
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
  const [locale, setLocal] = useState(navigator.language.split('-')[0]);
  const [messages, setMessages] = useState(loadLocaleData(locale));

  useEffect(() => {
    setMessages(loadLocaleData(locale));
  })


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
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/search/" element={<Search />} />
          <Route path="/server/" element={<ServerCreate />} />
          <Route path="/server/:serverId/" element={<Server />} />
          <Route path="/server_list/" element={<ServerListCreate />} />
          <Route path="/server_list/:serverListId/" element={<ServerList />} />
          <Route path="/machine/:uuid/" element={<Machine />} />
          <Route path="/machine/:uuid/task/" element={<MachineTaskCreate />} />
          <Route path="/machine/:uuid/task/:taskId/" element={<MachineTask />} />
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
