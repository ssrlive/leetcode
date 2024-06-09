#![allow(dead_code)]

// 3180. Maximum Total Reward Using Operations I
// https://leetcode.com/problems/maximum-total-reward-using-operations-i/
// https://leetcode.cn/problems/maximum-total-reward-using-operations-i/
//
// Medium
//
// You are given an integer array rewardValues of length n, representing the values of rewards.
//
// Initially, your total reward x is 0, and all indices are unmarked. You are allowed to perform the following operation any number of times:
//
//     Choose an unmarked index i from the range [0, n - 1].
//     If rewardValues[i] is greater than your current total reward x, then add rewardValues[i] to x (i.e., x = x + rewardValues[i]), and mark the index i.
//
// Return an integer denoting the maximum total reward you can collect by performing the operations optimally.
//
// Example 1:
//
// Input: rewardValues = [1,1,3,3]
//
// Output: 4
//
// Explanation:
//
// During the operations, we can choose to mark the indices 0 and 2 in order, and the total reward will be 4, which is the maximum.
//
// Example 2:
//
// Input: rewardValues = [1,6,4,3,2]
//
// Output: 11
//
// Explanation:
//
// Mark the indices 0, 2, and 1 in order. The total reward will then be 11, which is the maximum.
//
// Constraints:
//
//     1 <= rewardValues.length <= 2000
//     1 <= rewardValues[i] <= 2000
//

struct Solution;

impl Solution {
    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let n = reward_values.len();
        let mut reward_values = reward_values;
        reward_values.sort_unstable();
        let max_possible_score = reward_values[n - 1] * 2;

        let mut pre_dp = vec![false; max_possible_score as usize + 1];
        let mut cur_dp = vec![false; max_possible_score as usize + 1];
        pre_dp[0] = true;
        for i in 0..n {
            for j in 0..reward_values[i] {
                cur_dp[j as usize + reward_values[i] as usize] |= pre_dp[j as usize];
            }
            for j in 0..cur_dp.len() {
                pre_dp[j] |= cur_dp[j];
                cur_dp[j] = false;
            }
        }

        let mut ans = 0;
        for (i, &pre_dp_i) in pre_dp.iter().enumerate() {
            if pre_dp_i {
                ans = ans.max(i as i32);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let reward_values = vec![1, 1, 3, 3];
    let res = 4;
    assert_eq!(Solution::max_total_reward(reward_values), res);

    let reward_values = vec![1, 6, 4, 3, 2];
    let res = 11;
    assert_eq!(Solution::max_total_reward(reward_values), res);
}
