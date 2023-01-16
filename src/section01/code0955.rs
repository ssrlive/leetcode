#![allow(dead_code)]

// 955. Delete Columns to Make Sorted II
// https://leetcode.com/problems/delete-columns-to-make-sorted-ii/
// https://leetcode.cn/problems/delete-columns-to-make-sorted-ii/
//
// You are given an array of n strings strs, all of the same length.
//
// We may choose any deletion indices, and we delete all the characters in those indices for each string.
//
// For example, if we have strs = ["abcdef","uvwxyz"] and deletion indices {0, 2, 3}, then the final array after deletions is ["bef", "vyz"].
//
// Suppose we chose a set of deletion indices answer such that after deletions, the final array has its elements in lexicographic order (i.e., strs[0] <= strs[1] <= strs[2] <= ... <= strs[n - 1]). Return the minimum possible value of answer.length.
//
// Example 1:
//
// Input: strs = ["ca","bb","ac"]
// Output: 1
// Explanation:
// After deleting the first column, strs = ["a", "b", "c"].
// Now strs is in lexicographic order (ie. strs[0] <= strs[1] <= strs[2]).
// We require at least 1 deletion since initially strs was not in lexicographic order, so the answer is 1.
//
// Example 2:
//
// Input: strs = ["xc","yb","za"]
// Output: 0
// Explanation:
// strs is already in lexicographic order, so we do not need to delete anything.
// Note that the rows of strs are not necessarily in lexicographic order:
// i.e., it is NOT necessarily true that (strs[0][0] <= strs[0][1] <= ...)
//
// Example 3:
//
// Input: strs = ["zyx","wvu","tsr"]
// Output: 3
// Explanation: We have to delete every column.
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
        let mut res = 0;
        let n = strs.len();
        let m = strs[0].len();
        let mut sorted = vec![false; n - 1];
        for j in 0..m {
            let mut ii = 0;
            for i in 0..n - 1 {
                if !sorted[i] && strs[i].as_bytes()[j] > strs[i + 1].as_bytes()[j] {
                    res += 1;
                    ii = i;
                    break;
                }
                ii = i + 1;
            }
            if ii < n - 1 {
                continue;
            }
            for i in 0..n - 1 {
                sorted[i] = sorted[i] || strs[i].as_bytes()[j] < strs[i + 1].as_bytes()[j];
            }
        }
        res
    }
}

#[test]
fn test() {
    let strs = vec!["ca", "bb", "ac"];
    let res = Solution::min_deletion_size(strs.iter().map(|s| s.to_string()).collect());
    assert_eq!(res, 1);

    let strs = vec!["xc", "yb", "za"];
    let res = Solution::min_deletion_size(strs.iter().map(|s| s.to_string()).collect());
    assert_eq!(res, 0);

    let strs = vec!["zyx", "wvu", "tsr"];
    let res = Solution::min_deletion_size(strs.iter().map(|s| s.to_string()).collect());
    assert_eq!(res, 3);
}
