/// 返回第一个 `console.log` 的 (行号, 列号)，1-based。
/*
Option 是一个 枚举类型
enum Option<T> {
    Some(T),
    None,
}
*/
pub fn find_console_location(code: &str) -> Option<(usize, usize)> {
    for (i, line) in code.lines().enumerate() {
        if let Some(byte_idx) = line.find("console.log") {
            // 列号用“字符数”而不是字节，避免中文/Emoji 误差
            let col = line[..byte_idx].chars().count() + 1;
            return Some((i + 1, col));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::find_console_location;

    #[test]
    fn finds_first_match_on_first_line() {
        let code = r#"console.log("A")"#;
        assert_eq!(find_console_location(code), Some((1, 1)));
    }

    #[test]
    fn finds_first_match_on_third_line() {
        let code = r#"let x = 1;
let y = 2;
  console.log("B");"#;
        // 两个空格后出现，列号 = 3
        assert_eq!(find_console_location(code), Some((3, 3)));
    }

    #[test]
    fn returns_none_when_not_found() {
        let code = r#"let x = 1; /* no console here */"#;
        assert_eq!(find_console_location(code), None);
    }

    #[test]
    fn counts_multibyte_chars_correctly() {
        // 中文与 emoji 占多字节，但 chars().count() 会算成1个字符
        let code = "前面有中文🙂  console.log(1)";
        // “console.log” 前面有 “前面有中文🙂⎵⎵” 共 6 个可见字符 + 两个空格 = 9
        assert_eq!(find_console_location(code), Some((1, 9)));
    }
}
