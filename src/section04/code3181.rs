#![allow(dead_code)]

// 3181. Maximum Total Reward Using Operations II
// https://leetcode.com/problems/maximum-total-reward-using-operations-ii/
// https://leetcode.cn/problems/maximum-total-reward-using-operations-ii/
//
// Hard
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
//     1 <= rewardValues.length <= 5 * 10^4
//     1 <= rewardValues[i] <= 5 * 10^4
//

struct Solution;

impl Solution {
    pub fn max_total_reward(reward_values: Vec<i32>) -> i32 {
        let mut reward_values = reward_values;
        reward_values.sort_unstable();
        let mut dp = vec![0; 50000];
        let sz = reward_values.len();
        for i in 0..sz {
            let tmp_i = reward_values[i];
            if i == 0 || reward_values[i - 1] != tmp_i {
                let lim = tmp_i.min(reward_values[sz - 1] - tmp_i);
                for x in 0..lim {
                    let tmp = dp[x as usize];
                    dp[(tmp_i + tmp) as usize] = tmp_i + tmp;
                }
            }
        }
        reward_values[sz - 1] + *dp.iter().take(reward_values[sz - 1] as usize).max().unwrap()
    }
}

#[test]
fn test() {
    let reward_values = vec![1, 1, 3, 3];
    let result = 4;
    assert_eq!(Solution::max_total_reward(reward_values), result);

    let reward_values = vec![1, 6, 4, 3, 2];
    let result = 11;
    assert_eq!(Solution::max_total_reward(reward_values), result);
}
