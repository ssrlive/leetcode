#![allow(dead_code)]

// 3591. Check if Any Element Has Prime Frequency
// https://leetcode.com/problems/check-if-any-element-has-prime-frequency/
// https://leetcode.cn/problems/check-if-any-element-has-prime-frequency/
//
// Easy
//
// You are given an integer array nums.
//
// Return true if the frequency of any element of the array is prime, otherwise, return false.
//
// The frequency of an element x is the number of times it occurs in the array.
//
// A prime number is a natural number greater than 1 with only two factors, 1 and itself.
//
// Example 1:
//
// Input: nums = [1,2,3,4,5,4]
//
// Output: true
//
// Explanation:
//
// 4 has a frequency of two, which is a prime number.
//
// Example 2:
//
// Input: nums = [1,2,3,4,5]
//
// Output: false
//
// Explanation:
//
// All elements have a frequency of one.
//
// Example 3:
//
// Input: nums = [2,2,2,4,4]
//
// Output: true
//
// Explanation:
//
// Both 2 and 4 have a prime frequency.
//
// Constraints:
//
//     1 <= nums.length <= 100
//     0 <= nums[i] <= 100
//

struct Solution;

impl Solution {
    pub fn check_prime_frequency(nums: Vec<i32>) -> bool {
        let mut counts: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
        for &num in &nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        for &count in counts.values() {
            if count == 2 || count == 3 || count == 5 || count == 7 {
                return true;
            }
            if count > 1 && (count % 2 != 0 && count % 3 != 0 && count % 5 != 0 && count % 7 != 0) {
                return true;
            }
        }
        false
    }
}

#[test]
fn test() {
    assert!(Solution::check_prime_frequency(vec![1, 2, 3, 4, 5, 4]));
    assert!(!Solution::check_prime_frequency(vec![1, 2, 3, 4, 5]));
    assert!(Solution::check_prime_frequency(vec![2, 2, 2, 4, 4]));
}
