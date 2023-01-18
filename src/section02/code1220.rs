#![allow(dead_code)]

// 1220. Count Vowels Permutation
// https://leetcode.com/problems/count-vowels-permutation/
// https://leetcode.cn/problems/count-vowels-permutation/
//
// Hard
//
// Given an integer n, your task is to count how many strings of length n can be formed under the following rules:
//
//     Each character is a lower case vowel ('a', 'e', 'i', 'o', 'u')
//     Each vowel 'a' may only be followed by an 'e'.
//     Each vowel 'e' may only be followed by an 'a' or an 'i'.
//     Each vowel 'i' may not be followed by another 'i'.
//     Each vowel 'o' may only be followed by an 'i' or a 'u'.
//     Each vowel 'u' may only be followed by an 'a'.
//
// Since the answer may be too large, return it modulo 10^9 + 7.
//
// Example 1:
//
// Input: n = 1
// Output: 5
// Explanation: All possible strings are: "a", "e", "i" , "o" and "u".
//
// Example 2:
//
// Input: n = 2
// Output: 10
// Explanation: All possible strings are: "ae", "ea", "ei", "ia", "ie", "io", "iu", "oi", "ou" and "ua".
//
// Example 3:
//
// Input: n = 5
// Output: 68
//
// Constraints:
//
// -    1 <= n <= 2 * 10^4
//

struct Solution;

const MOD: u32 = 1_000_000_007;

impl Solution {
    pub fn count_vowel_permutation(n: i32) -> i32 {
        ((1..n)
            .fold([1; 5], |v, _| {
                [
                    (v[1] + v[2] + v[4]) % MOD,
                    (v[0] + v[2]) % MOD,
                    (v[1] + v[3]) % MOD,
                    (v[2]) % MOD,
                    (v[2] + v[3]) % MOD,
                ]
            })
            .iter()
            .sum::<u32>()
            % MOD) as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_vowel_permutation(1), 5);
    assert_eq!(Solution::count_vowel_permutation(2), 10);
    assert_eq!(Solution::count_vowel_permutation(5), 68);
}
