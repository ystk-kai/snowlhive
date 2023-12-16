use std::fs;
use std::path::Path;

/// 指定されたファイルの内容を結合し、必要に応じて指定された最大サイズに達した場合に新しいファイルを開始
///
/// Args:
///     files: 結合するファイルのパス一覧
///     input_dir: ファイルのルートディレクトリ
///     max_size: 結合するテキストの最大サイズ（バイト単位、オプション）
///
/// Returns:
///     結合したテキストのベクター。サイズ制限が指定されていない場合はベクターに1つの要素のみが含まれる
pub fn combine_files(files: &mut [String], input_dir: &str, max_size: Option<u64>) -> Vec<String> {
    let mut combined_texts = Vec::new();
    let mut current_text = String::new();
    files.sort();

    for file_path in files {
        let path = Path::new(file_path);
        if let Ok(text) = fs::read_to_string(path) {
            let relative_path = path
                .strip_prefix(input_dir)
                .unwrap_or(path)
                .to_string_lossy();

            let text_with_header = format!("--- File: {} ---\n\n{}", relative_path, text);

            // サイズ制限が指定されている場合、サイズをチェックする
            if let Some(max_size) = max_size {
                if current_text.len() + text_with_header.len() > max_size as usize {
                    combined_texts.push(current_text);
                    current_text = String::new();
                }
            }

            current_text.push_str(&text_with_header);
        }
    }

    if !current_text.is_empty() {
        combined_texts.push(current_text);
    }

    combined_texts
}
