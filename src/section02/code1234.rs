#![allow(dead_code)]

// 1234. Replace the Substring for Balanced String
// https://leetcode.com/problems/replace-the-substring-for-balanced-string/
// https://leetcode.cn/problems/replace-the-substring-for-balanced-string/
//
// Medium
//
// You are given a string s of length n containing only four kinds of characters: 'Q', 'W', 'E', and 'R'.
//
// A string is said to be balanced if each of its characters appears n / 4 times where n is the length of the string.
//
// Return the minimum length of the substring that can be replaced with any other string of the same length to make s balanced. If s is already balanced, return 0.
//
// Example 1:
//
// Input: s = "QWER"
// Output: 0
// Explanation: s is already balanced.
//
// Example 2:
//
// Input: s = "QQWE"
// Output: 1
// Explanation: We need to replace a 'Q' to 'R', so that "RQWE" (or "QRWE") is balanced.
//
// Example 3:
//
// Input: s = "QQQW"
// Output: 2
// Explanation: We can replace the first "QQ" to "ER".
//
// Constraints:
//
// -    n == s.length
// -    4 <= n <= 10^5
// -    n is a multiple of 4.
// -    s contains only 'Q', 'W', 'E', and 'R'.
//

struct Solution;

impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let mut count = vec![0; 128];
        let s = s.as_bytes();
        let n = s.len();
        let mut res = n as i32;
        let mut i = 0;
        let k = n / 4;
        for &c in s.iter() {
            count[c as usize] += 1;
        }
        for j in 0..n {
            count[s[j] as usize] -= 1;
            while i < n && count['Q' as usize] <= k && count['W' as usize] <= k && count['E' as usize] <= k && count['R' as usize] <= k {
                res = res.min((1 + j - i) as i32);
                count[s[i] as usize] += 1;
                i += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::balanced_string("QWER".to_string()), 0);
    assert_eq!(Solution::balanced_string("QQWE".to_string()), 1);
    assert_eq!(Solution::balanced_string("QQQW".to_string()), 2);
    assert_eq!(Solution::balanced_string("QQQQ".to_string()), 3);
}
