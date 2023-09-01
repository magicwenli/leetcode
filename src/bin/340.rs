// 340. 至多包含 K 个不同字符的最长子串

impl Solution {
    pub fn length_of_longest_substring_k_distinct(s: String, k: i32) -> i32 {
        /* 滑动窗口 保持最多只有k个不同字符 */
        let s = s.as_bytes();
        let (mut left, mut right) = (0, 0);
        let mut map = std::collections::HashMap::new();
        let mut max_len = 0;
        while right < s.len() {
            map.insert(s[right], right);
            right += 1;
            if map.len() == k as usize + 1 {
                let mut min = s.len();
                /* 找到index最小的元素并删除 */
                for &v in map.values() {
                    min = min.min(v);
                }
                map.remove(&s[min]);
                left = min + 1;
            }
            max_len = max_len.max(right - left);
        }
        max_len as i32
    }
}

struct Solution {}

#[test]
fn test_length_of_longest_substring_k_distinct() {
    assert_eq!(
        Solution::length_of_longest_substring_k_distinct("eceba".to_string(), 2),
        3
    );
    assert_eq!(
        Solution::length_of_longest_substring_k_distinct("ccaabbb".to_string(), 2),
        5
    );
}
