 # Linty-RS 渐进式学习任务清单（升级版）

聚焦：循序渐进做一个可用的 JS/TS Lint & Fix 原型，逐步引入 Rust / AST / Node 生态集成。保持每一阶段都有“可运行的产出”。

---

## 总览（阶段目标一览）
1. CLI 雏形：能扫描文件里简单的 `console.log` 并报告。
2. AST 解析：用 AST 精准定位调用，减少误报。
3. Auto-fix：支持自动修复（例如注释掉 `console.log`）。
4. 第二条规则：实现 `prefer-const` 类规则。
5. Node 集成：供 Node/Vite 环境调用（模块或 CLI）。
6. 测试与演示：完善测试、快照、基准、CI。
7. 发布与扩展（可选）：配置化与 npm 发布。

---

## 阶段 1：CLI 雏形
目标：最小可用命令 `linty file.js`，读取文件并用简单字符串匹配查找 `console.log`。

关键词：Rust CLI / clap / anyhow / 文件 I/O

人话描述：做一个“能跑、能提示”的最小骨架。

技术重点：
- 命令行参数解析（`clap`）
- 文件读取（`std::fs`）
- 基础字符串搜索
- 结果输出格式（行号 / 简单诊断）

Rust 知识重点：
- 项目结构：`cargo new` / `src/main.rs`
- `Cargo.toml` 依赖管理
- 错误处理：`Result`、`?`、`anyhow`
- 基本标准库（`fs::read_to_string`）

验收标准：
- 运行 `linty examples/a.js` 输出包含文件名与匹配行。
- 非法路径返回友好错误信息。

---

## 阶段 2：AST 解析
目标：使用 `oxc_parser` 将源码解析为 ESTree 风格 AST，精准定位 `console.log` 调用。

关键词：`oxc_parser` / ESTree / `CallExpression`

技术重点：
- 解析源码 -> AST
- 遍历与匹配节点类型
- 提取函数调用信息（callee / arguments）

Rust 知识重点：
- 引用与借用（`&node` 与值所有权）
- `match` 模式匹配枚举变体
- enum / struct 组织 AST 数据
- 递归 / 迭代器遍历

验收标准：
- 能区分 `obj.console.log` 与 非相关字符串出现。
- 输出位置（行/列或 byte span）正确。

---

## 阶段 3：Auto-fix
目标：在报告的同时支持“修复”：将 `console.log(...)` 注释掉或移除。

关键词：`span` / 源码切片 / 区间替换

技术重点：
- 收集多处修改位置
- 按偏移从后往前排序替换（避免位移错乱）
- 无损编辑：基于原始源码切片重建

Rust 知识重点：
- `&str` 与 `String` 切片与所有权
- `Vec` 管理补丁集合
- 排序与迭代应用修改

验收标准：
- 提供 `--fix` 选项生成修改后的文件（或写回 / 输出到 stdout）。
- 修复后再次运行无同类诊断。

---

## 阶段 4：第二条规则（prefer-const）
目标：实现：声明为 `let` 且未被重新赋值的变量，建议改为 `const`。

关键词：`VariableDeclarator` / `AssignmentExpression` / 符号表

技术重点：
- 构建符号表记录变量 -> 赋值次数
- 作用域处理（函数 / 块）
- 简单数据流起步（统计再判断）

Rust 知识重点：
- `HashMap<String, u32>`
- 可变借用 `&mut`
- 生命周期与作用域结束时机

验收标准：
- 对未再赋值的 `let` 给出诊断（含建议）。
- 对有重新赋值的变量不误报。

---

## 阶段 5：Node 集成
目标：可供 Node/Vite 使用。两种路线择一或都做。

关键词：napi-rs（A）/ CLI + `child_process`（B）

方案 A（napi 模块）：
- 导出 `lint(source: string) -> JSON` / `fix(...)` 接口
- 性能最佳，直接内存交互

方案 B（CLI 调用）：
- 提供 `--json` 输出
- Vite 插件 `spawn`/`exec` 调用

技术重点：
- 输入输出协议（JSON Schema 简化）
- 跨平台构建（了解 cross / `napi build`）
- 性能差异简单对比（可选）

Rust 知识重点：
- `serde` 序列化 / 反序列化
- `stdin` / `stdout` 处理
- `#[napi]` 宏（若走方案 A）

验收标准：
- Node 中 `require('linty-rs')` 或 CLI 方式可以获得诊断数组。
- 能处理最少 3 个不同文件速度可接受。

---

## 阶段 6：测试与演示
目标：确保规则正确性 + 基础性能指标 + 演示输出。

关键词：`insta` / `criterion` / fixtures

技术重点：
- 快照测试（输入源码 -> JSON 结果）
- 基准测试（AST 解析 / 规则运行耗时）
- CI（GitHub Actions 基础）

Rust 知识重点：
- `#[test]` 单元测试
- `insta` 快照（`assert_snapshot!`）
- `criterion` 基准
- workspace 将 CLI / 核心库拆分

验收标准：
- `cargo test` 通过 + 快照稳定。
- 至少 1 份基准输出并可复现。

---

## 阶段 7（可选）：发布与扩展
目标：对外发布并支持规则配置。

关键词：npm 包 / 配置文件 / Vite 插件

技术重点：
- 读取 `linty.config.{json,toml}`
- 规则开关 / 严重级别控制
- npm 发布流程（含 README、版本、二进制/模块）

Rust 知识重点：
- `serde_json` / `toml` 解析
- `Option` + 默认值模式
- 打包（napi + npm scripts）

验收标准：
- npm 安装后：`npx linty-rs file.js` 正常输出。
- 可在配置中关闭某条规则并生效。

---

## 建议的执行节奏
- 每完成一个阶段：写一段 README 小结（学到什么 / 下一步）
- 严控范围：功能做到“刚好能展示”即可，再迭代增强
- 提前抽离：第 2 阶段开始将“规则”与“运行器”分离，方便扩展

## 后续可能扩展（留空位，不抢主线）
- 更多规则：未使用变量 / 空的 catch / 魔法数字
- 复杂数据流：作用域嵌套 + 暂存死区
- 增量分析：只 lint 改动的文件或 diff
- 编辑器集成：LSP（完全独立一条线）

---

## 最小 JSON 输出（示例草案）
```jsonc
[
	{
		"rule": "no-console",
		"message": "Unexpected console.log",
		"severity": "warn",
		"span": { "start": 120, "end": 135 },
		"fix": { "replacement": "// console.log(...)" }
	}
]
```

> 真实实现时可再精简；保持字段稳定有助前期集成。

---

（完）