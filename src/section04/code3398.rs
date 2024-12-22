#![allow(dead_code)]

// 3398. Smallest Substring With Identical Characters I
// https://leetcode.com/problems/smallest-substring-with-identical-characters-i/
// https://leetcode.cn/problems/smallest-substring-with-identical-characters-i/
//
// Hard
//
// You are given a binary string s of length n and an integer numOps.
//
// You are allowed to perform the following operation on s at most numOps times:
//
// Select any index i (where 0 <= i < n) and flip s[i], i.e., if s[i] == '1', change s[i] to '0' and vice versa.
// You need to minimize the length of the longest substring of s such that all the characters in the substring are identical.
//
// Return the minimum length after the operations.
//
// A substring is a contiguous non-empty sequence of characters within a string.
//
// Example 1:
//
// Input: s = "000001", numOps = 1
//
// Output: 2
//
// Explanation:
//
// By changing s[2] to '1', s becomes "001001". The longest substrings with identical characters are s[0..1] and s[3..4].
//
// Example 2:
//
// Input: s = "0000", numOps = 2
//
// Output: 1
//
// Explanation:
//
// By changing s[0] and s[2] to '1', s becomes "1010".
//
// Example 3:
//
// Input: s = "0101", numOps = 0
//
// Output: 1
//
// Constraints:
//
// 1 <= n == s.length <= 1000
// s consists only of '0' and '1'.
// 0 <= numOps <= n
//

struct Solution;

impl Solution {
    pub fn min_length(s: String, num_ops: i32) -> i32 {
        fn check(s: &[u8], mut num_ops: i32, _mid: usize, mut start_char: u8) -> bool {
            for &s_i in s {
                if start_char == s_i {
                    num_ops -= 1;
                }
                start_char = if start_char == b'0' { b'1' } else { b'0' };
            }
            num_ops >= 0
        }

        fn is_valid(s: &[u8], mut num_ops: i32, mid: usize) -> bool {
            if mid == 1 {
                return check(s, num_ops, mid, b'1') || check(s, num_ops, mid, b'0');
            }
            let mut count = 0;
            let mut last = b' ';
            for &s_i in s {
                if s_i == last {
                    count += 1;
                } else {
                    num_ops -= count as i32 / (mid + 1) as i32;
                    last = s_i;
                    count = 1;
                }
            }
            if count > mid {
                num_ops -= count as i32 / (mid + 1) as i32;
            }
            num_ops >= 0
        }

        let s = s.as_bytes();
        let mut start = 1;
        let mut end = s.len();
        let mut ans = s.len();
        while start <= end {
            let mid = start + (end - start) / 2;
            if is_valid(s, num_ops, mid) {
                ans = mid;
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        ans as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_length("000001".to_string(), 1), 2);
    assert_eq!(Solution::min_length("0000".to_string(), 2), 1);
    assert_eq!(Solution::min_length("0101".to_string(), 0), 1);
}
