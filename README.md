# bench.im

bench.im 是现代网络性能监控平台，由前端 Web 界面、服务器后端和监控客户端共同构成一个完整的网络监控解决方案。该平台旨在替代传统的 Smokeping，提供更直观、更易用的网络延迟与可达性可视化体验，并支持动态管理监控节点和目标。

## 项目结构

本项目采用 monorepo 架构，包含以下三个组件：

```
bench.im/
├── web/                    # 前端 Web 界面 (React + Vite)
├── server/                 # 服务器后端 (Rust + Axum + SeaORM)
└── client/                 # 监控客户端 (Rust)
```

### 组件说明

- **web/**: 网页前端，提供用户界面和数据可视化
- **server/**: 服务器后端，提供 API 服务和数据存储
- **client/**: 监控客户端，执行 ping 测试并上报数据

## 编译步骤

### 统一构建

使用提供的构建脚本同时构建所有组件：

```bash
./scripts/build.sh
```

### 单独构建

#### 前端 Web 界面

```bash
cd web
npm install
npm run build
```

#### 服务器后端

```bash
cd server
cargo build --release
```

#### 监控客户端

```bash
cd client
cargo build --release
```

## 部署方法

### 服务器后端部署

1. 编译后端：

```bash
cd server
cargo build --release
```

2. 在二进制文件同级目录创建 `.env` 文件：

```env
DATABASE_URL=sqlite:db.sqlite3?mode=rwc
ADMIN_PASSWORD=your_pass_word
```

3. 运行服务器：

```bash
./server/target/release/bim-server
```

4. 将前端构建产物放入 `static` 目录：

```bash
cp -r web/dist/* server/static/
```

### 监控客户端部署

1. 编译客户端：

```bash
cd client
cargo build --release
```

2. 运行客户端：

```bash
./client/target/release/bim -m <机器id> -t <机器密钥> -s https://bench.im
```

### systemd 部署

详细的 systemd 配置请参考各组件目录下的 README 文件。

## 开发指南

### 前端开发

```bash
cd web
npm install
npm run dev
```

### 后端开发

```bash
cd server
cargo run
```

## License

本项目采用 MIT License。
