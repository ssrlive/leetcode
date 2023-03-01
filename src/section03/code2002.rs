#![allow(dead_code)]

/*

// 2002. Maximum Product of the Length of Two Palindromic Subsequences
// https://leetcode.com/problems/maximum-product-of-the-length-of-two-palindromic-subsequences/
// https://leetcode.cn/problems/maximum-product-of-the-length-of-two-palindromic-subsequences/
//
// Medium
//
// Given a string s, find two disjoint palindromic subsequences of s such that the product of their lengths is maximized. The two subsequences are disjoint if they do not both pick a character at the same index.

Return the maximum possible product of the lengths of the two palindromic subsequences.

A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters. A string is palindromic if it reads the same forward and backward.

Example 1:
example-1

Input: s = "leetcodecom"
Output: 9
Explanation: An optimal solution is to choose "ete" for the 1st subsequence and "cdc" for the 2nd subsequence.
The product of their lengths is: 3 * 3 = 9.

Example 2:

Input: s = "bb"
Output: 1
Explanation: An optimal solution is to choose "b" (the first character) for the 1st subsequence and "b" (the second character) for the 2nd subsequence.
The product of their lengths is: 1 * 1 = 1.

Example 3:

Input: s = "accbcaxxcxx"
Output: 25
Explanation: An optimal solution is to choose "accca" for the 1st subsequence and "xxcxx" for the 2nd subsequence.
The product of their lengths is: 5 * 5 = 25.

Constraints:

    2 <= s.length <= 12
    s consists of lowercase English letters only.
*/

struct Solution;

impl Solution {
    pub fn max_product(s: String) -> i32 {
        let mut pali_len: Vec<(usize, usize)> = (1..1 << s.len())
            .filter_map(|mask| {
                let subseq: String = (0..s.len())
                    .filter(|i| mask & (1 << i) > 0)
                    .map(|i| s.chars().nth(i).unwrap())
                    .collect();

                if subseq == subseq.chars().rev().collect::<String>() {
                    Some((mask, subseq.len()))
                } else {
                    None
                }
            })
            .collect();

        pali_len.sort_by(|a, b| b.1.cmp(&a.1));

        let mut res = 1;
        for i in 0..pali_len.len() {
            let (m1, l1) = pali_len[i];
            if l1.pow(2) < res {
                break;
            }
            for pali_len_j in pali_len.iter().skip(i + 1) {
                let (m2, l2) = pali_len_j;
                if m1 & m2 == 0 && l1 * l2 > res {
                    res = l1 * l2;
                    break;
                }
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_product("leetcodecom".to_string()), 9);
    assert_eq!(Solution::max_product("bb".to_string()), 1);
    assert_eq!(Solution::max_product("accbcaxxcxx".to_string()), 25);
}
