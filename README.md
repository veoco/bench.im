# bench.im

bench.im 是现代网络性能监控平台，由服务器后端和监控客户端共同构成一个完整的网络监控解决方案。该平台旨在替代传统的 Smokeping，提供更直观、更易用的网络延迟与可达性可视化体验，并支持动态管理监控节点和目标。

## 项目结构

本项目采用 monorepo 架构，包含以下组件：

```
bench.im/
├── server/                 # 服务器后端 (Rust + Axum + SeaORM + Askama)
│   ├── web/               # 嵌入式 Web 界面 (Tailwind CSS + Askama 模板)
│   └── ...                # API 服务和数据存储
└── client/                # 监控客户端 (Rust)
```

### 组件说明

- **server/web/**: 嵌入式 Web 前端，使用 Askama 模板引擎和 Tailwind CSS 生成，通过 rust-embed 嵌入到二进制中
- **server/**: 服务器后端，提供 API 服务、数据存储和 Web 界面
- **client/**: 监控客户端，执行 ping 测试并上报数据

## 编译步骤

### 统一构建

使用提供的构建脚本同时构建所有组件：

```bash
./scripts/build.sh
```

### 单独构建

#### 服务器后端

```bash
cd server
cargo build --release
```

Web 资源（CSS、JS、HTML 模板）会在编译时自动生成并嵌入到二进制中，无需单独构建步骤。

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
LISTEN_ADDRESS=0.0.0.0:3000
SITE_NAME=Bench.im                    # 网站名称，可选，默认 Bench.im

# 申请加入功能配置（可选）
ENABLE_APPLY=false                    # 是否启用申请加入功能，默认 false
IP2REGION_V4_DB=server/ip2region_v4.xdb   # IPv4 数据库路径，可选
IP2REGION_V6_DB=server/ip2region_v6.xdb   # IPv6 数据库路径，可选
```

3. 运行服务器：

```bash
./target/release/bim-server
```

服务器会自动提供 Web 界面（已嵌入二进制中），访问 `http://localhost:3000` 即可。

### 监控客户端部署

1. 编译客户端：

```bash
cd client
cargo build --release
```

2. 运行客户端：

```bash
./target/release/bim -m <机器id> -t <机器密钥> -s https://bench.im
```

### systemd 部署

详细的 systemd 配置请参考各组件目录下的 README 文件。

## 申请加入功能

bench.im 支持访客通过 Web 界面自助申请成为监控节点，无需管理员手动创建。

### 功能特性

- **自动命名**: 按照 `{省份}{运营商}{3位序号}` 格式自动命名（如：北京联通001）
- **智能限制**:
  - 仅限中国大陆 IP（国家识别为"中国"）
  - 每个 (省份, 运营商) 组合最多接受 3 个申请者
  - 每个 IP 地址只能有一个活跃申请
- **自动清理**: 申请者 1 天内未上线将自动删除

### 启用方法

1. 下载 ip2region 数据库文件：

```bash
# 下载 IPv4 数据库（约 10.6MB）
wget https://github.com/lionsoul2014/ip2region/raw/master/data/ip2region_v4.xdb -O server/ip2region_v4.xdb

# 下载 IPv6 数据库（约 36MB）
wget https://github.com/lionsoul2014/ip2region/raw/master/data/ip2region_v6.xdb -O server/ip2region_v6.xdb
```

2. 在 `.env` 中启用功能：

```env
ENABLE_APPLY=true
```

3. 重启服务器，访问首页点击"申请加入"按钮即可

> **注意**: 如果数据库文件不存在，服务器会正常启动但自动禁用申请功能，并在日志中输出警告。

## 开发指南

### 后端开发

```bash
cd server
cargo run
```

Web 资源的修改：
- 模板文件位于 `server/web/templates/`
- CSS 和 JS 位于 `server/web/assets/`
- Tailwind CSS 配置在 `server/web/tailwind.config.js`
- 运行 `cargo build` 时会自动重新生成 CSS

### 前端样式开发

如需修改 Tailwind CSS 样式：

```bash
cd server/web
# 手动生成 CSS（用于开发调试）
./tailwindcss -i input.css -o assets/css/app.css --watch
```

## 测试

运行完整测试环境：

```bash
./scripts/test.sh
```

快速测试（无模拟数据）：

```bash
./scripts/test.sh --quick
```

清理测试环境：

```bash
./scripts/test.sh --cleanup
```

## License

本项目采用 MIT License。
