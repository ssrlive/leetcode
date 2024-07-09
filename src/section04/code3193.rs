#![allow(dead_code)]

// 3193. Count the Number of Inversions
// https://leetcode.com/problems/count-the-number-of-inversions/
// https://leetcode.cn/problems/count-the-number-of-inversions/
//
// Hard
//
// You are given an integer n and a 2D array requirements, where requirements[i] = [endi, cnti]
// represents the end index and the inversion count of each requirement.
//
// A pair of indices (i, j) from an integer array nums is called an inversion if:
//
//     i < j and nums[i] > nums[j]
//
// Return the number of permutations perm of [0, 1, 2, ..., n - 1] such that for all requirements[i],
// perm[0..endi] has exactly cnti inversions.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: n = 3, requirements = [[2,2],[0,0]]
//
// Output: 2
//
// Explanation:
//
// The two permutations are:
//
//     [2, 0, 1]
//         Prefix [2, 0, 1] has inversions (0, 1) and (0, 2).
//         Prefix [2] has 0 inversions.
//     [1, 2, 0]
//         Prefix [1, 2, 0] has inversions (0, 2) and (1, 2).
//         Prefix [1] has 0 inversions.
//
// Example 2:
//
// Input: n = 3, requirements = [[2,2],[1,1],[0,0]]
//
// Output: 1
//
// Explanation:
//
// The only satisfying permutation is [2, 0, 1]:
//
//     Prefix [2, 0, 1] has inversions (0, 1) and (0, 2).
//     Prefix [2, 0] has an inversion (0, 1).
//     Prefix [2] has 0 inversions.
//
// Example 3:
//
// Input: n = 2, requirements = [[0,0],[1,0]]
//
// Output: 1
//
// Explanation:
//
// The only satisfying permutation is [0, 1]:
//
//     Prefix [0] has 0 inversions.
//     Prefix [0, 1] has an inversion (0, 1).
//
// Constraints:
//
//     2 <= n <= 300
//     1 <= requirements.length <= n
//     requirements[i] = [endi, cnti]
//     0 <= endi <= n - 1
//     0 <= cnti <= 400
//     The input is generated such that there is at least one i such that endi == n - 1.
//     The input is generated such that all endi are unique.
//

struct Solution;

impl Solution {
    pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
        const MOD: i32 = 1000000007;
        let n = n as usize;
        let mut req_map = vec![-1; n];
        for x in &requirements {
            if x[1] > x[0] * (x[0] + 1) / 2 {
                return 0;
            }
            req_map[x[0] as usize] = x[1];
        }
        let m = req_map[n - 1] as usize;
        let mut dp = vec![vec![0; m + 1]; n];
        dp[0][0] = 1;
        for i in 1..n {
            let mut sum = 0;
            let tm = m.min(i * (i + 1) / 2);
            for j in 0..=tm {
                sum += dp[i - 1][j];
                sum %= MOD;
                if j > i {
                    sum -= dp[i - 1][j - i - 1]
                };
                while sum < 0 {
                    sum += MOD;
                }
                sum %= MOD;
                if req_map[i] < 0 || req_map[i] as usize == j {
                    dp[i][j] = sum;
                }
            }
        }
        dp[n - 1][req_map[n - 1] as usize]
    }
}

#[test]
fn test() {
    let n = 3;
    let requirements = vec![vec![2, 2], vec![0, 0]];
    let output = 2;
    assert_eq!(Solution::number_of_permutations(n, requirements), output);
    let n = 3;
    let requirements = vec![vec![2, 2], vec![1, 1], vec![0, 0]];
    let output = 1;
    assert_eq!(Solution::number_of_permutations(n, requirements), output);
    let n = 2;
    let requirements = vec![vec![0, 0], vec![1, 0]];
    let output = 1;
    assert_eq!(Solution::number_of_permutations(n, requirements), output);
}
