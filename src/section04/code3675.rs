#![allow(dead_code)]

// 3675. Minimum Operations to Transform String
// https://leetcode.com/problems/minimum-operations-to-transform-string/
// https://leetcode.cn/problems/minimum-operations-to-transform-string/
//
// Medium
//
// You are given a string s consisting only of lowercase English letters.
//
// You can perform the following operation any number of times (including zero):
//
// Choose any character c in the string and replace every occurrence of c with the next lowercase letter in the English alphabet.
//
// Return the minimum number of operations required to transform s into a string consisting of only 'a' characters.
//
// Note: Consider the alphabet as circular, thus 'a' comes after 'z'.
//
// Example 1:
//
// Input: s = "yz"
//
// Output: 2
//
// Explanation:
//
// Change 'y' to 'z' to get "zz".
// Change 'z' to 'a' to get "aa".
// Thus, the answer is 2.
//
// Example 2:
//
// Input: s = "a"
//
// Output: 0
//
// Explanation:
//
// The string "a" only consists of 'a'​​​​​​​ characters. Thus, the answer is 0.
//
// Constraints:
//
// 1 <= s.length <= 5 * 10^5
// s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut ops = 0;

        for c in s.chars() {
            let x = (26 - ((c as u8) - b'a')) % 26;
            ops = std::cmp::max(ops, x as i32);
        }

        ops
    }
}

#[test]
fn test() {
    let s = "yz".to_string();
    assert_eq!(Solution::min_operations(s), 2);

    let s = "a".to_string();
    assert_eq!(Solution::min_operations(s), 0);
}
