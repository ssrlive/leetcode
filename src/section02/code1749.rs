#![allow(dead_code)]

/*

// 1749. Maximum Absolute Sum of Any Subarray
Medium
1K
15
Companies

You are given an integer array nums. The absolute sum of a subarray [numsl, numsl+1, ..., numsr-1, numsr] is abs(numsl + numsl+1 + ... + numsr-1 + numsr).

Return the maximum absolute sum of any (possibly empty) subarray of nums.

Note that abs(x) is defined as follows:

    If x is a negative integer, then abs(x) = -x.
    If x is a non-negative integer, then abs(x) = x.

Example 1:

Input: nums = [1,-3,2,3,-4]
Output: 5
Explanation: The subarray [2,3] has absolute sum = abs(2+3) = abs(5) = 5.

Example 2:

Input: nums = [2,-5,1,-4,3,-2]
Output: 8
Explanation: The subarray [-5,1,-4] has absolute sum = abs(-5+1-4) = abs(-8) = 8.

Constraints:

    1 <= nums.length <= 10^5
    -10^4 <= nums[i] <= 10^4
*/

struct Solution;

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = 0;
        let mut min_sum = 0;
        let mut max = 0;
        let mut min = 0;
        for num in nums {
            max_sum += num;
            min_sum += num;
            if max_sum > max {
                max = max_sum;
            }
            if min_sum < min {
                min = min_sum;
            }
            if max_sum < 0 {
                max_sum = 0;
            }
            if min_sum > 0 {
                min_sum = 0;
            }
        }
        if max > -min {
            max
        } else {
            -min
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_absolute_sum(vec![1, -3, 2, 3, -4]), 5);
    assert_eq!(Solution::max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8);
}
