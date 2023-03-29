import { useState } from 'react'

const serversList = {
  "V.PS AMS": {
    "upload_url": "https://ams.speedtest.v.ps/empty.php",
    "download_url": "https://ams.speedtest.v.ps/garbage.php"
  },
  "V.PS AMS Nano": {
    "upload_url": "https://nano-ams.speedtest.v.ps/empty.php",
    "download_url": "https://nano-ams.speedtest.v.ps/garbage.php"
  },
  "V.PS AMS Storage": {
    "upload_url": "https://storage-ams.speedtest.v.ps/empty.php",
    "download_url": "https://storage-ams.speedtest.v.ps/garbage.php"
  },
  "V.PS FRA": {
    "upload_url": "https://fra.speedtest.v.ps/empty.php",
    "download_url": "https://fra.speedtest.v.ps/garbage.php"
  },
  "V.PS FRA Nano": {
    "upload_url": "https://nano-fra.speedtest.v.ps/empty.php",
    "download_url": "https://nano-fra.speedtest.v.ps/garbage.php"
  },
  "V.PS DUS": {
    "upload_url": "https://dus.speedtest.v.ps/empty.php",
    "download_url": "https://dus.speedtest.v.ps/garbage.php"
  },
  "V.PS TLL": {
    "upload_url": "https://tll.speedtest.v.ps/empty.php",
    "download_url": "https://tll.speedtest.v.ps/garbage.php"
  },
  "V.PS SEA": {
    "upload_url": "https://sea.speedtest.v.ps/empty.php",
    "download_url": "https://sea.speedtest.v.ps/garbage.php"
  },
  "V.PS SEA2": {
    "upload_url": "https://sea2.speedtest.v.ps/empty.php",
    "download_url": "https://sea2.speedtest.v.ps/garbage.php"
  },
  "V.PS NYC": {
    "upload_url": "https://nyc.speedtest.v.ps/empty.php",
    "download_url": "https://nyc.speedtest.v.ps/garbage.php"
  },
  "V.PS HKG": {
    "upload_url": "https://hkg.speedtest.v.ps/empty.php",
    "download_url": "https://hkg.speedtest.v.ps/garbage.php"
  },
  "V.PS LON": {
    "upload_url": "https://lon.speedtest.v.ps/empty.php",
    "download_url": "https://lon.speedtest.v.ps/garbage.php"
  },
  "V.PS KIX": {
    "upload_url": "https://kix.speedtest.v.ps/empty.php",
    "download_url": "https://kix.speedtest.v.ps/garbage.php"
  },
  "V.PS SJC": {
    "upload_url": "https://sjc.speedtest.v.ps/empty.php",
    "download_url": "https://sjc.speedtest.v.ps/garbage.php"
  },
  "V.PS NRT": {
    "upload_url": "https://nrt.speedtest.v.ps/empty.php",
    "download_url": "https://nrt.speedtest.v.ps/garbage.php"
  },
  "V.PS SYD": {
    "upload_url": "https://syd.speedtest.v.ps/empty.php",
    "download_url": "https://syd.speedtest.v.ps/garbage.php"
  },
}

export {serversList}