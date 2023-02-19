#![allow(dead_code)]

/*

// 1737. Change Minimum Characters to Satisfy One of Three Conditions
// https://leetcode.com/problems/change-minimum-characters-to-satisfy-one-of-three-conditions/
// https://leetcode.cn/problems/change-minimum-characters-to-satisfy-one-of-three-conditions/
//
// Medium
//
// You are given two strings a and b that consist of lowercase letters. In one operation, you can change any character in a or b to any lowercase letter.

Your goal is to satisfy one of the following three conditions:

    Every letter in a is strictly less than every letter in b in the alphabet.
    Every letter in b is strictly less than every letter in a in the alphabet.
    Both a and b consist of only one distinct letter.

Return the minimum number of operations needed to achieve your goal.

Example 1:

Input: a = "aba", b = "caa"
Output: 2
Explanation: Consider the best way to make each condition true:
1) Change b to "ccc" in 2 operations, then every letter in a is less than every letter in b.
2) Change a to "bbb" and b to "aaa" in 3 operations, then every letter in b is less than every letter in a.
3) Change a to "aaa" and b to "aaa" in 2 operations, then a and b consist of one distinct letter.
The best way was done in 2 operations (either condition 1 or condition 3).

Example 2:

Input: a = "dabadd", b = "cda"
Output: 3
Explanation: The best way is to make condition 1 true by changing b to "eee".

Constraints:

    1 <= a.length, b.length <= 10^5
    a and b consist only of lowercase letters.
*/

struct Solution;

impl Solution {
    pub fn min_characters(a: String, b: String) -> i32 {
        let m = a.len();
        let n = b.len();
        let mut c1 = vec![0; 26];
        let mut c2 = vec![0; 26];
        for c in a.chars() {
            c1[c as usize - 'a' as usize] += 1;
        }
        for c in b.chars() {
            c2[c as usize - 'a' as usize] += 1;
        }
        let mut res = m + n;
        for i in 0..26 {
            res = res.min(m + n - c1[i] - c2[i]);
            if i > 0 {
                c1[i] += c1[i - 1];
                c2[i] += c2[i - 1];
            }
            if i < 25 {
                res = res.min(m - c1[i] + c2[i]);
                res = res.min(n - c2[i] + c1[i]);
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        ("aba", "caa", 2),
        ("dabadd", "cda", 3),
        ("a", "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz", 2),
    ];
    for (a, b, expected) in cases {
        assert_eq!(Solution::min_characters(a.to_string(), b.to_string()), expected);
    }
}
