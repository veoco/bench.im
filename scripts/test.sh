#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_DIR="$PROJECT_ROOT/test"
TEST_DB="$TEST_DIR/test-db.sqlite3"
TEST_STATIC="$TEST_DIR/static"
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
            sleep 2
            if kill -0 "$SERVER_PID_VAL" 2>/dev/null; then
                kill -9 "$SERVER_PID_VAL" 2>/dev/null || true
            fi
        fi
        rm -f "$SERVER_PID"
    fi

    if [ -f "$CLIENT_PID" ]; then
        CLIENT_PID_VAL=$(cat "$CLIENT_PID" 2>/dev/null || true)
        if [ -n "$CLIENT_PID_VAL" ] && kill -0 "$CLIENT_PID_VAL" 2>/dev/null; then
            log_info "停止客户端 (PID: $CLIENT_PID_VAL)..."
            kill "$CLIENT_PID_VAL" 2>/dev/null || true
            sleep 2
            if kill -0 "$CLIENT_PID_VAL" 2>/dev/null; then
                kill -9 "$CLIENT_PID_VAL" 2>/dev/null || true
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

    if ! command -v node &> /dev/null; then
        log_error "未找到 node，请安装 Node.js"
        exit 1
    fi
    log_success "Node.js: $(node --version)"

    if ! command -v npm &> /dev/null; then
        log_error "未找到 npm，请安装 npm"
        exit 1
    fi
    log_success "npm: $(npm --version)"

    if ! command -v sqlite3 &> /dev/null; then
        log_error "未找到 sqlite3，请安装 sqlite3"
        exit 1
    fi
    log_success "sqlite3: $(sqlite3 --version)"
}

build_web() {
    log_info "构建 Web 前端..."

    cd "$PROJECT_ROOT/web"
    npm ci
    npm run build
    cd "$PROJECT_ROOT"

    log_success "Web 前端构建完成！"
}

prepare_static_files() {
    log_info "准备静态文件..."

    mkdir -p "$TEST_STATIC"
    rm -rf "$TEST_STATIC"/*

    if [ -d "$PROJECT_ROOT/web/dist" ]; then
        cp -r "$PROJECT_ROOT/web/dist"/* "$TEST_STATIC"/
        log_success "静态文件已复制到 $TEST_STATIC"
    else
        log_warn "web/dist 目录不存在，先构建 Web 前端"
        build_web
        cp -r "$PROJECT_ROOT/web/dist"/* "$TEST_STATIC"/
        log_success "静态文件已复制到 $TEST_STATIC"
    fi
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

    TARGETS=(
        "百度|baidu.com||110.242.68.66|"
        "腾讯|qq.com||183.3.226.35|"
        "哔哩哔哩|bilibili.com||110.242.68.66|"
        "114DNS|114DNS||114.114.114.114|"
        "阿里云DNS|阿里云DNS||223.5.5.5|"
    )

    NOW=$(date +%s)
    SEVEN_DAYS_AGO=$((NOW - 7 * 24 * 60 * 60))

    sqlite3 "$TEST_DB" <<SQL
INSERT INTO machine (name, ip, created, updated, key)
VALUES ('test-machine', '127.0.0.1', datetime($SEVEN_DAYS_AGO, 'unixepoch'), datetime($NOW, 'unixepoch'), '$MACHINE_KEY');
SQL

    MACHINE_ID=$(sqlite3 "$TEST_DB" "SELECT id FROM machine WHERE name = 'test-machine';")
    echo "$MACHINE_ID" > "$MACHINE_ID_FILE"

    for TARGET in "${TARGETS[@]}"; do
        IFS='|' read -r NAME DOMAIN IPV4 IPV6 <<< "$TARGET"

        sqlite3 "$TEST_DB" <<SQL
INSERT INTO target (name, domain, ipv4, ipv6, created, updated)
VALUES ('$NAME', '$DOMAIN', '$IPV4', '$IPV6', datetime($SEVEN_DAYS_AGO, 'unixepoch'), datetime($NOW, 'unixepoch'));
SQL

        TARGET_ID=$(sqlite3 "$TEST_DB" "SELECT id FROM target WHERE name = '$NAME';")

        log_info "生成 $NAME 的 ping 数据..."

        CURRENT_TIME=$SEVEN_DAYS_AGO
        while [ $CURRENT_TIME -le $NOW ]; do
            MIN=$((RANDOM % 50 + 10))
            AVG=$((MIN + RANDOM % 30))
            FAIL=$((RANDOM % 3))

            sqlite3 "$TEST_DB" <<SQL
INSERT INTO ping (machine_id, target_id, ipv6, created, min, avg, fail)
VALUES ($MACHINE_ID, $TARGET_ID, 0, datetime($CURRENT_TIME, 'unixepoch'), $MIN, $AVG, $FAIL);
SQL

            CURRENT_TIME=$((CURRENT_TIME + 300))
        done
    done

    PING_COUNT=$(sqlite3 "$TEST_DB" "SELECT COUNT(*) FROM ping;")
    log_success "模拟数据生成完成！共 $PING_COUNT 条 ping 记录"
}

start_server() {
    log_info "启动服务器..."

    RUST_LOG=debug \
    DATABASE_URL="sqlite:$TEST_DB?mode=rwc" \
    ADMIN_PASSWORD="test-admin-123" \
    LISTEN_ADDRESS="127.0.0.1:3000" \
    STATIC_ROOT="$TEST_STATIC" \
        cargo run -p bim-server > "$SERVER_LOG" 2>&1 &
    SERVER_PID_VAL=$!
    echo $SERVER_PID_VAL > "$SERVER_PID"

    log_info "等待服务器启动..."
    sleep 5

    if ! kill -0 $SERVER_PID_VAL 2>/dev/null; then
        log_error "服务器启动失败，查看日志: $SERVER_LOG"
        exit 1
    fi

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
    build_web
    prepare_static_files
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
    echo "静态文件: $TEST_STATIC"
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
