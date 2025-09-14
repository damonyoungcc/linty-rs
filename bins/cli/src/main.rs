use anyhow::{Context, Result};
use clap::Parser;
use std::fs;
mod find;
use find::find_console_location;

#[derive(Parser, Debug)]
#[command(name = "linty", version, about = "Tiny linter in Rust")]
struct Args {
    /// 要检查的文件
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    println!("args: {:?}\n", args);

    // fs::read_to_string返回值是io::Result<String>，其实就是Result<String, io::Error>
    // 要么成功返回 String，要么失败，返回io:Error
    // with_context 来自 anyhow crate，是它给 Result<T, E> 提供的一个扩展方法。
    // 如果失败，就把错误信息包装一下，额外加上提示“无法读取文件”
    let code =
        fs::read_to_string(&args.file).with_context(|| format!("无法读取文件: {}", args.file))?;

    //这里用了 if let 模式匹配
    //如果结果是 Some((a, b)) → 把里面的值绑定到 line、col
    // 如果结果是 None → 条件不成立，直接走 else
    if let Some((line, col)) = find_console_location(&code) {
        println!(
            "{}:{}:{}  no-console  avoid console.log",
            args.file, line, col
        );
    } else {
        println!("{}: ok (no console.log)", args.file);
    }

    Ok(())
}
