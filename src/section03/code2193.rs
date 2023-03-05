#![allow(dead_code)]

/*

// 2193. Minimum Number of Moves to Make Palindrome
// https://leetcode.com/problems/minimum-number-of-moves-to-make-palindrome/
// https://leetcode.cn/problems/minimum-number-of-moves-to-make-palindrome/
//
// Hard
//
// You are given a string s consisting only of lowercase English letters.

In one move, you can select any two adjacent characters of s and swap them.

Return the minimum number of moves needed to make s a palindrome.

Note that the input will be generated such that s can always be converted to a palindrome.

Example 1:

Input: s = "aabb"
Output: 2
Explanation:
We can obtain two palindromes from s, "abba" and "baab".
- We can obtain "abba" from s in 2 moves: "aabb" -> "abab" -> "abba".
- We can obtain "baab" from s in 2 moves: "aabb" -> "abab" -> "baab".
Thus, the minimum number of moves needed to make s a palindrome is 2.

Example 2:

Input: s = "letelt"
Output: 2
Explanation:
One of the palindromes we can obtain from s in 2 moves is "lettel".
One of the ways we can obtain it is "letelt" -> "letetl" -> "lettel".
Other palindromes such as "tleelt" can also be obtained in 2 moves.
It can be shown that it is not possible to obtain a palindrome in less than 2 moves.

Constraints:

    1 <= s.length <= 2000
    s consists only of lowercase English letters.
    s can be converted to a palindrome using a finite number of moves.
*/

struct Solution;

impl Solution {
    pub fn min_moves_to_make_palindrome(s: String) -> i32 {
        let mut s = s;
        let mut ans = 0;
        while s.len() > 1 {
            let i = s.rfind(s.chars().next().unwrap()).unwrap();
            if i != 0 {
                ans += (s.len() - 1 - i) as i32;
                s = format!("{}{}", &s[1..i], &s[i + 1..]);
            } else {
                let i = s.find(s.chars().last().unwrap()).unwrap();
                ans += i as i32;
                s = format!("{}{}", &s[..i], &s[i + 1..(s.len() - 1)]);
            }
        }
        ans
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("aabb", 2),
        ("letelt", 2),
    ];
    for (s, expect) in cases {
        assert_eq!(Solution::min_moves_to_make_palindrome(s.to_string()), expect);
    }
}
