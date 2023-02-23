#![allow(dead_code)]

/*

// 1802. Maximum Value at a Given Index in a Bounded Array
// https://leetcode.com/problems/maximum-value-at-a-given-index-in-a-bounded-array/
// https://leetcode.cn/problems/maximum-value-at-a-given-index-in-a-bounded-array/
//
// Medium
//
// You are given three positive integers: n, index, and maxSum. You want to construct an array nums (0-indexed) that satisfies the following conditions:

    nums.length == n
    nums[i] is a positive integer where 0 <= i < n.
    abs(nums[i] - nums[i+1]) <= 1 where 0 <= i < n-1.
    The sum of all the elements of nums does not exceed maxSum.
    nums[index] is maximized.

Return nums[index] of the constructed array.

Note that abs(x) equals x if x >= 0, and -x otherwise.

Example 1:

Input: n = 4, index = 2,  maxSum = 6
Output: 2
Explanation: nums = [1,2,2,1] is one array that satisfies all the conditions.
There are no arrays that satisfy all the conditions and have nums[2] == 3, so 2 is the maximum nums[2].

Example 2:

Input: n = 6, index = 1,  maxSum = 10
Output: 3

Constraints:

    1 <= n <= maxSum <= 10^9
    0 <= index < n
*/

struct Solution;

impl Solution {
    pub fn max_value(n: i32, index: i32, max_sum: i32) -> i32 {
        fn test(n: i64, index: i64, a: i64) -> i64 {
            let b = a - index;
            let b = if b > 0 { b } else { 0 };
            let res = (a + b) * (a - b + 1) / 2;
            let b = a - ((n - 1) - index);
            let b = if b > 0 { b } else { 0 };
            let res = res + (a + b) * (a + 1 - b) / 2;
            res - a
        }

        let (n, index, max_sum) = (n as i64, index as i64, max_sum as i64);
        let mut left = 0;
        let mut right = max_sum - n;
        while left < right {
            let mid = (left + right + 1) / 2;
            if test(n, index, mid) <= max_sum - n {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        (left + 1) as _
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_value(4, 2, 6), 2);
    assert_eq!(Solution::max_value(6, 1, 10), 3);
    assert_eq!(Solution::max_value(3, 0, 815094800), 271698267);
}
