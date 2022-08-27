import { useState } from "react";
import { Link, useSearchParams, useNavigate } from "react-router-dom";
import { FormattedMessage } from "react-intl";

const Searchbar = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const [queryType, setQueryType] = useState(searchParams.get("t") ? searchParams.get("t") : "server");
  const [query, setQuery] = useState(searchParams.get("q") ? searchParams.get("q") : "");
  const navigate = useNavigate();

  const handleSearch = (e) => {
    e.preventDefault();
    navigate(`/search/?t=${queryType}&q=${query}`);
  }
  return (
    <div>
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
    </div>

  )
}

export default Searchbar;