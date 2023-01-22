#![allow(dead_code)]

// 2546. Apply Bitwise Operations to Make Strings Equal
// https://leetcode.com/problems/apply-bitwise-operations-to-make-strings-equal/
// https://leetcode.cn/problems/apply-bitwise-operations-to-make-strings-equal/
//
// Medium
//
// You are given two 0-indexed binary strings s and target of the same length n.
// You can do the following operation on s any number of times:
//
// Choose two different indices i and j where 0 <= i, j < n.
// Simultaneously, replace s[i] with (s[i] OR s[j]) and s[j] with (s[i] XOR s[j]).
//
// For example, if s = "0110", you can choose i = 0 and j = 2, then simultaneously replace s[0]
// with (s[0] OR s[2] = 0 OR 1 = 1), and s[2] with (s[0] XOR s[2] = 0 XOR 1 = 1), so we will have s = "1110".
//
// Return true if you can make the string s equal to target, or false otherwise.
//
// Example 1:
//
// Input: s = "1010", target = "0110"
// Output: true
// Explanation: We can do the following operations:
// - Choose i = 2 and j = 0. We have now s = "0010".
// - Choose i = 2 and j = 1. We have now s = "0110".
// Since we can make s equal to target, we return true.
//
// Example 2:
//
// Input: s = "11", target = "00"
// Output: false
// Explanation: It is not possible to make s equal to target with any number of operations.
//
// Constraints:
//
// -    n == s.length == target.length
// -    2 <= n <= 10^5
// -    s and target consist of only the digits 0 and 1.
//

struct Solution;

impl Solution {
    pub fn make_strings_equal(s: String, target: String) -> bool {
        let (mut cnt1, mut cnt2) = (0, 0);
        for c in s.chars() {
            if c == '1' {
                cnt1 += 1;
            }
        }
        for c in target.chars() {
            if c == '1' {
                cnt2 += 1;
            }
        }
        if cnt1 + cnt2 == 0 {
            return true;
        }
        cnt1 != 0 && cnt2 != 0
    }
}

#[test]
fn test() {
    let cases = vec![("1010", "0110", true), ("11", "00", false)];
    for (s, target, expected) in cases {
        let s = s.to_string();
        let target = target.to_string();
        assert_eq!(Solution::make_strings_equal(s, target), expected);
    }
}
