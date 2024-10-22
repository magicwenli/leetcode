use regex::Regex;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

struct Problem {
    id: String,
    name: String,
    link: String,
    category: String,
    difficulty: String,
}

const README_TEMPLATE: &str = r#"

## LeetCode rust

这是我在 LeetCode 上的 rust 解题仓库，主要用于记录自己的解题思路和代码。

<MARKDOWN_TABLE>

## License

MIT

"#;

fn main() -> io::Result<()> {
    let bin_dir = Path::new("src/bin");
    let readme_path = Path::new("README.md");
    let mut problems: Vec<Problem> = Vec::new();

    // Define regex patterns
    let re_id = Regex::new(r"@lc app=leetcode\.cn id=(\d+) lang=rust").unwrap();
    let re_name = Regex::new(r"\[\d+\] (.+)").unwrap();
    let re_link = Regex::new(r"https?://leetcode\.cn/problems/[^\s]+").unwrap();
    let re_category = Regex::new(r" \* ([\w,]+)").unwrap();
    let re_difficulty = Regex::new(r"(\w+) \(\d+\.\d+%\)").unwrap();

    // Iterate over each .rs file in src/bin
    for entry in fs::read_dir(bin_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            // 读取整个文件内容
            let content = fs::read_to_string(&path)?;
            let mut problem = Problem {
                id: String::new(),
                name: String::new(),
                link: String::new(),
                category: String::new(),
                difficulty: String::new(),
            };
            // 在全文中搜索并提取信息
            if let Some(caps) = re_id.captures(&content) {
                problem.id = caps[1].to_string();
            }
            if let Some(caps) = re_name.captures(&content) {
                problem.name = caps[1].trim().to_string();
            }
            if let Some(caps) = re_link.captures(&content) {
                problem.link = caps[0].to_string();
            }
            let mut caps = re_category.captures_iter(&content);
            if let Some(caps) = caps.nth(1) {
                problem.category = caps[1].to_string();
            }
            if let Some(caps) = re_difficulty.captures(&content) {
                problem.difficulty = caps[1].to_string();
            }

            if !problem.id.is_empty() {
                problems.push(problem);
            }
        }
    }

    problems.sort_by(|a, b| a.id.parse::<i32>().unwrap().cmp(&b.id.parse::<i32>().unwrap()));

    // Generate Markdown table
    let mut markdown = String::from("| ID | 名称 | 链接 | 分类 | 难度 |\n");
    markdown.push_str("|----|------|------|------|------|\n");
    for p in problems {
        markdown.push_str(&format!(
            "| {} | {} | [链接]({}) | {} | {} |\n",
            p.id, p.name, p.link, p.category, p.difficulty
        ));
    }

    // Write to README.md
    let mut readme = File::create(readme_path)?;
    let markdown = README_TEMPLATE.replace("<MARKDOWN_TABLE>", &markdown);
    readme.write_all(markdown.as_bytes())?;

    Ok(())
}