#![allow(dead_code)]

// 3405. Count the Number of Arrays with K Matching Adjacent Elements
// https://leetcode.com/problems/count-the-number-of-arrays-with-k-matching-adjacent-elements/
// https://leetcode.cn/problems/count-the-number-of-arrays-with-k-matching-adjacent-elements/
//
// Hard
//
// You are given three integers n, m, k. A good array arr of size n is defined as follows:
//
// Each element in arr is in the inclusive range [1, m].
// Exactly k indices i (where 1 <= i < n) satisfy the condition arr[i - 1] == arr[i].
// Return the number of good arrays that can be formed.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: n = 3, m = 2, k = 1
//
// Output: 4
//
// Explanation:
//
// There are 4 good arrays. They are [1, 1, 2], [1, 2, 2], [2, 1, 1] and [2, 2, 1].
// Hence, the answer is 4.
//
// Example 2:
//
// Input: n = 4, m = 2, k = 2
//
// Output: 6
//
// Explanation:
//
// The good arrays are [1, 1, 1, 2], [1, 1, 2, 2], [1, 2, 2, 2], [2, 1, 1, 1], [2, 2, 1, 1] and [2, 2, 2, 1].
// Hence, the answer is 6.
//
// Example 3:
//
// Input: n = 5, m = 2, k = 0
//
// Output: 2
//
// Explanation:
//
// The good arrays are [1, 2, 1, 2, 1] and [2, 1, 2, 1, 2]. Hence, the answer is 2.
//
// Constraints:
//
// 1 <= n <= 10^5
// 1 <= m <= 10^5
// 0 <= k <= n - 1
//

struct Solution;

impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        const MOD: i64 = 1000000007;
        if k >= n {
            return 0;
        }

        fn power(mut x: i64, mut y: i32) -> i64 {
            let mut result = 1;
            x %= MOD;
            while y > 0 {
                if y & 1 == 1 {
                    result = (result * x) % MOD;
                }
                x = (x * x) % MOD;
                y >>= 1;
            }
            result
        }

        fn mod_inverse(a: i64) -> i64 {
            power(a, MOD as i32 - 2)
        }

        fn n_cr(n: i32, r: i32) -> i64 {
            if r > n {
                return 0;
            }
            if r == 0 {
                return 1;
            }

            let mut num = 1;
            let mut den = 1;
            for i in 0..r {
                num = (num * (n - i) as i64) % MOD;
                den = (den * (i + 1) as i64) % MOD;
            }
            (num * mod_inverse(den) % MOD) % MOD
        }

        let kpos = n_cr(n - 1, k);
        let rem = power(m as i64 - 1, n - k - 1);

        let result = (kpos * m as i64 % MOD * rem % MOD) % MOD;

        result as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_good_arrays(3, 2, 1), 4);
    assert_eq!(Solution::count_good_arrays(4, 2, 2), 6);
    assert_eq!(Solution::count_good_arrays(5, 2, 0), 2);
}
