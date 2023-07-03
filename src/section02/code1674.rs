#![allow(dead_code)]

/*

// 1674. Minimum Moves to Make Array Complementary
// https://leetcode.com/problems/minimum-moves-to-make-array-complementary/
// https://leetcode.cn/problems/minimum-moves-to-make-array-complementary/
//
// Medium
//
// You are given an integer array nums of even length n and an integer limit. In one move, you can replace any integer from nums with another integer between 1 and limit, inclusive.

The array nums is complementary if for all indices i (0-indexed), nums[i] + nums[n - 1 - i] equals the same number. For example, the array [1,2,3,4] is complementary because for all indices i, nums[i] + nums[n - 1 - i] = 5.

Return the minimum number of moves required to make nums complementary.

Example 1:

Input: nums = [1,2,4,3], limit = 4
Output: 1
Explanation: In 1 move, you can change nums to [1,2,2,3] (underlined elements are changed).
nums[0] + nums[3] = 1 + 3 = 4.
nums[1] + nums[2] = 2 + 2 = 4.
nums[2] + nums[1] = 2 + 2 = 4.
nums[3] + nums[0] = 3 + 1 = 4.
Therefore, nums[i] + nums[n-1-i] = 4 for every i, so nums is complementary.

Example 2:

Input: nums = [1,2,2,1], limit = 2
Output: 2
Explanation: In 2 moves, you can change nums to [2,2,2,2]. You cannot change any number to 3 since 3 > limit.

Example 3:

Input: nums = [1,2,1,2], limit = 2
Output: 0
Explanation: nums is already complementary.

Constraints:

    n == nums.length
    2 <= n <= 10^5
    1 <= nums[i] <= limit <= 10^5
    n is even.
*/

struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let mut v = vec![0; 2 * limit as usize + 2];
        let mut ans = std::i32::MAX;
        for i in 0..n / 2 {
            let l = nums[i].min(nums[n - i - 1]) + 1;
            let mid = nums[i] + nums[n - i - 1];
            let r = nums[i].max(nums[n - i - 1]) + limit + 1;
            v[2] += 2;
            v[l as usize] -= 1;
            v[mid as usize] -= 1;
            v[mid as usize + 1] += 1;
            v[r as usize] += 1;
        }
        for i in 2..2 * limit as usize + 2 {
            v[i] += v[i - 1];
            ans = ans.min(v[i]);
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 2, 4, 3], 4, 1), (vec![1, 2, 2, 1], 2, 2), (vec![1, 2, 1, 2], 2, 0)];
    for (nums, limit, expected) in cases {
        assert_eq!(Solution::min_moves(nums, limit), expected);
    }
}
