#![allow(dead_code)]

// 2552. Count Increasing Quadruplets
// https://leetcode.com/problems/count-increasing-quadruplets/
// https://leetcode.cn/problems/count-increasing-quadruplets/
//
// Hard
//
// Given a 0-indexed integer array nums of size n containing all numbers from 1 to n, return the number of increasing quadruplets.
//
// A quadruplet (i, j, k, l) is increasing if:
//
//     0 <= i < j < k < l < n, and
//     nums[i] < nums[k] < nums[j] < nums[l].
//
// Example 1:
//
// Input: nums = [1,3,2,4,5]
// Output: 2
// Explanation:
// - When i = 0, j = 1, k = 2, and l = 3, nums[i] < nums[k] < nums[j] < nums[l].
// - When i = 0, j = 1, k = 2, and l = 4, nums[i] < nums[k] < nums[j] < nums[l].
// There are no other quadruplets, so we return 2.
//
// Example 2:
//
// Input: nums = [1,2,3,4]
// Output: 0
// Explanation: There exists only one quadruplet with i = 0, j = 1, k = 2, l = 3, but since nums[j] < nums[k], we return 0.
//
// Constraints:
//
// -    4 <= nums.length <= 4000
// -    1 <= nums[i] <= nums.length
// -    All the integers of nums are unique. nums is a permutation.
//

struct Solution;

impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i64 {
        let n = nums.len();
        let mut lessthan = vec![vec![0; n + 1]; n + 1];
        for item in lessthan.iter_mut() {
            item[0] = 0;
        }
        for i in 1..=n {
            for (j, item) in lessthan.iter_mut().enumerate() {
                item[i] = item[i - 1] + (nums[i - 1] < j as i32) as i32;
            }
        }
        let mut re = 0;
        for i in 0..n {
            for j in i + 1..n {
                if nums[i] > nums[j] {
                    let g = n as i32 - nums[i] - (j as i32 - lessthan[nums[i] as usize][j + 1]);
                    let l = lessthan[nums[j] as usize][i + 1];
                    re += g as i64 * l as i64;
                }
            }
        }
        re
    }
}

#[test]
fn test() {
    let cases = vec![(vec![1, 3, 2, 4, 5], 2), (vec![1, 2, 3, 4], 0)];
    for (nums, expect) in cases {
        assert_eq!(Solution::count_quadruplets(nums), expect);
    }
}
