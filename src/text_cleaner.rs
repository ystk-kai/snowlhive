use regex::Regex;

pub struct CleaningOptions {
    pub remove_codeblock: bool,
    pub remove_url: bool,
}

/// 指定されたテキストから不要な要素をクリーニングする
///
/// Args:
///     text: クリーニングするテキスト
///
/// Returns:
///    クリーニングされたテキストと、クリーニングの詳細
pub fn clean_text(text: &str, options: &CleaningOptions) -> (String, Vec<String>) {
    let mut cleaned_text = String::new();
    let mut details = Vec::new();
    let mut in_code_block = false;
    let mut code_block_start = 0;
    let mut line_number = 1;
    let url_regex = Regex::new(r"http[s]?://[^\s]+").unwrap();

    for line in text.lines() {
        if options.remove_codeblock && line.trim().starts_with("```") {
            in_code_block = !in_code_block;
            if in_code_block {
                code_block_start = line_number;
            } else {
                details.push(format!(
                    "Removed code block: Lines {} to {}",
                    code_block_start, line_number
                ));
            }
            line_number += 1;
            continue;
        }

        if !in_code_block {
            let mut modified_line = line.to_string();
            if options.remove_url {
                for url in url_regex.captures_iter(&modified_line) {
                    details.push(format!("Removed URL: {} at line {}", &url[0], line_number));
                }
                modified_line = url_regex.replace_all(&modified_line, "").to_string();
            }
            cleaned_text.push_str(&modified_line);
            cleaned_text.push('\n');
        }
        line_number += 1;
    }

    (cleaned_text, details)
}
