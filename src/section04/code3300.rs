#![allow(dead_code)]

// 3300. Minimum Element After Replacement With Digit Sum
// https://leetcode.com/problems/minimum-element-after-replacement-with-digit-sum/
// https://leetcode.cn/problems/minimum-element-after-replacement-with-digit-sum/
//
// Easy
//
// You are given an integer array nums.
//
// You replace each element in nums with the sum of its digits.
//
// Return the minimum element in nums after all replacements.
//
// Example 1:
//
// Input: nums = [10,12,13,14]
//
// Output: 1
//
// Explanation:
//
// nums becomes [1, 3, 4, 5] after all replacements, with minimum element 1.
//
// Example 2:
//
// Input: nums = [1,2,3,4]
//
// Output: 1
//
// Explanation:
//
// nums becomes [1, 2, 3, 4] after all replacements, with minimum element 1.
//
// Example 3:
//
// Input: nums = [999,19,199]
//
// Output: 10
//
// Explanation:
//
// nums becomes [27, 10, 19] after all replacements, with minimum element 10.
//
// Constraints:
//
//     1 <= nums.length <= 100
//     1 <= nums[i] <= 10^4
//

struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.iter().map(|&n| Self::digit_sum(n)).min().unwrap()
    }

    fn digit_sum(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            res += n % 10;
            n /= 10;
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_element(vec![10, 12, 13, 14]), 1);
    assert_eq!(Solution::min_element(vec![1, 2, 3, 4]), 1);
    assert_eq!(Solution::min_element(vec![999, 19, 199]), 10);
}
