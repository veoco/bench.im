import { useState } from 'react'

const serversList = {
  "V.PS, Amsterdam, Netherlands": {
    "upload_url": "https://ams.speedtest.v.ps/empty.php",
    "download_url": "https://ams.speedtest.v.ps/garbage.php"
  },
  "V.PS, Amsterdam, Netherlands, Nano": {
    "upload_url": "https://nano-ams.speedtest.v.ps/empty.php",
    "download_url": "https://nano-ams.speedtest.v.ps/garbage.php"
  },
  "V.PS, Amsterdam, Netherlands, Storage": {
    "upload_url": "https://storage-ams.speedtest.v.ps/empty.php",
    "download_url": "https://storage-ams.speedtest.v.ps/garbage.php"
  },
  "V.PS, Frankfurt, Germany": {
    "upload_url": "https://fra.speedtest.v.ps/empty.php",
    "download_url": "https://fra.speedtest.v.ps/garbage.php"
  },
  "V.PS, Frankfurt, Germany, Nano": {
    "upload_url": "https://nano-fra.speedtest.v.ps/empty.php",
    "download_url": "https://nano-fra.speedtest.v.ps/garbage.php"
  },
  "V.PS, Düsseldorf, Germany": {
    "upload_url": "https://dus.speedtest.v.ps/empty.php",
    "download_url": "https://dus.speedtest.v.ps/garbage.php"
  },
  "V.PS, Tallinn, Estonia": {
    "upload_url": "https://tll.speedtest.v.ps/empty.php",
    "download_url": "https://tll.speedtest.v.ps/garbage.php"
  },
  "V.PS, Seattle, United States": {
    "upload_url": "https://sea.speedtest.v.ps/empty.php",
    "download_url": "https://sea.speedtest.v.ps/garbage.php"
  },
  "V.PS, Seattle 2, United States": {
    "upload_url": "https://sea2.speedtest.v.ps/empty.php",
    "download_url": "https://sea2.speedtest.v.ps/garbage.php"
  },
  "V.PS, New York, United States": {
    "upload_url": "https://nyc.speedtest.v.ps/empty.php",
    "download_url": "https://nyc.speedtest.v.ps/garbage.php"
  },
  "V.PS, Hong Kong, China": {
    "upload_url": "https://hkg.speedtest.v.ps/empty.php",
    "download_url": "https://hkg.speedtest.v.ps/garbage.php"
  },
  "V.PS, London, United Kingdom": {
    "upload_url": "https://lon.speedtest.v.ps/empty.php",
    "download_url": "https://lon.speedtest.v.ps/garbage.php"
  },
  "V.PS, Osaka, Japan": {
    "upload_url": "https://kix.speedtest.v.ps/empty.php",
    "download_url": "https://kix.speedtest.v.ps/garbage.php"
  },
  "V.PS, San Jose, United States": {
    "upload_url": "https://sjc.speedtest.v.ps/empty.php",
    "download_url": "https://sjc.speedtest.v.ps/garbage.php"
  },
  "V.PS, Tokyo, Japan": {
    "upload_url": "https://nrt.speedtest.v.ps/empty.php",
    "download_url": "https://nrt.speedtest.v.ps/garbage.php"
  },
  "V.PS, Sydney, Australia": {
    "upload_url": "https://syd.speedtest.v.ps/empty.php",
    "download_url": "https://syd.speedtest.v.ps/garbage.php"
  },
}

export {serversList}