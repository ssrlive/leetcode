#![allow(dead_code)]

// 2851. String Transformation
// https://leetcode.com/problems/string-transformation/
// https://leetcode.cn/problems/string-transformation/
//
// Hard
//
// You are given two strings s and t of equal length n. You can perform the following operation on the string s:
//
//     Remove a suffix of s of length l where 0 < l < n and append it at the start of s.
//     For example, let s = 'abcd' then in one operation you can remove the suffix 'cd' and append it in front of s making s = 'cdab'.
//
// You are also given an integer k. Return the number of ways in which s can be transformed into t in exactly k operations.
//
// Since the answer can be large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: s = "abcd", t = "cdab", k = 2
// Output: 2
// Explanation:
// First way:
// In first operation, choose suffix from index = 3, so resulting s = "dabc".
// In second operation, choose suffix from index = 3, so resulting s = "cdab".
//
// Second way:
// In first operation, choose suffix from index = 1, so resulting s = "bcda".
// In second operation, choose suffix from index = 1, so resulting s = "cdab".
//
// Example 2:
//
// Input: s = "ababab", t = "ababab", k = 1
// Output: 2
// Explanation:
// First way:
// Choose suffix from index = 2, so resulting s = "ababab".
//
// Second way:
// Choose suffix from index = 4, so resulting s = "ababab".
//
// Constraints:
//
//     2 <= s.length <= 5 * 10^5
//     1 <= k <= 10^15
//     s.length == t.length
//     s and t consist of only lowercase English alphabets.
//

struct Solution;

impl Solution {
    const M: i64 = 1_000_000_007;

    fn add(x: i64, y: i64) -> i64 {
        if (x + y) >= Solution::M { x + y - Solution::M } else { x + y }
    }

    fn mul(x: i64, y: i64) -> i64 {
        x * y % Solution::M
    }

    fn getz(s: &str) -> Vec<i64> {
        let n = s.len();
        let mut z = vec![0_i64; n];
        let mut left = 0;
        let mut right = 0;
        for i in 1..n {
            if i <= right && z[i - left] <= right as i64 - i as i64 {
                z[i] = z[i - left];
            } else {
                z[i] = std::cmp::max(0, right as i64 - i as i64 + 1);
                while i as i64 + z[i] < n as i64 && s.as_bytes()[i + z[i] as usize] == s.as_bytes()[z[i] as usize] {
                    z[i] += 1;
                }
            }
            if i as i64 + z[i] - 1 > right as i64 {
                left = i;
                right = i + z[i] as usize - 1;
            }
        }
        z
    }

    fn mul2(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
        let m = a.len();
        let n = a[0].len();
        let p = b[0].len();
        let mut r = vec![vec![0; p]; m];
        for i in 0..m {
            for (j, b_j) in b.iter().enumerate().take(n) {
                for (k, &b_j_k) in b_j.iter().enumerate().take(p) {
                    r[i][k] = Solution::add(r[i][k], Solution::mul(a[i][j], b_j_k));
                }
            }
        }
        r
    }

    fn pow(a: &[Vec<i64>], y: i64) -> Vec<Vec<i64>> {
        let n = a.len();
        let mut r = vec![vec![0; n]; n];
        for (i, r_i) in r.iter_mut().enumerate().take(n) {
            r_i[i] = 1;
        }
        let mut x = a.to_owned();
        let mut y = y;
        while y > 0 {
            if y & 1 == 1 {
                r = Solution::mul2(&r, &x);
            }
            x = Solution::mul2(&x, &x);
            y >>= 1;
        }
        r
    }

    pub fn number_of_ways(s: String, t: String, k: i64) -> i32 {
        let n = s.len() as i64;
        let dp = &Solution::pow(&[vec![0_i64, 1], vec![n - 1, n - 2]], k)[0];
        let mut s = s;
        s.push_str(&t);
        s.push_str(&t);
        let z = Solution::getz(&s);
        let m = n + n;
        let mut r = 0;
        for i in n..m {
            if z[i as usize] >= n {
                r = Solution::add(r, dp[if (i - n) == 0 { 0 } else { 1 }]);
            }
        }
        r as _
    }
}

#[test]
fn test() {
    let s = "abcd".to_string();
    let t = "cdab".to_string();
    let k = 2;
    let output = 2;
    assert_eq!(Solution::number_of_ways(s, t, k), output);

    let s = "ababab".to_string();
    let t = "ababab".to_string();
    let k = 1;
    let output = 2;
    assert_eq!(Solution::number_of_ways(s, t, k), output);
}
