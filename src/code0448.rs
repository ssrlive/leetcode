#![allow(dead_code)]

// 448. Find All Numbers Disappeared in an Array
// https://leetcode.com/problems/find-all-numbers-disappeared-in-an-array/
// https://leetcode.cn/problems/find-all-numbers-disappeared-in-an-array/
//
// Given an array nums of n integers where nums[i] is in the range [1, n], return an array of all the integers
// in the range [1, n] that do not appear in nums.
//
// Example 1:
//
// Input: nums = [4,3,2,7,8,2,3,1]
// Output: [5,6]
//
// Example 2:
//
// Input: nums = [1,1]
// Output: [2]
//
// Constraints:
//
// - n == nums.length
// - 1 <= n <= 105
// - 1 <= nums[i] <= n
//
// Follow up: Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.
//

struct Solution;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        for i in 0..nums.len() {
            let j = nums[i].unsigned_abs() as usize - 1;
            if nums[j] > 0 {
                nums[j] *= -1;
            }
        }
        let mut ans = Vec::new();
        for (i, item) in nums.iter().enumerate() {
            if *item > 0 {
                ans.push((i + 1) as i32);
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![5, 6]
    );
    assert_eq!(Solution::find_disappeared_numbers(vec![1, 1]), vec![2]);
}
