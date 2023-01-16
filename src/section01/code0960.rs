#![allow(dead_code)]

// 960. Delete Columns to Make Sorted III
// https://leetcode.com/problems/delete-columns-to-make-sorted-iii/
// https://leetcode.cn/problems/delete-columns-to-make-sorted-iii/
//
// You are given an array of n strings strs, all of the same length.
//
// We may choose any deletion indices, and we delete all the characters in those indices for each string.
//
// For example, if we have strs = ["abcdef","uvwxyz"] and deletion indices {0, 2, 3}, then the final array after deletions is ["bef", "vyz"].
//
// Suppose we chose a set of deletion indices answer such that after deletions, the final array has every string (row) in lexicographic order.
// (i.e., (strs[0][0] <= strs[0][1] <= ... <= strs[0][strs[0].length - 1]), and (strs[1][0] <= strs[1][1] <= ... <= strs[1][strs[1].length - 1]), and so on).
// Return the minimum possible value of answer.length.
//
// Example 1:
//
// Input: strs = ["babca","bbazb"]
// Output: 3
// Explanation: After deleting columns 0, 1, and 4, the final array is strs = ["bc", "az"].
// Both these rows are individually in lexicographic order (ie. strs[0][0] <= strs[0][1] and strs[1][0] <= strs[1][1]).
// Note that strs[0] > strs[1] - the array strs is not necessarily in lexicographic order.
//
// Example 2:
//
// Input: strs = ["edcba"]
// Output: 4
// Explanation: If we delete less than 4 columns, the only row will not be lexicographically sorted.
//
// Example 3:
//
// Input: strs = ["ghi","def","abc"]
// Output: 0
// Explanation: All rows are already lexicographically sorted.
//
// Constraints:
//
// - n == strs.length
// - 1 <= n <= 100
// - 1 <= strs[i].length <= 100
// - strs[i] consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let m = strs.len();
        let n = strs[0].len();
        let mut res = n as i32 - 1;
        let mut dp = vec![1; n];
        for j in 0..n {
            for i in 0..j {
                let mut k = 0;
                while k < m {
                    if strs[k].as_bytes()[i] > strs[k].as_bytes()[j] {
                        break;
                    }
                    k += 1;
                }
                if k == m && dp[i] + 1 > dp[j] {
                    dp[j] = dp[i] + 1;
                }
            }
            res = res.min(n as i32 - dp[j]);
        }
        res
    }
}

#[test]
fn test() {
    let strs = vec!["babca".to_string(), "bbazb".to_string()];
    assert_eq!(Solution::min_deletion_size(strs), 3);

    let strs = vec!["edcba".to_string()];
    assert_eq!(Solution::min_deletion_size(strs), 4);

    let strs = vec!["ghi".to_string(), "def".to_string(), "abc".to_string()];
    assert_eq!(Solution::min_deletion_size(strs), 0);
}
