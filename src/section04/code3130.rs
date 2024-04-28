#![allow(dead_code)]

// 3130. Find All Possible Stable Binary Arrays II
// https://leetcode.com/problems/find-all-possible-stable-binary-arrays-ii/
// https://leetcode.cn/problems/find-all-possible-stable-binary-arrays-ii/
//
// Hard
//
// You are given 3 positive integers zero, one, and limit.
//
// A binary array arr is called stable if:
//
// The number of occurrences of 0 in arr is exactly zero.
// The number of occurrences of 1 in arr is exactly one.
// Each subarray  of arr with a size greater than limit must contain both 0 and 1.
// Return the total number of stable binary arrays.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: zero = 1, one = 1, limit = 2
//
// Output: 2
//
// Explanation:
//
// The two possible stable binary arrays are [1,0] and [0,1].
//
// Example 2:
//
// Input: zero = 1, one = 2, limit = 1
//
// Output: 1
//
// Explanation:
//
// The only possible stable binary array is [1,0,1].
//
// Example 3:
//
// Input: zero = 3, one = 3, limit = 2
//
// Output: 14
//
// Explanation:
//
// All the possible stable binary arrays are
// [0,0,1,0,1,1], [0,0,1,1,0,1], [0,1,0,0,1,1], [0,1,0,1,0,1], [0,1,0,1,1,0], [0,1,1,0,0,1], [0,1,1,0,1,0],
// [1,0,0,1,0,1], [1,0,0,1,1,0], [1,0,1,0,0,1], [1,0,1,0,1,0], [1,0,1,1,0,0], [1,1,0,0,1,0], and [1,1,0,1,0,0].
//
// Constraints:
//
// 1 <= zero, one, limit <= 1000
//

struct Solution;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let mut dp = vec![vec![vec![0; 2]; (one + 1) as usize]; (zero + 1) as usize];
        let mod_num = 1_000_000_007;
        for i in 0..=zero {
            for j in 0..=one {
                dp[i as usize][j as usize][0] = 0;
                dp[i as usize][j as usize][1] = 0;
                if i == 0 || j == 0 {
                    if 0 < i && i <= limit {
                        dp[i as usize][j as usize][0] = 1;
                    }
                    if 0 < j && j <= limit {
                        dp[i as usize][j as usize][1] = 1;
                    }
                    continue;
                }
                if i > 0 {
                    dp[i as usize][j as usize][0] = (dp[i as usize - 1][j as usize][0] + dp[i as usize - 1][j as usize][1]) % mod_num;
                }
                if i - limit > 0 {
                    dp[i as usize][j as usize][0] =
                        (dp[i as usize][j as usize][0] + mod_num - dp[(i - 1 - limit) as usize][j as usize][1]) % mod_num;
                }
                if j > 0 {
                    dp[i as usize][j as usize][1] = (dp[i as usize][j as usize - 1][0] + dp[i as usize][j as usize - 1][1]) % mod_num;
                }
                if j - limit > 0 {
                    dp[i as usize][j as usize][1] =
                        (dp[i as usize][j as usize][1] + mod_num - dp[i as usize][(j - 1 - limit) as usize][0]) % mod_num;
                }
            }
        }
        dp[zero as usize][one as usize][0] = (dp[zero as usize][one as usize][0] + dp[zero as usize][one as usize][1]) % mod_num;
        dp[zero as usize][one as usize][0]
    }
}

#[test]
fn test() {
    let zero = 1;
    let one = 1;
    let limit = 2;
    let output = 2;
    assert_eq!(Solution::number_of_stable_arrays(zero, one, limit), output);

    let zero = 1;
    let one = 2;
    let limit = 1;
    let output = 1;
    assert_eq!(Solution::number_of_stable_arrays(zero, one, limit), output);

    let zero = 3;
    let one = 3;
    let limit = 2;
    let output = 14;
    assert_eq!(Solution::number_of_stable_arrays(zero, one, limit), output);
}
