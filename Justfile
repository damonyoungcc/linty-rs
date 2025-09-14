# 使用 bash 执行
set shell := ["bash", "-cu"]

# ===== 基础 =====
# 一键初始化：按项目记录安装 rust、常用组件、构建验证
setup:
    @echo "==> mise install (根据 mise.toml/.tool-versions 安装)"
    mise install
    @echo "==> rustup components"
    rustup component add rustfmt clippy || true
    @echo "==> cargo check"
    cargo check

# 构建（workspace）
build:
    cargo build

# 只检查（不生成产物）
check:
    cargo check

# 清理
clean:
    cargo clean

# ===== 代码质量 =====
# 格式化
fmt:
    cargo fmt

# 静态检查（当作 lint 使用）
lint:
    cargo clippy --all-targets -- -D warnings

# 组合：格式化 + lint
qa: fmt lint

# ===== 运行 CLI =====
# 运行 CLI（必填：FILE）
# 用法：just run /path/to/file.js
run FILE:
    cargo run -p linty -- {{FILE}}

# 带 --fix 的运行
# 用法：just fix /path/to/file.js
fix FILE:
    cargo run -p linty -- {{FILE}} --fix

# 热编译运行（需要 cargo-watch：`cargo install cargo-watch`）
# 用法：just dev /path/to/file.js
dev FILE:
    cargo watch -w bins/cli -w crates/core -x "run -p linty -- {{FILE}}"

# ===== 测试 / 基准（占位，后续补齐） =====
test:
    cargo test -p linty

bench:
    cargo bench

# ===== 便利别名 =====
# 默认目标：显示帮助
default: help

help:
    @echo "Just commands:"
    @echo "  just setup          # 一键初始化（mise install + rustfmt/clippy + check）"
    @echo "  just build          # 构建 workspace"
    @echo "  just check          # 类型检查"
    @echo "  just clean          # 清理"
    @echo "  just fmt            # 格式化"
    @echo "  just lint           # clippy 静态检查（当作 lint）"
    @echo "  just qa             # fmt + lint"
    @echo "  just run FILE       # 运行 CLI：linty FILE"
    @echo "  just fix FILE       # 运行 CLI（自动修复）：linty FILE --fix"
    @echo "  just dev FILE       # 监听改动并自动重跑（需 cargo-watch）"
    @echo "  just test           # 单元测试（后续补）"
    @echo "  just bench          # 基准测试（后续补）"
