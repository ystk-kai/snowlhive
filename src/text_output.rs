use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

/// 指定されたテキストをファイルに書き出す
///
/// Args:
///     text: 書き出すテキスト
///     output_file: 出力ファイルのパス
pub fn write_to_file(text: &str, output_file: &str) {
    // 出力ファイルのディレクトリを確認し、存在しない場合は作成
    if let Some(parent) = Path::new(output_file).parent() {
        fs::create_dir_all(parent).expect("Failed to create output directory");
    }

    let mut output =
        BufWriter::new(File::create(output_file).expect("Failed to create output file"));
    write!(output, "{}", text).expect("Failed to write to file");
}
