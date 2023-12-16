/// 人間が読みやすいファイルサイズの文字列を解析してサイズをバイトで返す
///
/// この関数は、"KB"、"MB"、"GB"の接尾辞をサポートしており、大文字小文字を区別しません。
/// 文字列が認識された接尾辞で終わっている場合、その前の部分はu64値として解析され、
/// それから適切な1024のべき乗で乗算されます。
/// 文字列が認識された接尾辞で終わっていない場合、それは単純なu64値として解析されます。
///
/// # Arguments
///
/// * `size_str` - 人間が読みやすいファイルサイズを保持する文字列スライス。
///
/// # Returns
///
/// * `Some(u64)` - 入力文字列が有効な場合、解析されたサイズ（バイト）。
/// * `None` - 入力文字列が無効な場合（例："10 XB"）。
///
/// # Examples
///
/// ```
/// let size_in_bytes = parse_human_readable_size("10 MB");
/// assert_eq!(size_in_bytes, Some(10 * 1024 * 1024));
/// ```
pub fn parse_human_readable_size(size_str: &str) -> Option<u64> {
    let suffixes = [
        ("KB", 1024u64),
        ("MB", 1024u64 * 1024),
        ("GB", 1024u64 * 1024 * 1024),
    ];
    let size_str = size_str.trim().to_uppercase();

    for (suffix, multiplier) in suffixes.iter() {
        if size_str.ends_with(suffix) {
            let value_str = size_str.trim_end_matches(suffix).trim();
            if let Ok(value) = value_str.parse::<u64>() {
                return Some(value * multiplier);
            } else {
                return None;
            }
        }
    }

    size_str.parse::<u64>().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_human_readable_size() {
        assert_eq!(parse_human_readable_size("10 KB"), Some(10 * 1024));
        assert_eq!(parse_human_readable_size("10 MB"), Some(10 * 1024 * 1024));
        assert_eq!(parse_human_readable_size("10 GB"), Some(10 * 1024 * 1024 * 1024));
        assert_eq!(parse_human_readable_size("10"), Some(10));
        assert_eq!(parse_human_readable_size("10 XB"), None);
    }
}
