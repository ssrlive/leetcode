#![allow(dead_code)]

// 967. Numbers With Same Consecutive Differences
// https://leetcode.com/problems/numbers-with-same-consecutive-differences/
// https://leetcode.cn/problems/numbers-with-same-consecutive-differences/
//
// Given two integers n and k, return an array of all the integers of length n where the difference
// between every two consecutive digits is k. You may return the answer in any order.
//
// Note that the integers should not have leading zeros. Integers as 02 and 043 are not allowed.
//
// Example 1:
//
// Input: n = 3, k = 7
// Output: [181,292,707,818,929]
// Explanation: Note that 070 is not a valid number, because it has leading zeroes.
//
// Example 2:
//
// Input: n = 2, k = 1
// Output: [10,12,21,23,32,34,43,45,54,56,65,67,76,78,87,89,98]
//
// Constraints:
//
// - 2 <= n <= 9
// - 0 <= k <= 9
//

struct Solution;

impl Solution {
    pub fn nums_same_consec_diff(n: i32, k: i32) -> Vec<i32> {
        let mut answer: Vec<i32> = (1..=9).collect();
        for _ in 0..n - 1 {
            let mut v: Vec<i32> = Vec::new();
            for m in answer.iter() {
                for d in if k == 0 { vec![0] } else { vec![-k, k] }.iter() {
                    let j = (m % 10) + d;
                    if (0..10).contains(&j) {
                        v.push(m * 10 + j);
                    }
                }
            }
            answer = v;
        }
        if n == 1 {
            answer.push(0);
        }
        answer
    }
}

#[test]
fn test() {
    let cases = vec![
        (3, 7, vec![181, 292, 707, 818, 929]),
        (
            2,
            1,
            vec![10, 12, 21, 23, 32, 34, 43, 45, 54, 56, 65, 67, 76, 78, 87, 89, 98],
        ),
        (1, 0, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]),
        (2, 0, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]),
    ];
    for (n, k, expect) in cases {
        let mut result = Solution::nums_same_consec_diff(n, k);
        result.sort();
        assert_eq!(result, expect);
    }
}
