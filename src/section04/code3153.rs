#![allow(dead_code)]

// 3153. Sum of Digit Differences of All Pairs
// https://leetcode.com/problems/sum-of-digit-differences-of-all-pairs/
// https://leetcode.cn/problems/sum-of-digit-differences-of-all-pairs/
//
// Medium
//
// You are given an array nums consisting of positive integers where all integers have the same number of digits.
//
// The digit difference between two integers is the count of different digits that are in the same position in the two integers.
//
// Return the sum of the digit differences between all pairs of integers in nums.
//
// Example 1:
//
// Input: nums = [13,23,12]
//
// Output: 4
//
// Explanation:
// We have the following:
// - The digit difference between 13 and 23 is 1.
// - The digit difference between 13 and 12 is 1.
// - The digit difference between 23 and 12 is 2.
// So the total sum of digit differences between all pairs of integers is 1 + 1 + 2 = 4.
//
// Example 2:
//
// Input: nums = [10,10,10,10]
//
// Output: 0
//
// Explanation:
// All the integers in the array are the same. So the total sum of digit differences between all pairs of integers will be 0.
//
// Constraints:
//
// 2 <= nums.length <= 10^5
// 1 <= nums[i] < 10^9
// All integers in nums have the same number of digits.
//

struct Solution;

impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let mut ans = 0;
        let mut freq = vec![vec![0; 10]; 10];
        for &n in &nums {
            let mut n = n;
            for freq_i in freq.iter_mut().take(10) {
                freq_i[n as usize % 10] += 1;
                n /= 10;
                if n == 0 {
                    break;
                }
            }
        }
        for freq_i in freq.iter() {
            for &count in freq_i.iter() {
                ans += count as i64 * (nums.len() as i64 - count as i64);
            }
        }
        ans / 2
    }
}

#[test]
fn test() {
    let nums = vec![13, 23, 12];
    let res = 4;
    assert_eq!(Solution::sum_digit_differences(nums), res);

    let nums = vec![10, 10, 10, 10];
    let res = 0;
    assert_eq!(Solution::sum_digit_differences(nums), res);
}
