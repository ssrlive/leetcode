#![allow(dead_code)]

// 3518. Smallest Palindromic Rearrangement II
// https://leetcode.com/problems/smallest-palindromic-rearrangement-ii/
// https://leetcode.cn/problems/smallest-palindromic-rearrangement-ii/
//
// Hard
//
// You are given a string s and an integer k.
//
// Return the k-th palindromic of s. If there are fewer than k distinct palindromic permutations, return an empty string.
//
// Note: Different rearrangements that yield the same palindromic string are considered identical and are counted once.
//
// Example 1:
//
// Input: s = "abba", k = 2
//
// Output: "baab"
//
// Explanation:
//
//     The two distinct palindromic rearrangements of "abba" are "abba" and "baab".
//     Lexicographically, "abba" comes before "baab". Since k = 2, the output is "baab".
//
// Example 2:
//
// Input: s = "aa", k = 2
//
// Output: ""
//
// Explanation:
//
//     There is only one palindromic rearrangement: "aa".
//     The output is an empty string since k = 2 exceeds the number of possible rearrangements.
//
// Example 3:
//
// Input: s = "bacab", k = 1
//
// Output: "abcba"
//
// Explanation:
//
//     The two distinct palindromic rearrangements of "bacab" are "abcba" and "bacab".
//     Lexicographically, "abcba" comes before "bacab". Since k = 1, the output is "abcba".
//
// Constraints:
//
//     1 <= s.length <= 10^4
//     s consists of lowercase English letters.
//     s is guaranteed to be palindromic.
//     1 <= k <= 10^6
//

struct Solution;

impl Solution {
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        fn binom_capped(n: i32, r: i32, k: i32) -> i32 {
            let mut r = r;
            if r > n - r {
                r = n - r;
            }
            let mut res = 1;
            for i in 1..=r {
                res = res * (n - i + 1) / i;
                if res >= k {
                    return k;
                }
            }
            res
        }
        fn find_ways(k: i32, freq: &mut [i32]) -> i32 {
            let mut total = 0;
            let mut ways = 1;
            for f in freq.iter() {
                if *f == 0 {
                    continue;
                }
                let add_ways = binom_capped(total + f, *f, k);
                let tmp = ways as i64 * add_ways as i64;
                ways = if tmp >= k as i64 { k } else { tmp as i32 };
                total += f;
            }
            ways
        }

        let n = s.len() as i32;
        let len = n / 2;
        let mut count = vec![0; 26];
        for i in 0..len {
            count[s.chars().nth(i as usize).unwrap() as usize - 'a' as usize] += 1;
        }
        let mut i = 0;
        let mut j = n - 1;
        let mut c = 0_u8;
        let mut s = s.chars().collect::<Vec<char>>();
        let mut k = k;
        while i < j && c < 26 {
            if count[c as usize] > 0 {
                count[c as usize] -= 1;
                let ways = find_ways(k, &mut count);
                if ways >= k {
                    s[i as usize] = (c + b'a') as char;
                    s[j as usize] = (c + b'a') as char;
                    i += 1;
                    j -= 1;
                    c = 0;
                } else {
                    k -= ways;
                    count[c as usize] += 1;
                    c += 1;
                }
            } else {
                c += 1;
            }
        }
        if c > 25 {
            return "".to_string();
        }
        s.iter().collect()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::smallest_palindrome("abba".to_string(), 2), "baab".to_string());
    assert_eq!(Solution::smallest_palindrome("aa".to_string(), 2), "".to_string());
    assert_eq!(Solution::smallest_palindrome("bacab".to_string(), 1), "abcba".to_string());
}
