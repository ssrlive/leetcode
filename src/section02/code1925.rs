#![allow(dead_code)]

/*

// 1925. Count Square Sum Triples
// https://leetcode.com/problems/count-square-sum-triples/
// https://leetcode.cn/problems/count-square-sum-triples/
//
// Easy
//
// A square triple (a,b,c) is a triple where a, b, and c are integers and a2 + b2 = c2.

Given an integer n, return the number of square triples such that 1 <= a, b, c <= n.

Example 1:

Input: n = 5
Output: 2
Explanation: The square triples are (3,4,5) and (4,3,5).

Example 2:

Input: n = 10
Output: 4
Explanation: The square triples are (3,4,5), (4,3,5), (6,8,10), and (8,6,10).

Constraints:

    1 <= n <= 250
*/

struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        use std::collections::HashSet;

        if n <= 2 {
            return 0;
        }

        let (l, max_sum) = (n as usize, n * n);
        let mut count = 0;

        let squares_vec = (1..=n).map(|x| x * x).collect::<Vec<_>>();
        let squares_set = squares_vec.iter().collect::<HashSet<_>>();

        for (i, &square_1) in squares_vec[..(l - 2)].iter().enumerate() {
            for &square_2 in &squares_vec[i + 1..=(l - 1)] {
                match square_1 + square_2 {
                    sum if sum <= max_sum => count += 2 * (squares_set.contains(&sum) as i32),
                    _ => break,
                };
            }
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_triples(5), 2);
    assert_eq!(Solution::count_triples(10), 4);
    assert_eq!(Solution::count_triples(250), 330);
}
