#![allow(dead_code)]

// 1330. Reverse Subarray To Maximize Array Value
// https://leetcode.com/problems/reverse-subarray-to-maximize-array-value/
// https://leetcode.cn/problems/reverse-subarray-to-maximize-array-value/
//
// Hard
//
// You are given an integer array nums. The value of this array is defined as the sum of
// |nums[i] - nums[i + 1]| for all 0 <= i < nums.length - 1.
//
// You are allowed to select any subarray of the given array and reverse it. You can perform this operation only once.
//
// Find maximum possible value of the final array.
//
// Example 1:
//
// Input: nums = [2,3,1,5,4]
// Output: 10
// Explanation: By reversing the subarray [3,1,5] the array becomes [2,5,1,3,4] whose value is 10.
//
// Example 2:
//
// Input: nums = [2,4,9,24,2,1,10]
// Output: 68
//
// Constraints:
//
// -    1 <= nums.length <= 3 * 10^4
// -    -10^5 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        let mut res = 0;
        let mut min2 = 123456;
        let mut max2 = -123456;
        let n = nums.len();
        for i in 0..n - 1 {
            let a = nums[i];
            let b = nums[i + 1];
            total += (a - b).abs();
            res = res.max((nums[0] - b).abs() - (a - b).abs());
            res = res.max((nums[n - 1] - a).abs() - (a - b).abs());
            min2 = min2.min(a.max(b));
            max2 = max2.max(a.min(b));
        }
        total + res.max((max2 - min2) * 2)
    }
}

#[test]
fn test() {
    let nums = vec![2, 3, 1, 5, 4];
    let res = 10;
    assert_eq!(Solution::max_value_after_reverse(nums), res);
    let nums = vec![2, 4, 9, 24, 2, 1, 10];
    let res = 68;
    assert_eq!(Solution::max_value_after_reverse(nums), res);
}
