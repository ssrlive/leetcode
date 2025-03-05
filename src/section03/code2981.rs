#![allow(dead_code)]

// 2981. Find Longest Special Substring That Occurs Thrice I
// https://leetcode.com/problems/find-longest-special-substring-that-occurs-thrice-i/
// https://leetcode.cn/problems/find-longest-special-substring-that-occurs-thrice-i/
//
// Medium
//
// You are given a string s that consists of lowercase English letters.
//
// A string is called special if it is made up of only a single character. For example,
// the string "abc" is not special, whereas the strings "ddd", "zz", and "f" are special.
//
// Return the length of the longest special substring of s which occurs at least thrice,
// or -1 if no special substring occurs at least thrice.
//
// A substring is a contiguous non-empty sequence of characters within a string.
//
// Example 1:
//
// Input: s = "aaaa"
// Output: 2
// Explanation: The longest special substring which occurs thrice is "aa": substrings "aaaa", "aaaa", and "aaaa".
// It can be shown that the maximum length achievable is 2.
//
// Example 2:
//
// Input: s = "abcdef"
// Output: -1
// Explanation: There exists no special substring which occurs at least thrice. Hence return -1.
//
// Example 3:
//
// Input: s = "abcaba"
// Output: 1
// Explanation: The longest special substring which occurs thrice is "a": substrings "abcaba", "abcaba", and "abcaba".
// It can be shown that the maximum length achievable is 1.
//
// Constraints:
//
// 3 <= s.length <= 50
// s consists of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        let mut count = 0;
        let mut last = s.chars().nth(0).unwrap();
        let mut ans = 0;
        let mut ump = std::collections::HashMap::new();
        for c in s.chars() {
            if c == last {
                count += 1;
            } else {
                let v = ump.entry(last).or_insert(vec![]);
                v.push(count);
                count = 1;
                last = c;
            }
        }
        let v = ump.entry(last).or_insert(vec![]);
        v.push(count);

        for (_k, v) in ump {
            let mut l1 = 0;
            let mut l2 = 0;
            let mut l3 = 0;
            for e in v {
                if l1 < e {
                    l3 = l2;
                    l2 = l1;
                    l1 = e;
                } else if l2 < e {
                    l3 = l2;
                    l2 = e;
                } else if l3 < e {
                    l3 = e;
                }
            }
            if l1 == l2 && l1 > l3 {
                l2 -= 1;
            }
            if l1 + l2 + l3 >= 3 {
                ans = ans.max(l1 - 2).max(l2);
            }
        }
        if ans != 0 { ans } else { -1 }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_length("aaaa".to_string()), 2);
    assert_eq!(Solution::maximum_length("abcdef".to_string()), -1);
    assert_eq!(Solution::maximum_length("abcaba".to_string()), 1);
}
