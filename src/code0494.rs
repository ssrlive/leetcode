#![allow(dead_code)]

// 494. Target Sum
// https://leetcode.com/problems/target-sum/
// https://leetcode.cn/problems/target-sum/
//
// You are given an integer array nums and an integer target.
//
// You want to build an expression out of nums by adding one of the symbols '+' and '-' before each integer in nums and then concatenate all the integers.
//
// For example, if nums = [2, 1], you can add a '+' before 2 and a '-' before 1 and concatenate them to build the expression "+2-1".
// Return the number of different expressions that you can build, which evaluates to target.
//
// Example 1:
//
// Input: nums = [1,1,1,1,1], target = 3
// Output: 5
// Explanation: There are 5 ways to assign symbols to make the sum of nums be target 3.
// -1 + 1 + 1 + 1 + 1 = 3
// +1 - 1 + 1 + 1 + 1 = 3
// +1 + 1 - 1 + 1 + 1 = 3
// +1 + 1 + 1 - 1 + 1 = 3
// +1 + 1 + 1 + 1 - 1 = 3
//
// Example 2:
//
// Input: nums = [1], target = 1
// Output: 1
//
// Constraints:
//
// - 1 <= nums.length <= 20
// - 0 <= nums[i] <= 1000
// - 0 <= sum(nums[i]) <= 1000
// - -1000 <= target <= 1000
//

struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let mut memo = HashMap::new();
        Self::find_target_sum_ways_rec(&nums, 0, target, &mut memo)
    }

    fn find_target_sum_ways_rec(nums: &[i32], i: usize, target: i32, memo: &mut HashMap<(usize, i32), i32>) -> i32 {
        if let Some(&ways) = memo.get(&(i, target)) {
            return ways;
        }
        if i == nums.len() {
            if target == 0 {
                return 1;
            } else {
                return 0;
            }
        }
        let ways = Self::find_target_sum_ways_rec(nums, i + 1, target - nums[i], memo)
            + Self::find_target_sum_ways_rec(nums, i + 1, target + nums[i], memo);
        memo.insert((i, target), ways);
        ways
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
}
