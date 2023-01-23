#![allow(dead_code)]

// 1291. Sequential Digits
// https://leetcode.com/problems/sequential-digits/
// https://leetcode.cn/problems/sequential-digits/
//
// Medium
//
// An integer has sequential digits if and only if each digit in the number is one more than the previous digit.
//
// Return a sorted list of all the integers in the range [low, high] inclusive that have sequential digits.
//
// Example 1:
//
// Input: low = 100, high = 300
// Output: [123,234]
//
// Example 2:
//
// Input: low = 1000, high = 13000
// Output: [1234,2345,3456,4567,5678,6789,12345]
//
// Constraints:
//
// -    10 <= low <= high <= 10^9
//

struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        use std::collections::VecDeque;
        let mut vd: VecDeque<i32> = (1..=9).collect::<VecDeque<i32>>();
        let mut answer: Vec<i32> = Vec::new();
        while let Some(front) = vd.pop_front() {
            if low <= front && front <= high {
                answer.push(front);
            }
            let d = front % 10;
            if d < 9 {
                vd.push_back(front * 10 + d + 1);
            }
        }
        answer
    }
}

#[test]
fn test() {
    let cases = vec![
        (100, 300, vec![123, 234]),
        (1000, 13000, vec![1234, 2345, 3456, 4567, 5678, 6789, 12345]),
    ];
    for (low, high, expect) in cases {
        assert_eq!(Solution::sequential_digits(low, high), expect);
    }
}
