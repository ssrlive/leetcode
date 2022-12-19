#![allow(dead_code)]

// 3. Longest Substring Without Repeating Characters
// https://leetcode.com/problems/longest-substring-without-repeating-characters/
// https://leetcode.cn/problems/longest-substring-without-repeating-characters/
//
// Given a string s, find the length of the longest substring without repeating characters.
//
// Example 1:
//
// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.
//
// Example 2:
//
// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.
//
// Example 3:
//
// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
//
// Constraints:
//
// - 0 <= s.length <= 5 * 10^4
// - s consists of English letters, digits, symbols and spaces.
//

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max = 0;
        let mut start = 0;
        let mut end = 0;
        let chars = s.chars().collect::<Vec<char>>();
        let mut map = std::collections::HashMap::new();
        while end < chars.len() {
            let c = chars[end];
            if let Some(&i) = map.get(&c) {
                start = std::cmp::max(start, i + 1);
            }
            map.insert(c, end);
            max = std::cmp::max(max, end - start + 1);
            end += 1;
        }
        max as i32
    }

    pub fn length_of_longest_substring2(s: String) -> i32 {
        fn _length_of_longest_substring2(s: String) -> Option<i32> {
            let chars = s.chars().collect::<Vec<char>>();
            let mut seen = std::collections::HashMap::new();
            let mut a = 0;
            let mut b = 0;
            seen.insert(*chars.first()?, 0);
            let mut longest = 1;
            while b < chars.len() {
                if let Some(&i) = seen.get(chars.get(b)?) {
                    longest = longest.max(b - a);
                    a = i + 1;
                    b += 1;
                    seen.clear();
                    seen.extend((a..b).map(|k| (chars[k], k)));
                } else {
                    seen.insert(*chars.get(b)?, b);
                    b += 1;
                }
            }
            Some(longest.max(b - a) as i32)
        }
        _length_of_longest_substring2(s).unwrap_or(0)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::length_of_longest_substring("abcabcbb".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("bbbbb".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring("pwwkew".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("".to_string()), 0);
    assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring("dvdf".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring("abba".to_string()), 2);

    assert_eq!(Solution::length_of_longest_substring2("abcabcbb".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring2("bbbbb".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring2("pwwkew".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring2("".to_string()), 0);
    assert_eq!(Solution::length_of_longest_substring2(" ".to_string()), 1);
    assert_eq!(Solution::length_of_longest_substring2("dvdf".to_string()), 3);
    assert_eq!(Solution::length_of_longest_substring2("abba".to_string()), 2);
}
