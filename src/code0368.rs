#![allow(dead_code)]

// 368. Largest Divisible Subset
// https://leetcode.com/problems/largest-divisible-subset/
//
// Given a set of distinct positive integers nums, return the largest subset answer such that every pair (answer[i], answer[j]) of elements in this subset satisfies:
//
// answer[i] % answer[j] == 0, or
// answer[j] % answer[i] == 0
// If there are multiple solutions, return any of them.
//
// Example 1:
//
// Input: nums = [1,2,3]
// Output: [1,2]
// Explanation: [1,3] is also accepted.
//
// Example 2:
//
// Input: nums = [1,2,4,8]
// Output: [1,2,4,8]
//
// Constraints:
//
// 1 <= nums.length <= 1000
// 1 <= nums[i] <= 2 * 10^9
// All the integers in nums are unique.

struct Solution;

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() <= 1 {
            return nums;
        }
        let mut nums: Vec<i32> = nums;
        nums.sort();
        let mut ans: Vec<(usize, i32)> = Vec::new();

        let mut max: i32 = 0;
        let mut idx_ans: usize = 0;

        for i in 0..nums.len() {
            ans.push((i, 1));
            for j in 0..i {
                if nums[i] % nums[j] == 0 && ans[i].1 <= ans[j].1 {
                    ans[i].0 = j;
                    ans[i].1 = ans[j].1 + 1;

                    if max < ans[i].1 {
                        max = ans[i].1;
                        idx_ans = i;
                    }
                }
            }
        }

        let mut result: Vec<i32> = Vec::new();
        let mut idx = idx_ans;

        while ans[idx].1 > 1 {
            result.push(nums[idx]);
            idx = ans[idx].0;
        }
        result.push(nums[idx]);

        result
    }
}

#[test]
fn test() {
    assert_eq!(Solution::largest_divisible_subset(vec![1, 2, 3]), vec![2, 1]);
    assert_eq!(Solution::largest_divisible_subset(vec![1, 2, 4, 8]), vec![8, 4, 2, 1]);
}
