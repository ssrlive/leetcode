#![allow(dead_code)]

// 842. Split Array into Fibonacci Sequence
// https://leetcode.com/problems/split-array-into-fibonacci-sequence/
// https://leetcode.cn/problems/split-array-into-fibonacci-sequence/
//
// You are given a string of digits num, such as "123456579". We can split it into a Fibonacci-like sequence [123, 456, 579].
//
// Formally, a Fibonacci-like sequence is a list f of non-negative integers such that:
//
// 0 <= f[i] < 231, (that is, each integer fits in a 32-bit signed integer type),
// f.length >= 3, and
// f[i] + f[i + 1] == f[i + 2] for all 0 <= i < f.length - 2.
// Note that when splitting the string into pieces, each piece must not have extra leading zeroes, except if the piece is the number 0 itself.
//
// Return any Fibonacci-like sequence split from num, or return [] if it cannot be done.
//
// Example 1:
//
// Input: num = "1101111"
// Output: [11,0,11,11]
// Explanation: The output [110, 1, 111] would also be accepted.
//
// Example 2:
//
// Input: num = "112358130"
// Output: []
// Explanation: The task is impossible.
//
// Example 3:
//
// Input: num = "0123"
// Output: []
// Explanation: Leading zeroes are not allowed, so "01", "2", "3" is not valid.
//
// Constraints:
//
// - 1 <= num.length <= 200
// - num contains only digits.
//

struct Solution;

impl Solution {
    pub fn split_into_fibonacci(num: String) -> Vec<i32> {
        fn fun(ans: &mut Vec<i32>, num: &str, ind: usize) -> Option<bool> {
            if ind == num.len() {
                return Some(ans.len() > 2);
            }
            let mut val = 0_i64;
            for i in ind..num.len() {
                val = val * 10 + (num.chars().nth(i)? as u8 - b'0') as i64;
                if val > i32::MAX as i64 {
                    return Some(false);
                }
                let mut y = 0;
                if ans.len() >= 2 {
                    y = ans[ans.len() - 2];
                    y += ans[ans.len() - 1];
                }
                if ans.len() < 2 || y == val as i32 {
                    ans.push(val as i32);
                    if fun(ans, num, i + 1)? {
                        return Some(true);
                    }
                    ans.pop();
                }
                if i == ind && num.chars().nth(i)? == '0' {
                    return Some(false);
                }
            }
            Some(false)
        }

        let mut ans = vec![];
        let _ = fun(&mut ans, &num, 0).unwrap_or_default();
        ans
    }
}

#[test]
fn test() {
    let cases = vec![("1101111", vec![11, 0, 11, 11]), ("112358130", vec![]), ("0123", vec![])];
    for (num, want) in cases {
        assert_eq!(Solution::split_into_fibonacci(num.to_string()), want);
    }
}
