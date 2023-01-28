#![allow(dead_code)]

// 1404. Number of Steps to Reduce a Number in Binary Representation to One
// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/
// https://leetcode.cn/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/
//
// Medium
//
// Given the binary representation of an integer as a string s, return the number of steps
// to reduce it to 1 under the following rules:
//
//     If the current number is even, you have to divide it by 2.
//
//     If the current number is odd, you have to add 1 to it.
//
// It is guaranteed that you can always reach one for all test cases.
//
// Example 1:
//
// Input: s = "1101"
// Output: 6
// Explanation: "1101" corressponds to number 13 in their decimal representation.
// Step 1) 13 is odd, add 1 and obtain 14.
// Step 2) 14 is even, divide by 2 and obtain 7.
// Step 3) 7 is odd, add 1 and obtain 8.
// Step 4) 8 is even, divide by 2 and obtain 4.
// Step 5) 4 is even, divide by 2 and obtain 2.
// Step 6) 2 is even, divide by 2 and obtain 1.
//
// Example 2:
//
// Input: s = "10"
// Output: 1
// Explanation: "10" corressponds to number 2 in their decimal representation.
// Step 1) 2 is even, divide by 2 and obtain 1.
//
// Example 3:
//
// Input: s = "1"
// Output: 0
//
// Constraints:
//
// -    1 <= s.length <= 500
// -    s consists of characters '0' or '1'
// -    s[0] == '1'
//

struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        use std::collections::*;
        let mut result = 0;
        let f = |v: char| if v == '1' { 1 } else { 0 };
        let mut s = s.chars().into_iter().map(f).collect::<VecDeque<u8>>();
        while s.len() > 1 {
            let li = s.len() - 1;
            if s[li] == 1 {
                let mut stop = false;
                s[li] = 0;
                for i in (0..li).rev() {
                    if s[i] == 0 {
                        s[i] = 1;
                        stop = true;
                        break;
                    } else {
                        s[i] = 0;
                    }
                }

                if !stop {
                    s.push_front(1);
                }
            } else {
                s.pop_back();
            }
            result += 1;
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![("1101", 6), ("10", 1), ("1", 0)];
    for (s, expected) in cases {
        assert_eq!(Solution::num_steps(s.to_string()), expected);
    }
}
