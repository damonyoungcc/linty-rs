## just 工具执行 Justfile

```bash
just run fixtures/a.js
```

- just 是一个命令运行器，类似 Makefile 的现代替代品。
- 它读取项目根目录的 Justfile，找到 run FILE: 这个任务。
- 实际执行的是

```bash
cargo run -p linty -- fixtures/a.js
```

## cargo 接手

cargo 是 Rust 的官方构建工具 & 包管理器。

- cargo run -p linty -- fixtures/a.js 做的事

1. 看根目录 Cargo.toml → 确认依赖
2. 本地缓存里有没有？

- 有 → 用
- 没有 → 从 crates.io 下载到 ~/.cargo/registry

3. linty 的 main.rs 用 clap 解析命令行参数
4. 第一个参数 = fixtures/a.js。
5. 读取文件内容（fs::read_to_string）。
6. 用简单的字符串搜索 (contains("console.log")) 来检查是否包含。
7. 编译依赖 + 你的代码 → 放到 target/debug/
8. 运行生成的二进制
