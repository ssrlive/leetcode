#![allow(dead_code)]

// 3483. Unique 3-Digit Even Numbers
// https://leetcode.com/problems/unique-3-digit-even-numbers/
// https://leetcode.cn/problems/unique-3-digit-even-numbers/
//
// Easy
//
// You are given an array of digits called digits. Your task is to determine the number of distinct three-digit even numbers that can be formed using these digits.
//
// Note: Each copy of a digit can only be used once per number, and there may not be leading zeros.
//
// Example 1:
//
// Input: digits = [1,2,3,4]
//
// Output: 12
//
// Explanation: The 12 distinct 3-digit even numbers that can be formed are 124, 132, 134, 142, 214, 234, 312, 314, 324, 342, 412, and 432.
// Note that 222 cannot be formed because there is only 1 copy of the digit 2.
//
// Example 2:
//
// Input: digits = [0,2,2]
//
// Output: 2
//
// Explanation: The only 3-digit even numbers that can be formed are 202 and 220. Note that the digit 2 can be used twice because it appears twice in the array.
//
// Example 3:
//
// Input: digits = [6,6,6]
//
// Output: 1
//
// Explanation: Only 666 can be formed.
//
// Example 4:
//
// Input: digits = [1,3,5]
//
// Output: 0
//
// Explanation: No even 3-digit numbers can be formed.
//
// Constraints:
//
//     3 <= digits.length <= 10
//     0 <= digits[i] <= 9
//

struct Solution;

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut freq = [0; 10];
        for d in digits.iter() {
            freq[*d as usize] += 1;
        }

        let mut count = 0;
        for num in (100..1000).step_by(2) {
            let a = num / 100;
            let b = (num / 10) % 10;
            let c = num % 10;

            let mut needed = [0; 10];
            needed[a] += 1;
            needed[b] += 1;
            needed[c] += 1;

            let mut can_form = true;
            for d in 0..10 {
                if needed[d] > freq[d] {
                    can_form = false;
                    break;
                }
            }
            if can_form {
                count += 1;
            }
        }
        count
    }
}

#[test]
fn test() {
    let digits = vec![1, 2, 3, 4];
    let output = 12;
    assert_eq!(Solution::total_numbers(digits), output);

    let digits = vec![0, 2, 2];
    let output = 2;
    assert_eq!(Solution::total_numbers(digits), output);

    let digits = vec![6, 6, 6];
    let output = 1;
    assert_eq!(Solution::total_numbers(digits), output);

    let digits = vec![1, 3, 5];
    let output = 0;
    assert_eq!(Solution::total_numbers(digits), output);
}
