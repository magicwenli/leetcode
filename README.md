# LeetCode rust

这是我在 LeetCode 上的 rust 解题仓库，主要用于记录自己的解题思路和代码。

| ID | 名称 | 分类 | 难度 | 链接 | 路径 |
|----|------|------|------|------|------|
| 4 | 寻找两个正序数组的中位数 | algorithms | Hard | [链接](https://leetcode.cn/problems/median-of-two-sorted-arrays/description/) | [源码](src/bin/4_寻找两个正序数组的中位数.rs) |
| 5 | 最长回文子串 | algorithms | Medium | [链接](https://leetcode.cn/problems/longest-palindromic-substring/description/) | [源码](src/bin/5_最长回文子串.rs) |
| 6 | Z 字形变换 | algorithms | Medium | [链接](https://leetcode.cn/problems/zigzag-conversion/description/) | [源码](src/bin/6_z-字形变换.rs) |
| 7 | 整数反转 | algorithms | Medium | [链接](https://leetcode.cn/problems/reverse-integer/description/) | [源码](src/bin/7_整数反转.rs) |
| 8 | 字符串转换整数 (atoi) | algorithms | Medium | [链接](https://leetcode.cn/problems/string-to-integer-atoi/description/) | [源码](src/bin/8_字符串转换整数-atoi.rs) |
| 10 | 正则表达式匹配 | algorithms | Hard | [链接](https://leetcode.cn/problems/regular-expression-matching/description/) | [源码](src/bin/10_正则表达式匹配.rs) |
| 11 | 盛最多水的容器 | algorithms | Medium | [链接](https://leetcode.cn/problems/container-with-most-water/description/) | [源码](src/bin/11_盛最多水的容器.rs) |
| 14 | 最长公共前缀 | algorithms | Easy | [链接](https://leetcode.cn/problems/longest-common-prefix/description/) | [源码](src/bin/14_最长公共前缀.rs) |
| 17 | 电话号码的字母组合 | algorithms | Medium | [链接](https://leetcode.cn/problems/letter-combinations-of-a-phone-number/description/) | [源码](src/bin/17_电话号码的字母组合.rs) |
| 19 | 删除链表的倒数第 N 个结点 | algorithms | Medium | [链接](https://leetcode.cn/problems/remove-nth-node-from-end-of-list/description/) | [源码](src/bin/19_删除链表的倒数第-n-个结点.rs) |
| 21 | 合并两个有序链表 | algorithms | Easy | [链接](https://leetcode.cn/problems/merge-two-sorted-lists/description/) | [源码](src/bin/21_合并两个有序链表.rs) |
| 26 | 删除有序数组中的重复项 | algorithms | Easy | [链接](https://leetcode.cn/problems/remove-duplicates-from-sorted-array/description/) | [源码](src/bin/26_删除有序数组中的重复项.rs) |
| 28 | 找出字符串中第一个匹配项的下标 | algorithms | Easy | [链接](https://leetcode.cn/problems/find-the-index-of-the-first-occurrence-in-a-string/description/) | [源码](src/bin/28_找出字符串中第一个匹配项的下标.rs) |
| 35 | 搜索插入位置 | algorithms | Easy | [链接](https://leetcode.cn/problems/search-insert-position/description/) | [源码](src/bin/35_搜索插入位置.rs) |
| 58 | 最后一个单词的长度 | algorithms | Easy | [链接](https://leetcode.cn/problems/length-of-last-word/description/) | [源码](src/bin/58_最后一个单词的长度.rs) |
| 3185 | 构成整天的下标对数目 II | algorithms | Medium | [链接](https://leetcode.cn/problems/count-pairs-that-form-a-complete-day-ii/description/) | [源码](src/bin/3185_构成整天的下标对数目-ii.rs) |


## Config

- 使用魔改的vscode插件[LeetCode modified](https://marketplace.visualstudio.com/items?itemName=magicwenli.vscode-leetcode-modified)
- 修改git pre-commit hook

```shell
#!/bin/sh
cargo fmt --all
cargo run --bin readme_gen
git add README.md
```

- 手动修改template: `vscode-insiders\data\extensions\magicwenli.vscode-leetcode-modified-0.18.4\node_modules\vsc-leetcode-cli\templates\detailed.tpl`, 给template加上一些额外的东西

```rust
struct Solution;
fn main() {}

${comment.singleLine} @lc code=start
${code}
${comment.singleLine} @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
```

## License

MIT

