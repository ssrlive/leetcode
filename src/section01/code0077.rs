#![allow(dead_code)]

// 77. Combinations
// https://leetcode.com/problems/combinations/
// https://leetcode.cn/problems/combinations/
//
// Given two integers n and k, return all possible combinations of k numbers chosen from the range [1, n].
//
// You may return the answer in any order.
//
// Example 1:
//
// Input: n = 4, k = 2
// Output: [[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]]
// Explanation: There are 4 choose 2 = 6 total combinations.
// Note that combinations are unordered, i.e., [1,2] and [2,1] are considered to be the same combination.
//
// Example 2:
//
// Input: n = 1, k = 1
// Output: [[1]]
// Explanation: There is 1 choose 1 = 1 total combination.
//
// Constraints:
//
// - 1 <= n <= 20
// - 1 <= k <= n
//

struct Solution;

impl Solution {
    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        let mut res = vec![];
        if k == 0 {
            return vec![vec![]];
        }
        for i in k..=n {
            for mut pre in Self::combine(i - 1, k - 1) {
                pre.push(i);
                res.push(pre);
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (4, 2, vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4]]),
        (1, 1, vec![vec![1]]),
    ];
    for (n, k, expect) in cases {
        let mut res = Solution::combine(n, k);
        res.sort();
        assert_eq!(res, expect);
    }
}
