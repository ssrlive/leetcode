#![allow(dead_code)]

/*

// 1636. Sort Array by Increasing Frequency
// https://leetcode.com/problems/sort-array-by-increasing-frequency/
// https://leetcode.cn/problems/sort-array-by-increasing-frequency/
//
// Easy
//
// Given an array of integers nums, sort the array in increasing order based on the frequency of the values. If multiple values have the same frequency, sort them in decreasing order.

Return the sorted array.

Example 1:

Input: nums = [1,1,2,2,2,3]
Output: [3,1,1,2,2,2]
Explanation: '3' has a frequency of 1, '1' has a frequency of 2, and '2' has a frequency of 3.

Example 2:

Input: nums = [2,3,1,3,2]
Output: [1,3,3,2,2]
Explanation: '2' and '3' both have a frequency of 2, so they are sorted in decreasing order.

Example 3:

Input: nums = [-1,1,-6,4,5,-6,1,4,1]
Output: [5,-1,4,4,-6,-6,1,1,1]

Constraints:

    1 <= nums.length <= 100
    -100 <= nums[i] <= 100
*/

struct Solution;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort();
        let mut freq = vec![];
        let mut i = 0;
        while i < nums.len() {
            let mut j = i + 1;
            while j < nums.len() && nums[j] == nums[i] {
                j += 1;
            }
            freq.push((nums[i], j - i));
            i = j;
        }
        freq.sort_by(|a, b| if a.1 == b.1 { b.0.cmp(&a.0) } else { a.1.cmp(&b.1) });
        let mut res = vec![];
        for (num, count) in freq {
            for _ in 0..count {
                res.push(num);
            }
        }
        res
    }
}

#[test]
fn test() {
    let nums = vec![1, 1, 2, 2, 2, 3];
    let res = vec![3, 1, 1, 2, 2, 2];
    assert_eq!(Solution::frequency_sort(nums), res);
    let nums = vec![2, 3, 1, 3, 2];
    let res = vec![1, 3, 3, 2, 2];
    assert_eq!(Solution::frequency_sort(nums), res);
    let nums = vec![-1, 1, -6, 4, 5, -6, 1, 4, 1];
    let res = vec![5, -1, 4, 4, -6, -6, 1, 1, 1];
    assert_eq!(Solution::frequency_sort(nums), res);
}
