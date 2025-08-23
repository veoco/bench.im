# bench.im

bench.im 是现代网络性能监控平台的前端 Web 界面，与 bim-berver 和 bim 客户端共同构成一个完整的网络监控解决方案。该平台旨在替代传统的 Smokeping，提供更直观、更易用的网络延迟与可达性可视化体验，并支持动态管理监控节点和目标。

## 项目概述

网页前端（本项目）： https://github.com/veoco/bench.im

服务器后端：https://github.com/veoco/bim-server

监控客户端：https://github.com/veoco/bim

## 编译步骤

```
git clone https://github.com/veoco/bim.git
cd bim
npm i
npm build
```

## 部署方法

请将 index.html 和 assets 放入 bim-server 文件同级的 static 下。