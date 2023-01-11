#![allow(dead_code)]

// 1031. Maximum Sum of Two Non-Overlapping Subarrays
// https://leetcode.com/problems/maximum-sum-of-two-non-overlapping-subarrays/
// https://leetcode.cn/problems/maximum-sum-of-two-non-overlapping-subarrays/
//
// Given an integer array nums and two integers firstLen and secondLen, return the maximum sum of elements in two non-overlapping subarrays with lengths firstLen and secondLen.
//
// The array with length firstLen could occur before or after the array with length secondLen, but they have to be non-overlapping.
//
// A subarray is a contiguous part of an array.
//
// Example 1:
//
// Input: nums = [0,6,5,2,2,5,1,9,4], firstLen = 1, secondLen = 2
// Output: 20
// Explanation: One choice of subarrays is [9] with length 1, and [6,5] with length 2.
//
// Example 2:
//
// Input: nums = [3,8,1,3,2,1,8,9,0], firstLen = 3, secondLen = 2
// Output: 29
// Explanation: One choice of subarrays is [3,8,1] with length 3, and [8,9] with length 2.
//
// Example 3:
//
// Input: nums = [2,1,5,6,0,9,5,0,3,8], firstLen = 4, secondLen = 3
// Output: 31
// Explanation: One choice of subarrays is [5,6,0,9] with length 4, and [0,3,8] with length 3.
//
// Constraints:
//
// - 1 <= firstLen, secondLen <= 1000
// - 2 <= firstLen + secondLen <= 1000
// - firstLen + secondLen <= nums.length <= 1000
// - 0 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn max_sum_two_no_overlap(nums: Vec<i32>, first_len: i32, second_len: i32) -> i32 {
        let n = nums.len();
        let fl = first_len as usize;
        let sl = second_len as usize;
        let mut memol = vec![0; n];
        let mut memor = vec![0; n];

        let mut temp = 0;
        for &num in nums.iter().take(fl) {
            temp += num;
        }
        memol[fl - 1] = temp;
        for i in fl..n {
            temp -= nums[i - fl];
            temp += nums[i];
            memol[i] = std::cmp::max(memol[i - 1], temp);
        }

        let mut temp = 0;
        for i in (n - fl..n).rev() {
            temp += nums[i];
        }
        memor[n - fl] = temp;
        for i in (0..n - fl).rev() {
            temp -= nums[i + fl];
            temp += nums[i];
            memor[i] = std::cmp::max(memor[i + 1], temp);
        }

        let mut temp = 0;
        for &num in nums.iter().take(sl) {
            temp += num;
        }
        let mut max = temp + memor[sl];

        for i in sl..n {
            temp -= nums[i - sl];
            temp += nums[i];

            let lv = if sl <= i { memol[i - sl] } else { 0 };
            let rv = if i + 1 + fl < n { memor[i + 1] } else { 0 };
            let add = std::cmp::max(lv, rv);
            max = std::cmp::max(max, temp + add);
        }

        max
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2, 20),
        (vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2, 29),
        (vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3, 31),
    ];
    for (nums, first_len, second_len, expected) in cases {
        assert_eq!(Solution::max_sum_two_no_overlap(nums, first_len, second_len), expected);
    }
}
