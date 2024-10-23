/*
 * @lc app=leetcode.cn id=8 lang=rust
 *
 * [8] 字符串转换整数 (atoi)
 *
 * https://leetcode.cn/problems/string-to-integer-atoi/description/
 *
 * algorithms
 * Medium (21.39%)
 * Likes:    1861
 * Dislikes: 0
 * Total Accepted:    675.1K
 * Total Submissions: 3.2M
 * Testcase Example:  '"42"'
 *
 * 请你来实现一个 myAtoi(string s) 函数，使其能将字符串转换成一个 32 位有符号整数。
 *
 * 函数 myAtoi(string s) 的算法如下：
 *
 *
 * 空格：读入字符串并丢弃无用的前导空格（" "）
 * 符号：检查下一个字符（假设还未到字符末尾）为 '-' 还是 '+'。如果两者都不存在，则假定结果为正。
 * 转换：通过跳过前置零来读取该整数，直到遇到非数字字符或到达字符串的结尾。如果没有读取数字，则结果为0。
 * 舍入：如果整数数超过 32 位有符号整数范围 [−2^31,  2^31 − 1] ，需要截断这个整数，使其保持在这个范围内。具体来说，小于 −2^31
 * 的整数应该被舍入为 −2^31 ，大于 2^31 − 1 的整数应该被舍入为 2^31 − 1 。
 *
 *
 * 返回整数作为最终结果。
 *
 *
 *
 * 示例 1：
 *
 *
 * 输入：s = "42"
 *
 * 输出：42
 *
 * 解释：加粗的字符串为已经读入的字符，插入符号是当前读取的字符。
 *
 *
 * 带下划线线的字符是所读的内容，插入符号是当前读入位置。
 * 第 1 步："42"（当前没有读入字符，因为没有前导空格）
 * ⁠        ^
 * 第 2 步："42"（当前没有读入字符，因为这里不存在 '-' 或者 '+'）
 * ⁠        ^
 * 第 3 步："42"（读入 "42"）
 * ⁠          ^
 *
 *
 *
 * 示例 2：
 *
 *
 * 输入：s = " -042"
 *
 * 输出：-42
 *
 * 解释：
 *
 *
 * 第 1 步："   -042"（读入前导空格，但忽视掉）
 * ⁠           ^
 * 第 2 步："   -042"（读入 '-' 字符，所以结果应该是负数）
 * ⁠            ^
 * 第 3 步："   -042"（读入 "042"，在结果中忽略前导零）
 * ⁠              ^
 *
 *
 *
 * 示例 3：
 *
 *
 * 输入：s = "1337c0d3"
 *
 * 输出：1337
 *
 * 解释：
 *
 *
 * 第 1 步："1337c0d3"（当前没有读入字符，因为没有前导空格）
 * ⁠        ^
 * 第 2 步："1337c0d3"（当前没有读入字符，因为这里不存在 '-' 或者 '+'）
 * ⁠        ^
 * 第 3 步："1337c0d3"（读入 "1337"；由于下一个字符不是一个数字，所以读入停止）
 * ⁠            ^
 *
 *
 *
 * 示例 4：
 *
 *
 * 输入：s = "0-1"
 *
 * 输出：0
 *
 * 解释：
 *
 *
 * 第 1 步："0-1" (当前没有读入字符，因为没有前导空格)
 * ⁠        ^
 * 第 2 步："0-1" (当前没有读入字符，因为这里不存在 '-' 或者 '+')
 * ⁠        ^
 * 第 3 步："0-1" (读入 "0"；由于下一个字符不是一个数字，所以读入停止)
 * ⁠         ^
 *
 *
 *
 * 示例 5：
 *
 *
 * 输入：s = "words and 987"
 *
 * 输出：0
 *
 * 解释：
 *
 * 读取在第一个非数字字符“w”处停止。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= s.length <= 200
 * s 由英文字母（大写和小写）、数字（0-9）、' '、'+'、'-' 和 '.' 组成
 *
 *
 */

struct Solution;
fn main() {}

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        // 1. 读入并丢弃前导空格
        let mut iter = s.trim_start().chars().peekable();
        // 2. 读入符号
        let sign = match iter.peek() {
            Some(&'+') => {
                iter.next();
                1
            }
            Some(&'-') => {
                iter.next();
                -1
            }
            _ => 1,
        };
        let mut res = 0i32;
        // 3. 读入数字, 遇到非数字字符或到达字符串的结尾就break
        while let Some(c) = iter.peek() {
            match c {
                '0'..='9' => {
                    let digit = c.to_digit(10).unwrap() as i32;
                    match res.checked_mul(10).and_then(|r| r.checked_add(digit)) {
                        Some(r) => {
                            res = r;
                            iter.next();
                        }
                        None => {
                            return if sign == 1 { i32::MAX } else { i32::MIN };
                        }
                    }
                }
                _ => break,
            }
        }
        res * sign
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "42".to_string();
        assert_eq!(Solution::my_atoi(s), 42);
        let s = " -042".to_string();
        assert_eq!(Solution::my_atoi(s), -42);
        let s = "1337c0d3".to_string();
        assert_eq!(Solution::my_atoi(s), 1337);
        let s = "0-1".to_string();
        assert_eq!(Solution::my_atoi(s), 0);
        let s = "words and 987".to_string();
        assert_eq!(Solution::my_atoi(s), 0);
        // 舍入：如果整数数超过 32 位有符号整数范围 [−2^31,  2^31 − 1] ，需要截断这个整数，使其保持在这个范围内
        let s = i32::MAX.to_string() + "1";
        assert_eq!(Solution::my_atoi(s), i32::MAX);
        // negative overflow
        let s = "-91283472332".to_string();
        assert_eq!(Solution::my_atoi(s), i32::MIN);
    }
}
