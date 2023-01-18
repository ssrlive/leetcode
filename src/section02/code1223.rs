#![allow(dead_code)]

// 1223. Dice Roll Simulation
// https://leetcode.com/problems/dice-roll-simulation/
// https://leetcode.cn/problems/dice-roll-simulation/
//
// Hard
//
// A die simulator generates a random number from 1 to 6 for each roll. You introduced a constraint to the generator such that it cannot roll the number i more than rollMax[i] (1-indexed) consecutive times.
//
// Given an array of integers rollMax and an integer n, return the number of distinct sequences that can be obtained with exact n rolls. Since the answer may be too large, return it modulo 109 + 7.
//
// Two sequences are considered different if at least one element differs from each other.
//
// Example 1:
//
// Input: n = 2, rollMax = [1,1,2,2,2,3]
// Output: 34
// Explanation: There will be 2 rolls of die, if there are no constraints on the die, there are 6 * 6 = 36 possible combinations. In this case, looking at rollMax array, the numbers 1 and 2 appear at most once consecutively, therefore sequences (1,1) and (2,2) cannot occur, so the final answer is 36-2 = 34.
//
// Example 2:
//
// Input: n = 2, rollMax = [1,1,1,1,1,1]
// Output: 30
//
// Example 3:
//
// Input: n = 3, rollMax = [1,1,1,2,2,3]
// Output: 181
//
// Constraints:
//
// -    1 <= n <= 5000
// -    rollMax.length == 6
// -    1 <= rollMax[i] <= 15
//

struct Solution;

impl Solution {
    pub fn die_simulator(n: i32, roll_max: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut dp = vec![[1i64; 6]; n];

        for i in 1..n {
            for j in 0..6 {
                dp[i][j] = 0;
                for k in 0..6 {
                    dp[i][j] += dp[i - 1][k];
                }

                if roll_max[j] as usize <= i {
                    if roll_max[j] as usize == i {
                        dp[i][j] -= 1;
                    } else {
                        for k in 0..6 {
                            if k != j {
                                dp[i][j] -= dp[i - 1 - roll_max[j] as usize][k];
                            }
                        }
                    }
                }
                dp[i][j] %= 1_000_000_007;
                if dp[i][j] < 0 {
                    dp[i][j] += 1_000_000_007;
                }
            }
        }

        (dp[n - 1].iter().sum::<i64>() % 1_000_000_007) as _
    }
}

#[test]
fn test() {
    let cases = vec![
        (2, vec![1, 1, 2, 2, 2, 3], 34),
        (2, vec![1, 1, 1, 1, 1, 1], 30),
        (3, vec![1, 1, 1, 2, 2, 3], 181),
    ];
    for (n, roll_max, expected) in cases {
        assert_eq!(Solution::die_simulator(n, roll_max), expected);
    }
}
