#![allow(dead_code)]

// 3550. Smallest Index With Digit Sum Equal to Index
// https://leetcode.com/problems/smallest-index-with-digit-sum-equal-to-index/
// https://leetcode.cn/problems/smallest-index-with-digit-sum-equal-to-index/
//
// Easy
//
// You are given an integer array nums.
//
// Return the smallest index i such that the sum of the digits of nums[i] is equal to i.
//
// If no such index exists, return -1.
//
// Example 1:
//
// Input: nums = [1,3,2]
//
// Output: 2
//
// Explanation:
//
//     For nums[2] = 2, the sum of digits is 2, which is equal to index i = 2. Thus, the output is 2.
//
// Example 2:
//
// Input: nums = [1,10,11]
//
// Output: 1
//
// Explanation:
//
//     For nums[1] = 10, the sum of digits is 1 + 0 = 1, which is equal to index i = 1.
//     For nums[2] = 11, the sum of digits is 1 + 1 = 2, which is equal to index i = 2.
//     Since index 1 is the smallest, the output is 1.
//
// Example 3:
//
// Input: nums = [1,2,3]
//
// Output: -1
//
// Explanation:
//
//     Since no index satisfies the condition, the output is -1.
//
// Constraints:
//
//     1 <= nums.length <= 100
//     0 <= nums[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        fn digit_sum(n: i32) -> i32 {
            let mut sum = 0;
            let mut n = n;
            while n > 0 {
                sum += n % 10;
                n /= 10;
            }
            sum
        }

        for (i, &num) in nums.iter().enumerate() {
            if i as i32 == digit_sum(num) {
                return i as i32;
            }
        }
        -1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_index(vec![1, 3, 2]), 2);
    assert_eq!(Solution::smallest_index(vec![1, 10, 11]), 1);
    assert_eq!(Solution::smallest_index(vec![1, 2, 3]), -1);
}
