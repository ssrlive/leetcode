#![allow(dead_code)]

// 2457. Minimum Addition to Make Integer Beautiful
// https://leetcode.com/problems/minimum-addition-to-make-integer-beautiful/
// https://leetcode.cn/problems/minimum-addition-to-make-integer-beautiful/
//
// You are given two positive integers n and target.
//
// An integer is considered beautiful if the sum of its digits is less than or equal to target.
//
// Return the minimum non-negative integer x such that n + x is beautiful. The input will be generated such that it is always possible to make n beautiful.
//
// Example 1:
//
// Input: n = 16, target = 6
// Output: 4
// Explanation: Initially n is 16 and its digit sum is 1 + 6 = 7. After adding 4, n becomes 20 and digit sum becomes 2 + 0 = 2. It can be shown that we can not make n beautiful with adding non-negative integer less than 4.
//
// Example 2:
//
// Input: n = 467, target = 6
// Output: 33
// Explanation: Initially n is 467 and its digit sum is 4 + 6 + 7 = 17. After adding 33, n becomes 500 and digit sum becomes 5 + 0 + 0 = 5. It can be shown that we can not make n beautiful with adding non-negative integer less than 33.
//
// Example 3:
//
// Input: n = 1, target = 1
// Output: 0
// Explanation: Initially n is 1 and its digit sum is 1, which is already smaller than or equal to target.
//
// Constraints:
//
// - 1 <= n <= 10^12
// - 1 <= target <= 150
// - The input will be generated such that it is always possible to make n beautiful.
//

struct Solution;

impl Solution {
    pub fn make_integer_beautiful(n: i64, target: i32) -> i64 {
        fn add_digits(mut n: i64) -> i64 {
            let mut sum: i64 = 0;
            while n > 0 {
                let d = n % 10;
                sum += d;
                n /= 10;
            }
            sum
        }

        let mut m: i64 = n;
        let mut d: i64 = 0;
        let mut c: i64 = 1;
        while add_digits(n + d) > target as i64 {
            let p: i64 = 10 - (m % 10);
            m = (m + p) / 10;
            d += p * c;
            c *= 10;
        }
        d
    }
}

#[test]
fn test() {
    assert_eq!(Solution::make_integer_beautiful(16, 6), 4);
    assert_eq!(Solution::make_integer_beautiful(467, 6), 33);
    assert_eq!(Solution::make_integer_beautiful(1, 1), 0);
}
