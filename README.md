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
SERVER_URL=https://your-server.com    # 服务器URL，用于申请加入功能邮件等，可选

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

#### 环境变量说明

| 变量名 | 必填 | 默认值 | 说明 |
|--------|------|--------|------|
| `DATABASE_URL` | 是 | - | SQLite 数据库连接字符串 |
| `ADMIN_PASSWORD` | 是 | fake-admin-password | 管理员登录密码（请务必修改） |
| `LISTEN_ADDRESS` | 否 | 127.0.0.1:3000 | 服务器监听地址和端口 |
| `SITE_NAME` | 否 | Bench.im | 网站显示名称 |
| `SERVER_URL` | 否 | https://your-server.fake-url | 服务器公网 URL，用于申请加入功能 |
| `ENABLE_APPLY` | 否 | false | 是否启用申请加入功能 |
| `IP2REGION_V4_DB` | 否 | server/ip2region_v4.xdb | IPv4 地理位置数据库路径 |
| `IP2REGION_V6_DB` | 否 | server/ip2region_v6.xdb | IPv6 地理位置数据库路径 |
| `TRUSTED_PROXIES` | 否 | - | 可信代理 IP 列表（见下方说明）|

#### TRUSTED_PROXIES 配置说明

用于防止 IP 伪造攻击，控制服务器如何获取客户端真实 IP。

**场景 1：直接暴露在互联网（默认）**
```env
# 不设置或留空，服务器始终使用 TCP 连接地址
TRUSTED_PROXIES=
```

**场景 2：使用 Nginx/CDN 反向代理**
```env
# 配置可信代理 IP 地址，支持单个 IP 或 CIDR
TRUSTED_PROXIES=127.0.0.1,10.0.0.0/8,172.16.0.0/12,::1
```

**配置规则：**
- 不设置或为空：始终使用 TCP 连接地址（最安全，适用于直接暴露）
- 设置可信代理：仅当请求来自这些 IP 时才信任 `X-Forwarded-For` 头
- 支持格式：`192.168.1.1`（单个 IP）或 `10.0.0.0/8`（CIDR）
- 多个值用逗号分隔

**安全风险：**
错误配置此选项可能导致 IP 伪造攻击，例如攻击者通过伪造 `X-Forwarded-For` 头绕过地理限制申请成为监控节点。

#### HTTPS 与 Cookie Secure 标志

后台管理登录使用 Cookie 存储认证信息。为了提高安全性，当检测到 HTTPS 连接时，Cookie 会自动设置 `Secure` 标志（仅通过 HTTPS 传输）。

**自动检测（无需配置）：**
- Caddy、Traefik、Cloudflare 等反向代理会自动添加 `X-Forwarded-Proto: https` 头，服务器自动识别
- 本地 HTTP 开发环境不设置 `Secure`，可正常登录

**Nginx 用户需要手动添加配置：**

```nginx
server {
    listen 443 ssl;
    server_name your-domain.com;

    location / {
        proxy_pass http://localhost:3000;

        # 必须添加这一行，否则 Cookie 不会设置 Secure
        proxy_set_header X-Forwarded-Proto $scheme;

        # 其他可选头
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header Host $host;
    }
}
```

**注意事项：**
- 如果通过反向代理访问管理后台时发现无法登录（Cookie 问题），请检查是否正确传递了 `X-Forwarded-Proto` 头
- 直接暴露 HTTP（不通过反向代理）时，Cookie 不会设置 Secure，适合本地开发

#### systemd 服务配置

创建 `/etc/systemd/system/bim-server.service`：

```ini
[Unit]
Description=bim-server
After=network.target

[Service]
WorkingDirectory=/your_path
ExecStart=/your_path/bim-server
User=your_user
Group=your_group
Restart=always
RestartSec=3

[Install]
WantedBy=multi-user.target
```

启用并启动服务：

```bash
sudo systemctl daemon-reload
sudo systemctl enable bim-server
sudo systemctl start bim-server
```

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

#### 参数说明

| 参数 | 必填 | 说明 |
|------|------|------|
| `-m, --mid` | 是 | 机器 ID，从服务器后台获取 |
| `-t, --token` | 是 | 机器密钥，从服务器后台获取 |
| `-s, --server_url` | 是 | 服务器 URL，如 https://bench.im |
| `-h, --help` | 否 | 显示帮助信息 |

#### systemd 服务配置

创建 `/etc/systemd/system/bim.service`：

```ini
[Unit]
Description=bim
After=network.target

[Service]
ExecStart=/your_path/bim -m <机器id> -t <机器密钥> -s https://bench.im
Restart=always
RestartSec=3
DynamicUser=true
AmbientCapabilities=CAP_NET_RAW
CapabilityBoundingSet=CAP_NET_RAW
NoNewPrivileges=false

[Install]
WantedBy=multi-user.target
```

> **注意**: 客户端需要 `CAP_NET_RAW` 权限来发送 ICMP ping 包。

启用并启动服务：

```bash
sudo systemctl daemon-reload
sudo systemctl enable bim
sudo systemctl start bim
```

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

本项目采用 GNU General Public License v2.0。
