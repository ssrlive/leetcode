#![allow(dead_code)]

// 3068. Find the Maximum Sum of Node Values
// https://leetcode.com/problems/find-the-maximum-sum-of-node-values/
// https://leetcode.cn/problems/find-the-maximum-sum-of-node-values/
//
// Hard
//
// There exists an undirected tree with n nodes numbered 0 to n - 1. You are given a 0-indexed 2D integer array
// edges of length n - 1, where edges[i] = [ui, vi] indicates that there is an edge between nodes ui and vi in the tree.
// You are also given a positive integer k, and a 0-indexed array of non-negative integers
// nums of length n, where nums[i] represents the value of the node numbered i.
//
// Bogdan wants the sum of values of tree nodes to be maximum, for which Bogdan can perform
// the following operation any number of times (including zero) on the tree:
//
// Choose any edge [u, v] connecting the nodes u and v, and update their values as follows:
// nums[u] = nums[u] XOR k
// nums[v] = nums[v] XOR k
// Return the maximum possible sum of the values Bogdan can achieve by performing the operation any number of times.
//
// Example 1:
//
// Input: nums = [1,2,1], k = 3, edges = [[0,1],[0,2]]
// Output: 6
// Explanation: Bogdan can achieve the maximum sum of 6 using a single operation:
// - Choose the edge [0,2]. nums[0] and nums[2] become: 1 XOR 3 = 2, and the array nums becomes: [1,2,1] -> [2,2,2].
// The total sum of values is 2 + 2 + 2 = 6.
// It can be shown that 6 is the maximum achievable sum of values.
//
// Example 2:
//
// Input: nums = [2,3], k = 7, edges = [[0,1]]
// Output: 9
// Explanation: Bogdan can achieve the maximum sum of 9 using a single operation:
// - Choose the edge [0,1]. nums[0] becomes: 2 XOR 7 = 5 and nums[1] become: 3 XOR 7 = 4, and the array nums becomes: [2,3] -> [5,4].
// The total sum of values is 5 + 4 = 9.
// It can be shown that 9 is the maximum achievable sum of values.
//
// Example 3:
//
// Input: nums = [7,7,7,7,7,7], k = 3, edges = [[0,1],[0,2],[0,3],[0,4],[0,5]]
// Output: 42
// Explanation: The maximum achievable sum is 42 which can be achieved by Bogdan performing no operations.
//
// Constraints:
//
// 2 <= n == nums.length <= 2 * 10^4
// 1 <= k <= 10^9
// 0 <= nums[i] <= 10^9
// edges.length == n - 1
// edges[i].length == 2
// 0 <= edges[i][0], edges[i][1] <= n - 1
// The input is generated such that edges represent a valid tree.
//

struct Solution;

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, _edges: Vec<Vec<i32>>) -> i64 {
        let nums: Vec<i64> = nums.iter().map(|&x| x as i64).collect();
        let k = k as i64;
        let mut sum = 0;
        let mut cnt = 0;
        let mut sacrifice = i64::MAX;
        for &n in nums.iter() {
            let tmp = n ^ k;
            sum += tmp.max(n);
            cnt += if tmp > n { 1 } else { 0 };
            sacrifice = sacrifice.min((n - tmp).abs());
        }
        sum - if cnt % 2 == 1 { sacrifice } else { 0 }
    }
}

#[test]
fn test() {
    let nums = vec![1, 2, 1];
    let k = 3;
    let edges = vec![vec![0, 1], vec![0, 2]];
    let res = 6;
    assert_eq!(Solution::maximum_value_sum(nums, k, edges), res);

    let nums = vec![2, 3];
    let k = 7;
    let edges = vec![vec![0, 1]];
    let res = 9;
    assert_eq!(Solution::maximum_value_sum(nums, k, edges), res);

    let nums = vec![7, 7, 7, 7, 7, 7];
    let k = 3;
    let edges = vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![0, 4], vec![0, 5]];
    let res = 42;
    assert_eq!(Solution::maximum_value_sum(nums, k, edges), res);
}
