use anyhow::{Context, Result};
use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(name = "linty", version, about = "Tiny linter in Rust")]
struct Args {
    /// 要检查的文件
    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let code =
        fs::read_to_string(&args.file).with_context(|| format!("无法读取文件: {}", args.file))?;

    if code.contains("console.log") {
        println!("{}:1:1  no-console  avoid console.log", args.file);
    } else {
        println!("{}: ok (no console.log)", args.file);
    }

    Ok(())
}
