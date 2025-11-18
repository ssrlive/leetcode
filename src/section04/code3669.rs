#![allow(dead_code)]

// 3669. Balanced K-Factor Decomposition
// https://leetcode.com/problems/balanced-k-factor-decomposition/
// https://leetcode.cn/problems/balanced-k-factor-decomposition/
//
// Medium
//
// Given two integers n and k, split the number n into exactly k positive integers such that the product of these integers is equal to n.
//
// Return any one split in which the maximum difference between any two numbers is minimized. You may return the result in any order.
//
// Example 1:
//
// Input: n = 100, k = 2
//
// Output: [10,10]
//
// Explanation:
//
// The split [10, 10] yields 10 * 10 = 100 and a max-min difference of 0, which is minimal.
//
// Example 2:
//
// Input: n = 44, k = 3
//
// Output: [2,2,11]
//
// Explanation:
//
// Split [1, 1, 44] yields a difference of 43
// Split [1, 2, 22] yields a difference of 21
// Split [1, 4, 11] yields a difference of 10
// Split [2, 2, 11] yields a difference of 9
// Therefore, [2, 2, 11] is the optimal split with the smallest difference 9.
//
// Constraints:
//
// 4 <= n <= 10^5
// 2 <= k <= 5
// k is strictly less than the total number of positive divisors of n.
//

struct Solution;

impl Solution {
    pub fn min_difference(n: i32, k: i32) -> Vec<i32> {
        let mut best: Vec<i32> = Vec::new();
        let mut best_diff = i32::MAX;

        fn backtrack(n: i32, k: i32, start: i32, curr: &mut Vec<i32>, best: &mut Vec<i32>, best_diff: &mut i32) {
            if k == 1 {
                if n >= start {
                    curr.push(n);
                    check(curr, best, best_diff);
                    curr.pop();
                }
                return;
            }

            for i in start..=n {
                if n % i == 0 {
                    curr.push(i);
                    backtrack(n / i, k - 1, i, curr, best, best_diff);
                    curr.pop();
                }
            }
        }

        fn check(curr: &[i32], best: &mut Vec<i32>, best_diff: &mut i32) {
            let mn = *curr.iter().min().unwrap();
            let mx = *curr.iter().max().unwrap();
            if mx - mn < *best_diff {
                *best_diff = mx - mn;
                *best = curr.to_vec();
            }
        }

        let mut curr: Vec<i32> = Vec::new();
        backtrack(n, k, 1, &mut curr, &mut best, &mut best_diff);
        best
    }
}

#[test]
fn test() {
    let n = 100;
    let k = 2;
    assert_eq!(Solution::min_difference(n, k), vec![10, 10]);

    let n = 44;
    let k = 3;
    assert_eq!(Solution::min_difference(n, k), vec![2, 2, 11]);
}
