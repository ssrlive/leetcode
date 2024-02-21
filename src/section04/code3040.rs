#![allow(dead_code)]

// 3040. Maximum Number of Operations With the Same Score II
// https://leetcode.com/problems/maximum-number-of-operations-with-the-same-score-ii/
// https://leetcode.cn/problems/maximum-number-of-operations-with-the-same-score-ii/
//
// Medium
//
// Given an array of integers called nums, you can perform any of the following operation while nums contains at least 2 elements:
//
// Choose the first two elements of nums and delete them.
// Choose the last two elements of nums and delete them.
// Choose the first and the last elements of nums and delete them.
// The score of the operation is the sum of the deleted elements.
//
// Your task is to find the maximum number of operations that can be performed, such that all operations have the same score.
//
// Return the maximum number of operations possible that satisfy the condition mentioned above.
//
// Example 1:
//
// Input: nums = [3,2,1,2,3,4]
// Output: 3
// Explanation: We perform the following operations:
// - Delete the first two elements, with score 3 + 2 = 5, nums = [1,2,3,4].
// - Delete the first and the last elements, with score 1 + 4 = 5, nums = [2,3].
// - Delete the first and the last elements, with score 2 + 3 = 5, nums = [].
// We are unable to perform any more operations as nums is empty.
//
// Example 2:
//
// Input: nums = [3,2,6,1,4]
// Output: 2
// Explanation: We perform the following operations:
// - Delete the first two elements, with score 3 + 2 = 5, nums = [6,1,4].
// - Delete the last two elements, with score 1 + 4 = 5, nums = [6].
// It can be proven that we can perform at most 2 operations.
//
// Constraints:
//
// 2 <= nums.length <= 2000
// 1 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn max_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 1;
        let n = nums.len();

        for (l, r) in [(0, 1), (0, n - 1), (n - 2, n - 1)] {
            let mut memo = vec![vec![-1; n]; n];
            ans = ans.max(Self::dp(0, n - 1, nums[l] + nums[r], &nums, &mut memo));
        }

        ans
    }

    fn dp(l: usize, r: usize, target: i32, nums: &Vec<i32>, memo: &mut Vec<Vec<i32>>) -> i32 {
        if r > nums.len() || l >= r {
            return 0;
        };
        if memo[l][r] != -1 {
            return memo[l][r];
        };

        let mut ans = 0;

        if nums[l] + nums[l + 1] == target {
            ans = ans.max(1 + Self::dp(l + 2, r, target, nums, memo));
        }
        if nums[r] + nums[r - 1] == target {
            ans = ans.max(1 + Self::dp(l, r - 2, target, nums, memo));
        }
        if nums[l] + nums[r] == target {
            ans = ans.max(1 + Self::dp(l + 1, r - 1, target, nums, memo));
        }

        memo[l][r] = ans;
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_operations(vec![3, 2, 1, 2, 3, 4]), 3);
    assert_eq!(Solution::max_operations(vec![3, 2, 6, 1, 4]), 2);
}
