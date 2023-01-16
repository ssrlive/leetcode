#![allow(dead_code)]

// 45. Jump Game II
// https://leetcode.com/problems/jump-game-ii/
// https://leetcode.cn/problems/jump-game-ii/
//
// You are given a 0-indexed array of integers nums of length n. You are initially positioned at nums[0].
//
// Each element nums[i] represents the maximum length of a forward jump from index i.
// In other words, if you are at nums[i], you can jump to any nums[i + j] where:
//
// - 0 <= j <= nums[i] and
// - i + j < n
//
// Return the minimum number of jumps to reach nums[n - 1]. The test cases are generated such that you can reach nums[n - 1].
//
// Example 1:
//
// Input: nums = [2,3,1,1,4]
// Output: 2
// Explanation: The minimum number of jumps to reach the last index is 2. Jump 1 step from index 0 to 1, then 3 steps to the last index.
//
// Example 2:
//
// Input: nums = [2,3,0,1,4]
// Output: 2
//
// Constraints:
//
// - 1 <= nums.length <= 10^4
// - 0 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut jumps = 0;
        let mut i = 0; // Current position
        let mut explored = 0; // The highest index we have explored
        while explored < n - 1 {
            // Choose where to jump based on how far the next position can take us
            let to_explore = (n - 1).min(i + nums[i] as usize);
            i = (explored + 1..=to_explore)
                .max_by(|&j, &k| (j + nums[j] as usize).cmp(&(k + nums[k] as usize)))
                .unwrap();
            explored = to_explore;
            jumps += 1;
        }
        jumps
    }
}

#[test]
fn test() {
    assert_eq!(Solution::jump(vec![2, 3, 1, 1, 4]), 2);
    assert_eq!(Solution::jump(vec![2, 3, 0, 1, 4]), 2);
}
