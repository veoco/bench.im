import { useEffect } from "react";
import { Link, useSearchParams, useNavigate } from "react-router-dom";
import useSWR from 'swr'

import ServerItem from "./server_item";
import ServerListItem from "./server_list_item";


const Search = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const { data, error } = useSWR(`/api/search/?type=${searchParams.get("t")}&query=${searchParams.get("q")}`)

  const isServerList = searchParams.get("t") == "server_list";
  const serverListDiv = (
    <Link className="text-sm float-right bg-white w-5 text-center border border-gray-700" to="/server_list/">+</Link>
  );

  useEffect(() => {
    document.title = `Search - Bench.im`;
  });

  if (error || !data) {
    return (
      <div>
        <div className="mx-auto sm:w-2/5 py-2 text-justify">
          <p>Found 0 results for "{searchParams.get("q")}" {isServerList?serverListDiv:""}</p>
        </div>
      </div>
    )
  }

  return (
    <div>
      <div className="mx-auto sm:w-2/5 py-2 text-justify">
        <p>Found {data.count} results for "{searchParams.get("q")}" {isServerList?serverListDiv:""}</p>
        {data.results.map((item) => {
          if (searchParams.get("t") == "server") {
            return (
              <ServerItem item={item} key={item.pk} />
            )
          } else if (isServerList) {
            return (
              <ServerListItem item={item} key={item.pk} />
            )
          }
        })}
      </div>
    </div>
  )
}

export default Search;