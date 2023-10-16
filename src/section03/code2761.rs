#![allow(dead_code)]

// 2761. Prime Pairs With Target Sum
// https://leetcode.com/problems/prime-pairs-with-target-sum/
// https://leetcode.cn/problems/prime-pairs-with-target-sum/
//
// Medium
//
// You are given an integer n. We say that two integers x and y form a prime number pair if:
//
//    1 <= x <= y <= n
//     x + y == n
//     x and y are prime numbers
//
// Return the 2D sorted list of prime number pairs [xi, yi]. The list should be sorted in increasing order of xi.
// If there are no prime number pairs at all, return an empty array.
//
// Note: A prime number is a natural number greater than 1 with only two factors, itself and 1.
//
// Example 1:
//
// Input: n = 10
// Output: [[3,7],[5,5]]
// Explanation: In this example, there are two prime pairs that satisfy the criteria.
// These pairs are [3,7] and [5,5], and we return them in the sorted order as described in the problem statement.
//
// Example 2:
//
// Input: n = 2
// Output: []
// Explanation: We can show that there is no prime number pair that gives a sum of 2, so we return an empty array.
//
// Constraints:
//
//     1 <= n <= 10^6
//

struct Solution;

impl Solution {
    pub fn find_prime_pairs(n: i32) -> Vec<Vec<i32>> {
        let mut p = vec![1; n as usize + 1];

        for i in 2..n as usize {
            if i * i > n as usize {
                break;
            }
            if p[i] == 0 {
                continue;
            }
            for k in 2..n as usize / i + 1 {
                if k * i <= n as usize {
                    p[k * i] = 0;
                }
            }
        }

        let mut ret = vec![];
        for i in 2..=n / 2 {
            if p[i as usize] == 0 || p[n as usize - i as usize] == 0 {
                continue;
            }
            ret.push(vec![i, n - i]);
        }

        ret
    }
}

#[test]
fn test() {
    let n = 10;
    let res = vec![vec![3, 7], vec![5, 5]];
    assert_eq!(Solution::find_prime_pairs(n), res);

    let n = 2;
    let res: Vec<Vec<i32>> = vec![];
    assert_eq!(Solution::find_prime_pairs(n), res);
}
