#![allow(dead_code)]

/*

// 1696. Jump Game VI
// https://leetcode.com/problems/jump-game-vi/
// https://leetcode.cn/problems/jump-game-vi/
//
// Medium
//
// You are given a 0-indexed integer array nums and an integer k.

You are initially standing at index 0. In one move, you can jump at most k steps forward without going outside the boundaries of the array. That is, you can jump from index i to any index in the range [i + 1, min(n - 1, i + k)] inclusive.

You want to reach the last index of the array (index n - 1). Your score is the sum of all nums[j] for each index j you visited in the array.

Return the maximum score you can get.

Example 1:

Input: nums = [1,-1,-2,4,-7,3], k = 2
Output: 7
Explanation: You can choose your jumps forming the subsequence [1,-1,4,3] (underlined above). The sum is 7.

Example 2:

Input: nums = [10,-5,-2,4,0,3], k = 3
Output: 17
Explanation: You can choose your jumps forming the subsequence [10,4,3] (underlined above). The sum is 17.

Example 3:

Input: nums = [1,-5,-20,4,-1,3,-6,-3], k = 2
Output: 0

Constraints:

    1 <= nums.length, k <= 10^5
    -10^4 <= nums[i] <= 10^4
*/

struct Solution;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; nums.len()];
        let mut q = std::collections::VecDeque::new();
        dp[0] = nums[0];
        q.push_back(0);
        for i in 1..nums.len() {
            while q.front().unwrap() < &(i as i32 - k) {
                q.pop_front();
            }
            dp[i] = dp[*q.front().unwrap() as usize] + nums[i];
            while !q.is_empty() && dp[*q.back().unwrap() as usize] < dp[i] {
                q.pop_back();
            }
            q.push_back(i as i32);
        }
        dp[nums.len() - 1]
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, -1, -2, 4, -7, 3], 2, 7),
        (vec![10, -5, -2, 4, 0, 3], 3, 17),
        (vec![1, -5, -20, 4, -1, 3, -6, -3], 2, 0),
    ];
    for (nums, k, expected) in cases {
        assert_eq!(Solution::max_result(nums, k), expected);
    }
}
