import { Link } from "react-router-dom";

const Home = () => {
  return (
    <div className="mx-auto sm:w-2/5 py-2 text-justify">
      <div>
        <p>Need your own server list? ➡️ <Link className="text-sm float-right bg-white w-5 text-center border border-gray-700" to="/server_list/">+</Link></p>
      </div>
      <div className="mt-3 prose max-w-none prose-p:my-2 prose-pre:my-2">
        <h3 className="font-bold text-lg">Usage:</h3>
        <p className="font-bold">1. Download client:</p>
        <pre><code>wget https://bench.im/dl/linux/x86_64/bim</code></pre>
        <p>or (for aarch64):</p>
        <pre><code>wget https://bench.im/dl/linux/aarch64/bim</code></pre>
        <p>and then add execute permission:</p>
        <pre><code>chmod +x bim</code></pre>
        <p className="font-bold">2. Run with a search keywords for server:</p>
        <p>Search keywords could be an id, country code(Must be all caps) or a name:</p>
        <pre><code>./bim CN</code></pre>
        <p>or an id for server list:</p>
        <pre><code>./bim -s 1</code></pre>
        <p className="font-bold">3. Threads (Optional):</p>
        <p>You can specify the number of threads to be used by the client:</p>
        <pre><code>./bim -s 1 -t 1</code></pre>
      </div>
    </div>
  )
}

export default Home;