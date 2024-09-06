#![allow(dead_code)]

// 3272. Find the Count of Good Integers
// https://leetcode.com/problems/find-the-count-of-good-integers/
// https://leetcode.cn/problems/find-the-count-of-good-integers/
//
// Hard
//
// You are given two positive integers n and k.
//
// An integer x is called k-palindromic if:
//
//     x is a palindrome.
//     x is divisible by k.
//
// An integer is called good if its digits can be rearranged to form a k-palindromic integer. For example, for k = 2, 2020 can
// be rearranged to form the k-palindromic integer 2002, whereas 1010 cannot be rearranged to form a k-palindromic integer.
//
// Return the count of good integers containing n digits.
//
// Note that any integer must not have leading zeros, neither before nor after rearrangement.
// For example, 1010 cannot be rearranged to form 101.
//
// Example 1:
//
// Input: n = 3, k = 5
//
// Output: 27
//
// Explanation:
//
// Some of the good integers are:
//
//     551 because it can be rearranged to form 515.
//     525 because it is already k-palindromic.
//
// Example 2:
//
// Input: n = 1, k = 4
//
// Output: 2
//
// Explanation:
//
// The two good integers are 4 and 8.
//
// Example 3:
//
// Input: n = 5, k = 6
//
// Output: 2468
//
// Constraints:
//
//     1 <= n <= 10
//     1 <= k <= 9
//

struct Solution;

impl Solution {
    const F: [i64; 11] = [1, 1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800];
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        let k = k as i64;
        let n = n as usize;
        let s = 10i64.pow(((n + 1) / 2) as u32 - 1);
        let d = n / 2;
        let b = 10i64.pow(d as u32);
        let r = if n % 2 == 0 { 1 } else { 10 };

        let mut seen = vec![false; s as usize * 10];
        let mut res = 0;
        for i in s..s * 10 {
            if k % 2 == 0 && (i / s) % 2 != 0 {
                continue;
            }
            let mut j = i / r;
            let mut digits = Vec::new();
            while j > 0 {
                digits.push(j % 10);
                j /= 10;
            }
            digits.sort_unstable();
            let mut base = 1;
            let mut encoded = 0;
            for di in digits {
                encoded += di * base;
                base *= 10;
            }
            if n % 2 == 1 {
                encoded += (i % 10) * base;
            }
            if seen[encoded as usize] {
                continue;
            }

            j = i * b;
            let mut ji = i;
            if n % 2 == 1 {
                ji /= 10
            }
            let mut ti = b / 10;
            while ti > 0 {
                j += (ji % 10) * ti;
                ji /= 10;
                ti /= 10;
            }
            if j % k == 0 {
                seen[encoded as usize] = true;
                let mut cd = vec![0; 10];
                while j > 0 {
                    cd[(j % 10) as usize] += 1;
                    j /= 10;
                }
                let mut add = Self::F[n];
                for &i in &cd {
                    add /= Self::F[i];
                }
                if cd[0] != 0 {
                    cd[0] -= 1;
                    let mut sub = Self::F[n - 1];
                    for &i in &cd {
                        sub /= Self::F[i];
                    }
                    add -= sub;
                }
                res += add;
            }
        }
        res
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_good_integers(3, 5), 27);
    assert_eq!(Solution::count_good_integers(1, 4), 2);
    assert_eq!(Solution::count_good_integers(5, 6), 2468);
}
