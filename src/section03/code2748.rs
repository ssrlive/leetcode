#![allow(dead_code)]

// 2748. Number of Beautiful Pairs
// https://leetcode.com/problems/number-of-beautiful-pairs/
// https://leetcode.cn/problems/number-of-beautiful-pairs/
//
// Easy
//
// You are given a 0-indexed integer array nums. A pair of indices i, j where 0 <= i < j < nums.length is called beautiful
// if the first digit of nums[i] and the last digit of nums[j] are coprime.
//
// Return the total number of beautiful pairs in nums.
//
// Two integers x and y are coprime if there is no integer greater than 1 that divides both of them.
// In other words, x and y are coprime if gcd(x, y) == 1, where gcd(x, y) is the greatest common divisor of x and y.
//
// Example 1:
//
// Input: nums = [2,5,1,4]
// Output: 5
// Explanation: There are 5 beautiful pairs in nums:
// When i = 0 and j = 1: the first digit of nums[0] is 2, and the last digit of nums[1] is 5. We can confirm that 2 and 5 are coprime, since gcd(2,5) == 1.
// When i = 0 and j = 2: the first digit of nums[0] is 2, and the last digit of nums[2] is 1. Indeed, gcd(2,1) == 1.
// When i = 1 and j = 2: the first digit of nums[1] is 5, and the last digit of nums[2] is 1. Indeed, gcd(5,1) == 1.
// When i = 1 and j = 3: the first digit of nums[1] is 5, and the last digit of nums[3] is 4. Indeed, gcd(5,4) == 1.
// When i = 2 and j = 3: the first digit of nums[2] is 1, and the last digit of nums[3] is 4. Indeed, gcd(1,4) == 1.
// Thus, we return 5.
//
// Example 2:
//
// Input: nums = [11,21,12]
// Output: 2
// Explanation: There are 2 beautiful pairs:
// When i = 0 and j = 1: the first digit of nums[0] is 1, and the last digit of nums[1] is 1. Indeed, gcd(1,1) == 1.
// When i = 0 and j = 2: the first digit of nums[0] is 1, and the last digit of nums[2] is 2. Indeed, gcd(1,2) == 1.
// Thus, we return 2.
//
// Constraints:
//
//     2 <= nums.length <= 100
//     1 <= nums[i] <= 9999
//     nums[i] % 10 != 0
//

struct Solution;

impl Solution {
    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut count = 0;
        let gcd = |a: i32, b: i32| {
            let mut result = a.min(b);
            while result > 0 {
                if a % result == 0 && b % result == 0 {
                    break;
                }
                result -= 1
            }
            result
        };
        for i in 0..len - 1 {
            let first_char = format!("{}", nums[i]).chars().next().unwrap();
            let first_digit = first_char.to_digit(10).unwrap() as i32;
            for num in nums.iter().take(len).skip(i + 1) {
                let last_digit = num % 10;
                if gcd(first_digit, last_digit) == 1 {
                    count += 1;
                }
            }
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_beautiful_pairs(vec![2, 5, 1, 4]), 5);
    assert_eq!(Solution::count_beautiful_pairs(vec![11, 21, 12]), 2);
}
