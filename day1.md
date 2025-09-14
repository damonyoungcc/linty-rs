# Day 1 - Project _linty-rs_ 🚀

## 🎯 目标

构建一个基于 **Rust** 的迷你 Linter 工具 —— **linty-rs**。
首个功能：检测 JavaScript / TypeScript 文件中是否存在 `console.log`，并输出所在位置。

---

## 📦 项目准备

- **创建仓库**：`linty-rs`
- **工具链管理**：使用 `mise` 配置 Rust 版本（`stable`）
- **依赖管理**：`Cargo.toml` 中引入：

  - [`anyhow`](https://docs.rs/anyhow) → 错误处理更优雅
  - [`clap`](https://docs.rs/clap) → 命令行参数解析

---

## 🏗️ 阶段 1 - CLI 雏形

### 已实现功能

- 支持命令：

  ```bash
  cargo run -p linty -- examples/a.js
  ```

- 读取目标文件，检查是否包含 `console.log`
- 输出结果格式模仿 ESLint：

  ```
  examples/a.js:2:1  no-console  avoid console.log
  ```

- 错误时（文件不存在等）返回友好的提示

### 技术点

- **clap**：自动生成 CLI 参数解析
- **anyhow**：错误处理，`?` 传播 + `with_context`
- **Rust 基础**：

  - `Option` / `Some` / `None`
  - `Result<T, E>` vs `anyhow::Result<T>`
  - `if let` 模式匹配
  - 遍历字符串行：`code.lines().enumerate()`

---

## 🧪 测试

- **单元测试**：对 `find_console_location` 函数进行测试，覆盖：

  - 命中首行
  - 命中多行
  - 未命中返回 `None`

- **集成测试**：用 `assert_cmd` + `assert_fs`，验证 CLI 对外输出。

---

## ⚙️ 工程化

- **Git 标签**：在完成阶段 1 后，打上 tag：

  ```bash
  git tag stage-1
  git push origin stage-1
  ```

- **CI 配置**：

  - 使用 GitHub Actions：

    - 构建、测试、clippy、fmt
    - PR 自动跑覆盖率，评论到 PR 下方

- **Pre-commit hook**：

  - 自动执行 `cargo fmt --all --check` 和 `cargo clippy --workspace`，保证提交质量。

---

## ✅ 阶段成果

- 完成 **Day 1 目标**：实现了最小可用 CLI，能检测 `console.log` 并输出行号。
- 工程环境基本搭好：测试、CI、代码风格检查。

---

## 🔜 下一步计划

- **阶段 1.5**：扩展为检测 **多个** `console.log`（而不是只报第一个）
- **阶段 2**：引入 AST 解析（`oxc_parser`），避免误报注释/字符串里的 `console.log`
- **阶段 3**：增加 `--fix` 功能，自动将 `console.log` 注释掉
