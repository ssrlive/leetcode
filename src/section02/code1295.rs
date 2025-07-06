#![allow(dead_code)]

// 1295. Find Numbers with Even Number of Digits
// https://leetcode.com/problems/find-numbers-with-even-number-of-digits/
// https://leetcode.cn/problems/find-numbers-with-even-number-of-digits/
//
// Easy
//
// Given an array nums of integers, return how many of them contain an even number of digits.
//
// Example 1:
//
// Input: nums = [12,345,2,6,7896]
// Output: 2
// Explanation:
// 12 contains 2 digits (even number of digits).
// 345 contains 3 digits (odd number of digits).
// 2 contains 1 digit (odd number of digits).
// 6 contains 1 digit (odd number of digits).
// 7896 contains 4 digits (even number of digits).
// Therefore only 12 and 7896 contain an even number of digits.
//
// Example 2:
//
// Input: nums = [555,901,482,1771]
// Output: 1
// Explanation:
// Only 1771 contains an even number of digits.
//
// Constraints:
//
// -    1 <= nums.length <= 500
// -    1 <= nums[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        nums.iter().map(|n| n.to_string()).filter(|v| v.len().is_multiple_of(2)).count() as i32
    }

    pub fn find_numbers_2(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for num in nums {
            let mut n = num;
            let mut cnt = 0;
            while n > 0 {
                n /= 10;
                cnt += 1;
            }
            if cnt % 2 == 0 {
                ans += 1;
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_numbers(vec![12, 345, 2, 6, 7896]), 2);
    assert_eq!(Solution::find_numbers(vec![555, 901, 482, 1771]), 1);
}
