#![allow(dead_code)]

// 3258. Count Substrings That Satisfy K-Constraint I
// https://leetcode.com/problems/count-substrings-that-satisfy-k-constraint-i/
// https://leetcode.cn/problems/count-substrings-that-satisfy-k-constraint-i/
//
// Easy
//
// You are given a binary string s and an integer k.
//
// A binary string satisfies the k-constraint if either of the following conditions holds:
//
//     The number of 0's in the string is at most k.
//     The number of 1's in the string is at most k.
//
// Return an integer denoting the number of substrings of s that satisfy the k-constraint.
//
// Example 1:
//
// Input: s = "10101", k = 1
//
// Output: 12
//
// Explanation:
//
// Every substring of s except the substrings "1010", "10101", and "0101" satisfies the k-constraint.
//
// Example 2:
//
// Input: s = "1010101", k = 2
//
// Output: 25
//
// Explanation:
//
// Every substring of s except the substrings with a length greater than 5 satisfies the k-constraint.
//
// Example 3:
//
// Input: s = "11111", k = 1
//
// Output: 15
//
// Explanation:
//
// All substrings of s satisfy the k-constraint.
//
// Constraints:
//
//     1 <= s.length <= 50
//     1 <= k <= s.length
//     s[i] is either '0' or '1'.
//

struct Solution;

impl Solution {
    pub fn count_k_constraint_substrings(s: String, k: i32) -> i32 {
        let s = std::iter::once('0')
            .chain(s.chars())
            .scan(0, |acc, c| {
                if c == '1' {
                    *acc += 1
                };
                Some(*acc)
            })
            .collect::<Vec<i32>>();

        let mut total = 0;
        for i in 0..s.len() {
            for j in i + 1..s.len() {
                let x = s[j] - s[i];
                if x <= k || (j - i) as i32 - x <= k {
                    total += 1;
                }
            }
        }
        total
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_k_constraint_substrings("10101".to_string(), 1), 12);
    assert_eq!(Solution::count_k_constraint_substrings("1010101".to_string(), 2), 25);
    assert_eq!(Solution::count_k_constraint_substrings("11111".to_string(), 1), 15);
}
