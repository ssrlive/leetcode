#![allow(dead_code)]

// 912. Sort an Array
// https://leetcode.com/problems/sort-an-array/
// https://leetcode.cn/problems/sort-an-array/
//
// Given an array of integers nums, sort the array in ascending order and return it.
//
// You must solve the problem without using any built-in functions in O(nlog(n)) time complexity and with the smallest space complexity possible.
//
// Example 1:
//
// Input: nums = [5,2,3,1]
// Output: [1,2,3,5]
// Explanation: After sorting the array, the positions of some numbers are not changed (for example, 2 and 3), while the positions of other numbers are changed (for example, 1 and 5).
//
// Example 2:
//
// Input: nums = [5,1,1,2,0,0]
// Output: [0,0,1,1,2,5]
// Explanation: Note that the values of nums are not necessairly unique.
//
// Constraints:
//
// - 1 <= nums.length <= 5 * 10^4
// - -5 * 10^4 <= nums[i] <= 5 * 10^4
//

struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        fn merge_sort(src: &[i32]) -> Vec<i32> {
            if src.len() <= 1 {
                return src.to_vec();
            }
            let mid = src.len() / 2;
            let pt1 = merge_sort(&src[..mid]);
            let pt2 = merge_sort(&src[mid..src.len()]);
            let mut res = vec![];
            let mut i = 0;
            let mut j = 0;
            while i < pt1.len() || j < pt2.len() {
                if i < pt1.len() && j < pt2.len() {
                    if pt1[i] < pt2[j] {
                        res.push(pt1[i]);
                        i += 1;
                    } else {
                        res.push(pt2[j]);
                        j += 1;
                    }
                } else if i < pt1.len() {
                    res.push(pt1[i]);
                    i += 1;
                } else if j < pt2.len() {
                    res.push(pt2[j]);
                    j += 1;
                }
            }
            res
        }
        merge_sort(&nums)
    }
}

#[test]
fn test() {}
