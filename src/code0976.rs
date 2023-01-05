#![allow(dead_code)]

// 976. Largest Perimeter Triangle
// https://leetcode.com/problems/largest-perimeter-triangle/
// https://leetcode.cn/problems/largest-perimeter-triangle/
//
// Given an integer array nums, return the largest perimeter of a triangle with a non-zero area, formed from three of these lengths.
// If it is impossible to form any triangle of a non-zero area, return 0.
//
// Example 1:
//
// Input: nums = [2,1,2]
// Output: 5
// Explanation: You can form a triangle with three side lengths: 1, 2, and 2.
//
// Example 2:
//
// Input: nums = [1,2,1,10]
// Output: 0
// Explanation:
// You cannot use the side lengths 1, 1, and 2 to form a triangle.
// You cannot use the side lengths 1, 1, and 10 to form a triangle.
// You cannot use the side lengths 1, 2, and 10 to form a triangle.
// As we cannot use any three side lengths to form a triangle of non-zero area, we return 0.
//
// Constraints:
//
// - 3 <= nums.length <= 10^4
// - 1 <= nums[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn largest_perimeter(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable_by(|a, b| b.cmp(a));
        match nums.windows(3).find(|l| l[0] < l[1] + l[2]) {
            Some(l) => l.iter().sum(),
            None => 0,
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::largest_perimeter(vec![2, 1, 2]), 5);
    assert_eq!(Solution::largest_perimeter(vec![1, 2, 1, 10]), 0);
}
