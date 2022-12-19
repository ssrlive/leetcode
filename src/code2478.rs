#![allow(dead_code)]

// 2478. Number of Beautiful Partitions
// https://leetcode.com/problems/number-of-beautiful-partitions/
// https://leetcode.cn/problems/number-of-beautiful-partitions/
//
// You are given a string s that consists of the digits '1' to '9' and two integers k and minLength.
//
// A partition of s is called beautiful if:
//
// - s is partitioned into k non-intersecting substrings.
// - Each substring has a length of at least minLength.
// - Each substring starts with a prime digit and ends with a non-prime digit.
//   Prime digits are '2', '3', '5', and '7', and the rest of the digits are non-prime.
//
// Return the number of beautiful partitions of s. Since the answer may be very large, return it modulo 109 + 7.
//
// A substring is a contiguous sequence of characters within a string.
//
// Example 1:
//
// Input: s = "23542185131", k = 3, minLength = 2
// Output: 3
// Explanation: There exists three ways to create a beautiful partition:
// "2354 | 218 | 5131"
// "2354 | 21851 | 31"
// "2354218 | 51 | 31"
//
// Example 2:
//
// Input: s = "23542185131", k = 3, minLength = 3
// Output: 1
// Explanation: There exists one way to create a beautiful partition: "2354 | 218 | 5131".
//
// Example 3:
//
// Input: s = "3312958", k = 3, minLength = 1
// Output: 1
// Explanation: There exists one way to create a beautiful partition: "331 | 29 | 58".
//
// Constraints:
//
// - 1 <= k, minLength <= s.length <= 1000
// - s consists of the digits '1' to '9'.
//

struct Solution;

impl Solution {
    pub fn beautiful_partitions(s: String, k: i32, min_length: i32) -> i32 {
        fn is_prime(b: u8) -> bool {
            matches!(b, b'2' | b'3' | b'5' | b'7')
        }

        const MOD: i32 = 1_000_000_007;
        let len = s.len();
        let mut dp = vec![vec![0; len + 1]; k as usize + 1];
        dp[0][0] = 1;
        let mut sums = vec![0; k as usize + 1];
        let mut cur = 0;
        for i in 0..len {
            if !is_prime(s.as_bytes()[i]) {
                for pos in cur..=i as i32 - min_length + 1 {
                    let pos = pos as usize;
                    if is_prime(s.as_bytes()[pos]) {
                        for j in 0..=k {
                            let j = j as usize;
                            sums[j] = (sums[j] + dp[j][pos]) % MOD;
                        }
                    }
                }
                cur = 0.max(i as i32 - min_length + 2);
                dp[0][i] = 1;
                for j in 1..=k as usize {
                    dp[j][i + 1] = sums[j - 1];
                }
            }
        }
        dp[k as usize][len]
    }
}

#[test]
fn test() {
    let s = "242538614532395749146912679859".to_string();
    let k = 1;
    let min_length = 6;
    let res = 1;
    assert_eq!(Solution::beautiful_partitions(s, k, min_length), res);

    let s = "23542185131".to_string();
    let k = 3;
    let min_length = 2;
    let res = 3;
    assert_eq!(Solution::beautiful_partitions(s, k, min_length), res);

    let s = "23542185131".to_string();
    let k = 3;
    let min_length = 3;
    let res = 1;
    assert_eq!(Solution::beautiful_partitions(s, k, min_length), res);

    let s = "3312958".to_string();
    let k = 3;
    let min_length = 1;
    let res = 1;
    assert_eq!(Solution::beautiful_partitions(s, k, min_length), res);
}
