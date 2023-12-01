use std::fs;
use std::path::Path;

/// 指定されたファイルの内容を結合
///
/// Args:
///     files: 結合するファイルのパス一覧
///     input_dir: ファイルのルートディレクトリ
///
/// Returns:
///     結合したテキスト
pub fn combine_files(files: &mut [String], input_dir: &str) -> String {
    let mut combined_text = String::new();
    files.sort();

    for file_path in files {
        let path = Path::new(file_path);
        if let Ok(text) = fs::read_to_string(path) {
            // ルートディレクトリからの相対パスを計算
            let relative_path = path
                .strip_prefix(input_dir)
                .unwrap_or(path)
                .to_string_lossy();

            // 区切り文字とファイルのパスを挿入
            combined_text.push_str(&format!("--- File: {} ---\n\n", relative_path));
            // ファイルの内容を追加
            combined_text.push_str(&text);
            // ファイル間の区切りとして改行を追加
            combined_text.push('\n');
        }
    }

    combined_text
}
