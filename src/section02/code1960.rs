#![allow(dead_code)]

/*

// 1960. Maximum Product of the Length of Two Palindromic Substrings
// https://leetcode.com/problems/maximum-product-of-the-length-of-two-palindromic-substrings/
// https://leetcode.cn/problems/maximum-product-of-the-length-of-two-palindromic-substrings/
//
// Hard
//
// You are given a 0-indexed string s and are tasked with finding two non-intersecting palindromic substrings of odd length such that the product of their lengths is maximized.

More formally, you want to choose four integers i, j, k, l such that 0 <= i <= j < k <= l < s.length and both the substrings s[i...j] and s[k...l] are palindromes and have odd lengths. s[i...j] denotes a substring from index i to index j inclusive.

Return the maximum possible product of the lengths of the two non-intersecting palindromic substrings.

A palindrome is a string that is the same forward and backward. A substring is a contiguous sequence of characters in a string.

Example 1:

Input: s = "ababbb"
Output: 9
Explanation: Substrings "aba" and "bbb" are palindromes with odd length. product = 3 * 3 = 9.

Example 2:

Input: s = "zaaaxbbby"
Output: 9
Explanation: Substrings "aaa" and "bbb" are palindromes with odd length. product = 3 * 3 = 9.

Constraints:

    2 <= s.length <= 10^5
    s consists of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn max_product(s: String) -> i64 {
        use std::collections::VecDeque;

        fn manacher_odd(s: &str) -> Vec<i32> {
            let n = s.len();
            let s = String::from("$") + s + &String::from("^");
            let s = s.chars().collect::<Vec<char>>();
            let mut p = vec![0; n + 2];
            let (mut l, mut r) = (1, 1);
            for i in 1..=n {
                p[i] = i32::max(0, i32::min(r - i as i32, p[(l + (r - i as i32)) as usize]));
                while s[i - p[i] as usize] == s[i + p[i] as usize] {
                    p[i] += 1;
                }
                if i as i32 + p[i] > r {
                    l = i as i32 - p[i];
                    r = i as i32 + p[i];
                }
            }
            p.pop();
            p.remove(0);
            p
        }

        let data = manacher_odd(&s);

        let n = data.len();
        let mut q = VecDeque::<(i32, i32)>::new();
        let mut right = vec![1; n];

        for i in (0..n).rev() {
            while let Some(a) = q.front() {
                if a.0 - a.1 <= i as i32 {
                    break;
                }
                q.pop_front();
            }

            if let Some(a) = q.front() {
                right[i] += (a.0 - i as i32) * 2;
            }

            q.push_back((i as i32, data[i] - 1));
        }

        while !q.is_empty() {
            q.pop_front();
        }
        let (mut ret, mut mx_left) = (1, 1);
        for i in 0..n - 1 {
            let mut temp = 1;

            while let Some(a) = q.front() {
                if a.0 + a.1 >= i as i32 {
                    break;
                }
                q.pop_front();
            }

            if let Some(a) = q.front() {
                temp += (i as i32 - a.0) as i64 * 2;
            }

            q.push_back((i as i32, data[i] - 1));

            mx_left = mx_left.max(temp);
            ret = ret.max(mx_left * right[i + 1] as i64);
        }

        ret
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("ababbb", 9),
        ("zaaaxbbby", 9),
        ("a", 1),
        ("aa", 1),
        ("aaa", 1),
    ];
    for (s, expect) in cases {
        assert_eq!(Solution::max_product(s.to_string()), expect);
    }
}
