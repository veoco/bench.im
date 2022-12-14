import { FormattedMessage } from "react-intl";

import Searchbar from "./searchbar";

const Home = () => {
  return (
    <div>
      <Searchbar />
      <div className="mx-auto sm:w-2/5 py-2 text-justify">
        <div className="prose max-w-none prose-p:my-2 prose-pre:my-2">
          <h3 className="font-bold text-lg"><FormattedMessage defaultMessage="Usage:" /></h3>
          <p className="font-bold">1. <FormattedMessage defaultMessage="Download client:" /></p>
          <pre><code>wget https://bench.im/dl/linux/x86_64/bim</code></pre>
          <p><FormattedMessage defaultMessage="and then add execute permission:" /></p>
          <pre><code>chmod +x bim</code></pre>
          <p className="font-bold">2. <FormattedMessage defaultMessage="Run with a search keywords for server:" /></p>
          <p><FormattedMessage defaultMessage="Search keywords could be an id, country code(Must be all caps) or a name:" /></p>
          <pre><code>./bim CN</code></pre>
          <p><FormattedMessage defaultMessage="or an id for server list:" /></p>
          <pre><code>./bim -s 1</code></pre>
          <p className="font-bold">3. <FormattedMessage defaultMessage="Threads (Optional):" /></p>
          <p><FormattedMessage defaultMessage="You can specify the number of threads to be used by the client:" /></p>
          <pre><code>./bim -s 1 -t 1</code></pre>
        </div>
      </div>
    </div>
  )
}

export default Home;