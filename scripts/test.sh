#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_DIR="$PROJECT_ROOT/test"
TEST_DB="$TEST_DIR/test-db.sqlite3"
SERVER_PID="$TEST_DIR/server.pid"
CLIENT_PID="$TEST_DIR/client.pid"
SERVER_LOG="$TEST_DIR/server.log"
CLIENT_LOG="$TEST_DIR/client.log"
MACHINE_ID_FILE="$TEST_DIR/machine_id"
MACHINE_KEY="test-machine-key-12345"

cd "$PROJECT_ROOT"

QUICK_MODE=0
CLEANUP_ONLY=0
RESET_ONLY=0

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

cleanup() {
    log_info "正在清理测试环境..."

    if [ -f "$SERVER_PID" ]; then
        SERVER_PID_VAL=$(cat "$SERVER_PID" 2>/dev/null || true)
        if [ -n "$SERVER_PID_VAL" ] && kill -0 "$SERVER_PID_VAL" 2>/dev/null; then
            log_info "停止服务器 (PID: $SERVER_PID_VAL)..."
            kill "$SERVER_PID_VAL" 2>/dev/null || true
            wait "$SERVER_PID_VAL" 2>/dev/null || true
            if kill -0 "$SERVER_PID_VAL" 2>/dev/null; then
                kill -9 "$SERVER_PID_VAL" 2>/dev/null || true
                wait "$SERVER_PID_VAL" 2>/dev/null || true
            fi
        fi
        rm -f "$SERVER_PID"
    fi

    if [ -f "$CLIENT_PID" ]; then
        CLIENT_PID_VAL=$(cat "$CLIENT_PID" 2>/dev/null || true)
        if [ -n "$CLIENT_PID_VAL" ] && kill -0 "$CLIENT_PID_VAL" 2>/dev/null; then
            log_info "停止客户端 (PID: $CLIENT_PID_VAL)..."
            kill "$CLIENT_PID_VAL" 2>/dev/null || true
            wait "$CLIENT_PID_VAL" 2>/dev/null || true
            if kill -0 "$CLIENT_PID_VAL" 2>/dev/null; then
                kill -9 "$CLIENT_PID_VAL" 2>/dev/null || true
                wait "$CLIENT_PID_VAL" 2>/dev/null || true
            fi
        fi
        rm -f "$CLIENT_PID"
    fi

    log_success "清理完成！"
}

full_cleanup() {
    cleanup

    log_info "删除测试目录..."
    rm -rf "$TEST_DIR"

    log_success "完整清理完成！"
}

check_dependencies() {
    log_info "检查依赖..."

    if ! command -v cargo &> /dev/null; then
        log_error "未找到 cargo，请安装 Rust"
        exit 1
    fi
    log_success "Rust: $(cargo --version)"

    if ! command -v sqlite3 &> /dev/null; then
        log_error "未找到 sqlite3，请安装 sqlite3"
        exit 1
    fi
    log_success "sqlite3: $(sqlite3 --version)"
}

compile_project() {
    log_info "编译项目 (debug 模式)..."

    log_info "编译 server..."
    cargo build -p bim-server

    log_info "编译 client..."
    cargo build -p bim

    log_success "编译完成！"
}

init_database() {
    log_info "初始化数据库..."

    mkdir -p "$TEST_DIR"
    rm -f "$TEST_DB" "$TEST_DB-shm" "$TEST_DB-wal"

    log_info "运行数据库迁移..."
    DATABASE_URL="sqlite:$TEST_DB?mode=rwc" cargo run -p migration -- up

    log_success "数据库初始化完成！"
}

generate_mock_data() {
    log_info "生成模拟数据..."

    NOW=$(date +%s)
    SEVEN_DAYS_AGO=$((NOW - 7 * 24 * 60 * 60))

    MACHINE_ID=1
    echo "$MACHINE_ID" > "$MACHINE_ID_FILE"

    log_info "执行 CTE 批量数据生成..."

    sqlite3 "$TEST_DB" <<SQL
BEGIN TRANSACTION;

WITH RECURSIVE machine_series(n) AS (
    SELECT 1
    UNION ALL
    SELECT n + 1 FROM machine_series WHERE n < 11
)
INSERT INTO machine (name, ip, created, updated, key)
SELECT 
    'machine-' || printf('%02d', n),
    '192.168.' || ((n - 1) / 256) || '.' || ((n - 1) % 256),
    datetime($SEVEN_DAYS_AGO, 'unixepoch'),
    datetime($NOW, 'unixepoch'),
    '$MACHINE_KEY'
FROM machine_series;

WITH RECURSIVE target_series(n) AS (
    SELECT 6
    UNION ALL
    SELECT n + 1 FROM target_series WHERE n < 32
)
INSERT INTO target (name, domain, ipv4, ipv6, created, updated)
VALUES 
    ('百度', 'baidu.com', '110.242.68.66', '2400:da00:2::29', datetime($SEVEN_DAYS_AGO, 'unixepoch'), datetime($NOW, 'unixepoch')),
    ('腾讯', 'qq.com', '183.3.226.35', '2402:4e00:1020:1000:0:9227:4a71:30e9', datetime($SEVEN_DAYS_AGO, 'unixepoch'), datetime($NOW, 'unixepoch')),
    ('哔哩哔哩', 'bilibili.com', '110.242.68.66', '2400:da00:200a:1::1', datetime($SEVEN_DAYS_AGO, 'unixepoch'), datetime($NOW, 'unixepoch')),
    ('114DNS', '114DNS', '114.114.114.114', '2400:3800::2001:da8:1', datetime($SEVEN_DAYS_AGO, 'unixepoch'), datetime($NOW, 'unixepoch')),
    ('阿里云DNS', '阿里云DNS', '223.5.5.5', '2400:3200::1', datetime($SEVEN_DAYS_AGO, 'unixepoch'), datetime($NOW, 'unixepoch'))
UNION ALL
SELECT 
    'target-' || printf('%02d', n),
    'target' || n || '.example.com',
    '10.' || ((n - 1) / 256 / 256 % 256) || '.' || ((n - 1) / 256 % 256) || '.' || ((n - 1) % 256),
    'fd00::' || printf('%x', n),
    datetime($SEVEN_DAYS_AGO, 'unixepoch'),
    datetime($NOW, 'unixepoch')
FROM target_series;

WITH RECURSIVE time_series(ts) AS (
    SELECT $SEVEN_DAYS_AGO
    UNION ALL
    SELECT ts + 300 FROM time_series WHERE ts < $NOW
),
machine_ids AS (
    SELECT id FROM machine
),
target_ids AS (
    SELECT id FROM target
),
ip_versions AS (
    SELECT 0 AS is_ipv6 UNION ALL SELECT 1
)
INSERT INTO ping (machine_id, target_id, ipv6, created, min, avg, fail)
SELECT 
    m.id,
    t.id,
    iv.is_ipv6,
    datetime(ts.ts, 'unixepoch'),
    CASE 
        WHEN iv.is_ipv6 = 0 THEN abs(random() % 50 + 10)
        ELSE abs(random() % 60 + 15)
    END,
    CASE 
        WHEN iv.is_ipv6 = 0 THEN abs(random() % 50 + 10) + abs(random() % 30)
        ELSE abs(random() % 60 + 15) + abs(random() % 35)
    END,
    CASE 
        WHEN iv.is_ipv6 = 0 THEN abs(random() % 3)
        ELSE abs(random() % 4)
    END
FROM time_series ts
CROSS JOIN machine_ids m
CROSS JOIN target_ids t
CROSS JOIN ip_versions iv;

COMMIT;
SQL

    PING_COUNT=$(sqlite3 "$TEST_DB" "SELECT COUNT(*) FROM ping;")
    log_success "模拟数据生成完成！共 $PING_COUNT 条 ping 记录"
}

start_server() {
    log_info "启动服务器..."

    RUST_LOG=debug \
    DATABASE_URL="sqlite:$TEST_DB?mode=rwc" \
    ADMIN_PASSWORD="test-admin-123" \
    LISTEN_ADDRESS="127.0.0.1:3000" \
    SITE_NAME="BimTest" \
    ENABLE_APPLY=true \
        cargo run -p bim-server > "$SERVER_LOG" 2>&1 &
    SERVER_PID_VAL=$!
    echo $SERVER_PID_VAL > "$SERVER_PID"

    log_info "等待服务器启动..."
    TIMEOUT=10
    ELAPSED=0
    while [ $ELAPSED -lt $TIMEOUT ]; do
        if ! kill -0 $SERVER_PID_VAL 2>/dev/null; then
            log_error "服务器启动失败，查看日志: $SERVER_LOG"
            exit 1
        fi
        if command -v nc &> /dev/null; then
            if nc -z 127.0.0.1 3000 2>/dev/null; then
                break
            fi
        elif command -v python3 &> /dev/null; then
            if python3 -c "import socket; sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM); sock.settimeout(1); result = sock.connect_ex(('127.0.0.1', 3000)); sock.close(); exit(0 if result == 0 else 1)" 2>/dev/null; then
                break
            fi
        fi
        sleep 0.5
        ELAPSED=$((ELAPSED + 1))
    done

    log_success "服务器已启动 (PID: $SERVER_PID_VAL, http://127.0.0.1:3000)"
}

start_client() {
    log_info "启动客户端..."

    if [ -f "$MACHINE_ID_FILE" ]; then
        MACHINE_ID=$(cat "$MACHINE_ID_FILE")
    else
        MACHINE_ID=1
    fi

    RUST_LOG=debug \
        cargo run -p bim -- \
        --mid "$MACHINE_ID" \
        --token "$MACHINE_KEY" \
        --server_url "http://127.0.0.1:3000" \
        > "$CLIENT_LOG" 2>&1 &
    CLIENT_PID_VAL=$!
    echo $CLIENT_PID_VAL > "$CLIENT_PID"

    log_success "客户端已启动 (PID: $CLIENT_PID_VAL)"
}

print_usage() {
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  --quick     快速测试（无模拟数据）"
    echo "  --cleanup   清理所有测试遗留文件"
    echo "  --reset     仅重置数据库（保留编译缓存）"
    echo "  -h, --help  显示帮助信息"
    echo ""
    echo "默认行为: 完整测试 + 不清理"
}

parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --quick)
                QUICK_MODE=1
                shift
                ;;
            --cleanup)
                CLEANUP_ONLY=1
                shift
                ;;
            --reset)
                RESET_ONLY=1
                shift
                ;;
            -h|--help)
                print_usage
                exit 0
                ;;
            *)
                log_error "未知选项: $1"
                print_usage
                exit 1
                ;;
        esac
    done
}

main() {
    parse_args "$@"

    if [ $CLEANUP_ONLY -eq 1 ]; then
        full_cleanup
        exit 0
    fi

    if [ $RESET_ONLY -eq 1 ]; then
        cleanup
        rm -rf "$TEST_DIR"
        log_success "测试环境已重置！"
        exit 0
    fi

    trap cleanup SIGINT SIGTERM

    echo "========================================="
    echo "bench.im 测试环境"
    echo "========================================="
    echo ""

    check_dependencies
    compile_project
    init_database

    if [ $QUICK_MODE -eq 0 ]; then
        generate_mock_data
    else
        log_warn "快速模式：跳过模拟数据生成"
        sqlite3 "$TEST_DB" <<SQL
INSERT OR IGNORE INTO machine (name, ip, created, updated, key)
VALUES ('test-machine', '127.0.0.1', datetime('now'), datetime('now'), '$MACHINE_KEY');
SQL
        MACHINE_ID=$(sqlite3 "$TEST_DB" "SELECT id FROM machine WHERE name = 'test-machine';")
        echo "$MACHINE_ID" > "$MACHINE_ID_FILE"
    fi

    start_server
    start_client

    echo ""
    echo "========================================="
    echo "测试环境已就绪！"
    echo "========================================="
    echo ""
    if [ $QUICK_MODE -eq 0 ]; then
        echo "模式: 完整测试（含模拟数据）"
    else
        echo "模式: 快速测试（无模拟数据）"
    fi
    echo ""
    echo "测试目录: $TEST_DIR"
    echo "数据库: $TEST_DB"
    echo "服务器: http://127.0.0.1:3000"
    echo "管理密码: test-admin-123"
    echo ""
    echo "服务端日志: $SERVER_LOG"
    echo "客户端日志: $CLIENT_LOG"
    echo ""
    echo "查看日志命令:"
    echo "  tail -f $SERVER_LOG"
    echo "  tail -f $CLIENT_LOG"
    echo ""
    echo "按 Ctrl+C 停止服务并清理"
    echo ""

    wait
}

main "$@"
