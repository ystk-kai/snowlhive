mod file_list;
mod size_utils;
mod text_cleaner;
mod text_combiner;
mod text_output;

use clap::Parser;
use humansize::{format_size, DECIMAL};
use std::path::Path;
use tracing::{debug, info, Level};

/// Aggregates document information into a single file
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Sets the input directory to use
    #[clap(value_parser)]
    input_dir: String,

    /// Sets the output directory
    #[clap(
        short = 'd',
        long = "output-dir",
        value_parser,
        default_value = "./output"
    )]
    output_dir: String,

    /// Sets the target file extensions, separated by commas
    #[clap(short = 'x', long, value_parser, default_value = "*.md,*.mdx")]
    extensions: String,

    /// Sets the pattern to exclude files
    #[clap(short = 'e', long, value_parser)]
    exclude: Option<String>,

    /// Sets the output file name
    #[clap(short = 'n', long = "output-name", value_parser)]
    output_name: Option<String>,

    /// Enable verbose output
    #[clap(long)]
    verbose: bool,

    /// Remove Markdown code blocks
    #[clap(long = "remove-codeblock")]
    remove_codeblock: bool,

    /// Remove URLs
    #[clap(long = "remove-url")]
    remove_url: bool,

    /// Disable ignoring files specified in .gitignore
    #[clap(long = "disabled-gitignore")]
    disabled_gitignore: bool,

    /// Sets the maximum file size for output files
    #[clap(short = 's', long = "max-size", value_parser)]
    max_size: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    // Loggerの初期化
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_max_level(Level::TRACE)
            .init();
    } else {
        tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    }

    // 出力ファイル名が指定されていない場合、input_dir のディレクトリ名を使用
    let output_name = cli.output_name.unwrap_or_else(|| {
        Path::new(&cli.input_dir)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
            + ".txt"
    });

    debug!("Output file name: {}", output_name);

    // ファイル一覧を取得
    let mut files = file_list::get_file_list(
        &cli.input_dir,
        &Some(cli.extensions),
        &cli.exclude,
        !cli.disabled_gitignore,
    );

    debug!("File count: {}", files.len());
    debug!("Output directory: {}", cli.output_dir);

    if !files.is_empty() && cli.verbose {
        for file in &files {
            debug!("File: {}", file);
        }
    }

    // クリーニングオプションの定義
    let cleaning_options = text_cleaner::CleaningOptions {
        remove_codeblock: cli.remove_codeblock,
        remove_url: cli.remove_url,
    };

    // ファイルを結合
    let combined_texts = if let Some(max_size_str) = cli.max_size {
        let max_size =
            size_utils::parse_human_readable_size(&max_size_str).expect("Invalid size format");
        text_combiner::combine_files(&mut files, &cli.input_dir, Some(max_size))
    } else {
        text_combiner::combine_files(&mut files, &cli.input_dir, None)
    };

    // 結合されたテキストの数を取得
    let num_texts = combined_texts.len();

    // 結合したテキストのベクターに対してクリーニングと出力処理を行う
    for (index, combined_text) in combined_texts.into_iter().enumerate() {
        let (cleaned_text, cleaning_details) =
            text_cleaner::clean_text(&combined_text, &cleaning_options);

        // クリーニングの詳細を表示
        if !cleaning_details.is_empty() && cli.verbose {
            for detail in cleaning_details {
                debug!("Cleaning detail: {}", detail);
            }
        }

        // テキストが空でない場合のみ、ファイルに書き出す
        if !cleaned_text.is_empty() {
            let output_file_name = if num_texts > 1 {
                format!("{}_{}.txt", output_name.trim_end_matches(".txt"), index + 1)
            } else {
                output_name.clone()
            };
            let output_file_path = format!("{}/{}", cli.output_dir, output_file_name);

            let char_count = cleaned_text.len();
            let line_count = cleaned_text.lines().count();
            let file_size_bytes = cleaned_text.as_bytes().len();
            let readable_file_size = format_size(file_size_bytes as u64, DECIMAL);

            text_output::write_to_file(&cleaned_text, &output_file_path);
            info!(
                "Output file: {}, {}, {} characters, {} lines",
                output_file_path, readable_file_size, char_count, line_count
            );
        }
    }
}
