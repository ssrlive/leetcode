#![allow(dead_code)]

// 1358. Number of Substrings Containing All Three Characters
// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/
// https://leetcode.cn/problems/number-of-substrings-containing-all-three-characters/
//
// Medium
//
// Given a string s consisting only of characters a, b and c.
//
// Return the number of substrings containing at least one occurrence of all these characters a, b and c.
//
// Example 1:
//
// Input: s = "abcabc"
// Output: 10
// Explanation: The substrings containing at least one occurrence of the characters a, b and c are
// "abc", "abca", "abcab", "abcabc", "bca", "bcab", "bcabc", "cab", "cabc" and "abc" (again).
//
// Example 2:
//
// Input: s = "aaacb"
// Output: 3
// Explanation: The substrings containing at least one occurrence of the characters a, b and c are "aaacb", "aacb" and "acb".
//
// Example 3:
//
// Input: s = "abc"
// Output: 1
//
// Constraints:
//
// -    3 <= s.length <= 5 x 10^4
// -    s only consists of a, b or c characters.
//

struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut res = 0;
        let mut count = vec![0; 3];
        let (mut left, mut right) = (0, 0);
        let s = s.as_bytes();
        while right < s.len() {
            count[(s[right] - b'a') as usize] += 1;
            while count[0] > 0 && count[1] > 0 && count[2] > 0 {
                count[(s[left] - b'a') as usize] -= 1;
                left += 1;
            }
            res += left;
            right += 1;
        }
        res as i32
    }
}

#[test]
fn test() {}
