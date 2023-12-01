use ignore::{DirEntry, WalkBuilder};

/// 指定されたディレクトリ内のファイルリストを取得
///
/// Args:
///     input_dir: ファイルリストを取得するディレクトリ
///     extensions: 対象とするファイルの拡張子（カンマ区切り）
///     exclude: 除外するファイルのパターン
///
/// Returns:
///     ファイルリスト
pub fn get_file_list(
    input_dir: &str,
    extensions: &Option<String>,
    exclude: &Option<String>,
    use_gitignore: bool,
) -> Vec<String> {
    let exts = extensions.as_ref().map(|e| {
        e.split(',')
            .map(|ext| ext.trim_start_matches("*."))
            .collect::<Vec<_>>()
    });
    let exclude_pattern = exclude.as_ref().map(|s| s.as_str());

    let walker = WalkBuilder::new(input_dir)
        .git_ignore(use_gitignore)
        .build();

    walker
        .flatten()
        .filter(|entry| is_target_file(entry, &exts, &exclude_pattern))
        .map(|entry| entry.path().to_string_lossy().into_owned())
        .collect()
}

/// 指定されたファイルが対象のファイルかどうかを判定
///
/// Args:
///    entry: ファイル情報
///    exts: 対象とするファイルの拡張子
///    exclude_pattern: 除外するファイルのパターン
///
/// Returns:
///   対象のファイルの場合はtrue
fn is_target_file(
    entry: &DirEntry,
    exts: &Option<Vec<&str>>,
    exclude_pattern: &Option<&str>,
) -> bool {
    let path = entry.path();

    if !path.is_file() {
        return false;
    }

    let ext_match = exts.as_ref().map_or(true, |ext_list| {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map_or(false, |ext| ext_list.contains(&ext))
    });
    let exclude_match =
        exclude_pattern.map_or(false, |pattern| path.to_string_lossy().contains(pattern));

    ext_match && !exclude_match
}
