#![allow(dead_code)]

// 324. Wiggle Sort II
// https://leetcode.com/problems/wiggle-sort-ii/
// https://leetcode.cn/problems/wiggle-sort-ii/
//
// Given an integer array nums, reorder it such that nums[0] < nums[1] > nums[2] < nums[3]....
//
// You may assume the input array always has a valid answer.
//
// Example 1:
//
// Input: nums = [1,5,1,1,6,4]
// Output: [1,6,1,5,1,4]
// Explanation: [1,4,1,5,1,6] is also accepted.
//
// Example 2:
//
// Input: nums = [1,3,2,2,3,1]
// Output: [2,3,1,3,1,2]
//
// Constraints:
//
// - 1 <= nums.length <= 5 * 10^4
// - 0 <= nums[i] <= 5000
// - It is guaranteed that there will be an answer for the given input nums.
//
// Follow Up: Can you do it in O(n) time and/or in-place with O(1) extra space?
//

struct Solution;

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        let mut tmp = nums.clone();
        tmp.sort_unstable();
        let len = nums.len() as i32;
        let mut x = len / 2;
        let mut y = len - x;
        let mut k = len - 1;
        let mut i = 1;
        while x > 0 {
            nums[i] = tmp[k as usize];
            k -= 1;
            i += 2;
            x -= 1;
        }
        i = 0;
        while y > 0 {
            nums[i] = tmp[k as usize];
            k -= 1;
            i += 2;
            y -= 1;
        }
    }
}

#[test]
fn test_wiggle_sort() {
    let mut nums = vec![1, 5, 1, 1, 6, 4];
    Solution::wiggle_sort(&mut nums);
    assert_eq!(nums, vec![1, 6, 1, 5, 1, 4]);

    let mut nums = vec![1, 3, 2, 2, 3, 1];
    Solution::wiggle_sort(&mut nums);
    assert_eq!(nums, vec![2, 3, 1, 3, 1, 2]);
}
