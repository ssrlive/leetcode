#![allow(dead_code)]

/*

// 2091. Removing Minimum and Maximum From Array
// https://leetcode.com/problems/removing-minimum-and-maximum-from-array/
// https://leetcode.cn/problems/removing-minimum-and-maximum-from-array/
//
// Medium
//
// You are given a 0-indexed array of distinct integers nums.

There is an element in nums that has the lowest value and an element that has the highest value. We call them the minimum and maximum respectively. Your goal is to remove both these elements from the array.

A deletion is defined as either removing an element from the front of the array or removing an element from the back of the array.

Return the minimum number of deletions it would take to remove both the minimum and maximum element from the array.

Example 1:

Input: nums = [2,10,7,5,4,1,8,6]
Output: 5
Explanation:
The minimum element in the array is nums[5], which is 1.
The maximum element in the array is nums[1], which is 10.
We can remove both the minimum and maximum by removing 2 elements from the front and 3 elements from the back.
This results in 2 + 3 = 5 deletions, which is the minimum number possible.

Example 2:

Input: nums = [0,-4,19,1,8,-2,-3,5]
Output: 3
Explanation:
The minimum element in the array is nums[1], which is -4.
The maximum element in the array is nums[2], which is 19.
We can remove both the minimum and maximum by removing 3 elements from the front.
This results in only 3 deletions, which is the minimum number possible.

Example 3:

Input: nums = [101]
Output: 1
Explanation:
There is only one element in the array, which makes it both the minimum and maximum element.
We can remove it with 1 deletion.

Constraints:

    1 <= nums.length <= 10^5
    -10^5 <= nums[i] <= 10^5
    The integers in nums are distinct.
*/

struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let mut min = 0;
        let mut max = 0;

        for (i, &n) in nums.iter().enumerate() {
            if n < nums[min] {
                min = i;
            }
            if n > nums[max] {
                max = i;
            }
        }

        let d1 = (max + 1).max(min + 1);
        let d2 = (max + 1).min(min + 1);

        let a = nums.len() - d2 + 1;
        let b = d1;

        let c = d2 + (nums.len() - d1 + 1);

        a.min(b).min(c) as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 10, 7, 5, 4, 1, 8, 6], 5),
        (vec![0, -4, 19, 1, 8, -2, -3, 5], 3),
        (vec![101], 1),
    ];
    for (nums, expected) in cases {
        assert_eq!(Solution::minimum_deletions(nums), expected);
    }
}
