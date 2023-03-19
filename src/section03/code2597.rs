#![allow(dead_code)]

/*

// 2597. The Number of Beautiful Subsets
// https://leetcode.com/problems/the-number-of-beautiful-subsets/
// https://leetcode.cn/problems/the-number-of-beautiful-subsets/
//
// Medium
//
// You are given an array nums of positive integers and a positive integer k.

A subset of nums is beautiful if it does not contain two integers with an absolute difference equal to k.

Return the number of non-empty beautiful subsets of the array nums.

A subset of nums is an array that can be obtained by deleting some (possibly none) elements from nums. Two subsets are different if and only if the chosen indices to delete are different.

Example 1:

Input: nums = [2,4,6], k = 2
Output: 4
Explanation: The beautiful subsets of the array nums are: [2], [4], [6], [2, 6].
It can be proved that there are only 4 beautiful subsets in the array [2,4,6].

Example 2:

Input: nums = [1], k = 1
Output: 1
Explanation: The beautiful subset of the array nums is [1].
It can be proved that there is only 1 beautiful subset in the array [1].

Constraints:

    1 <= nums.length <= 20
    1 <= nums[i], k <= 1000
*/

struct Solution;

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        fn dfs(dp: &mut Vec<Vec<i32>>, i: usize, last_i: usize, gs: &Vec<i32>, k: i32) -> i32 {
            if i == gs.len() {
                return 1;
            }
            let ugly = gs[i] - gs[last_i] == k;
            if dp[i][ugly as usize] == 0 {
                dp[i][ugly as usize] = dfs(dp, i + 1, last_i, gs, k) + if ugly { 0 } else { dfs(dp, i + 1, i, gs, k) };
            }
            dp[i][ugly as usize]
        }

        let mut dp = vec![vec![0; 2]; 21];
        let mut gs = vec![];
        let mut m = std::collections::HashMap::new();
        for n in nums {
            m.entry(n % k).or_insert(vec![]).push(n);
        }
        for (_, mut g) in m {
            g.sort();
            gs.append(&mut g);
        }
        dfs(&mut dp, 0, gs.len() - 1, &gs, k) - 1
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 4, 6], 2, 4),
        (vec![1], 1, 1),
        (vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 1, 143),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::beautiful_subsets(nums, k), expected);
    }
}
