#![allow(dead_code)]

// 3326. Minimum Division Operations to Make Array Non Decreasing
// https://leetcode.com/problems/minimum-division-operations-to-make-array-non-decreasing/
// https://leetcode.cn/problems/minimum-division-operations-to-make-array-non-decreasing/
//
// Medium
//
// You are given an integer array nums.
//
// Any positive divisor of a natural number x that is strictly less than x is called a proper divisor of x.
// For example, 2 is a proper divisor of 4, while 6 is not a proper divisor of 6.
//
// You are allowed to perform an operation any number of times on nums, where in each operation
// you select any one element from nums and divide it by its greatest proper divisor.
//
// Return the minimum number of operations required to make the array non-decreasing.
//
// If it is not possible to make the array non-decreasing using any number of operations, return -1.
//
// Example 1:
//
// Input: nums = [25,7]
//
// Output: 1
//
// Explanation:
//
// Using a single operation, 25 gets divided by 5 and nums becomes [5, 7].
//
// Example 2:
//
// Input: nums = [7,7,6]
//
// Output: -1
//
// Example 3:
//
// Input: nums = [1,1,1,1]
//
// Output: 0
//
// Constraints:
//
// 1 <= nums.length <= 10^5
// 1 <= nums[i] <= 10^6
//

struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut result = 0;
        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                if nums[i] % 2 == 0 {
                    nums[i] = 2;
                } else {
                    for divisor in 3..=nums[i + 1] {
                        if nums[i] % divisor == 0 {
                            nums[i] = divisor;
                            break;
                        }
                    }
                }
                if nums[i] > nums[i + 1] {
                    return -1;
                }
                result += 1;
            }
        }
        result
    }
}

#[test]
fn test() {
    let nums = vec![25, 7];
    let result = 1;
    assert_eq!(Solution::min_operations(nums), result);

    let nums = vec![7, 7, 6];
    let result = -1;
    assert_eq!(Solution::min_operations(nums), result);

    let nums = vec![1, 1, 1, 1];
    let result = 0;
    assert_eq!(Solution::min_operations(nums), result);
}
