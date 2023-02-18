#![allow(dead_code)]

/*

// 1717. Maximum Score From Removing Substrings
Medium
529
29
Companies

You are given a string s and two integers x and y. You can perform two types of operations any number of times.

    Remove substring "ab" and gain x points.
        For example, when removing "ab" from "cabxbae" it becomes "cxbae".
    Remove substring "ba" and gain y points.
        For example, when removing "ba" from "cabxbae" it becomes "cabxe".

Return the maximum points you can gain after applying the above operations on s.

Example 1:

Input: s = "cdbcbbaaabab", x = 4, y = 5
Output: 19
Explanation:
- Remove the "ba" underlined in "cdbcbbaaabab". Now, s = "cdbcbbaaab" and 5 points are added to the score.
- Remove the "ab" underlined in "cdbcbbaaab". Now, s = "cdbcbbaa" and 4 points are added to the score.
- Remove the "ba" underlined in "cdbcbbaa". Now, s = "cdbcba" and 5 points are added to the score.
- Remove the "ba" underlined in "cdbcba". Now, s = "cdbc" and 5 points are added to the score.
Total score = 5 + 4 + 5 + 5 = 19.

Example 2:

Input: s = "aabbaaxybbaabb", x = 5, y = 4
Output: 20

Constraints:

    1 <= s.length <= 10^5
    1 <= x, y <= 10^4
    s consists of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        fn remove(s: &mut Vec<u8>, r: &[u8], x: i32) -> i32 {
            let mut i = 0;
            let mut ans = 0;
            for j in 0..s.len() {
                s[i] = s[j];
                i += 1;
                if i > 1 && s[i - 2] == r[0] && s[i - 1] == r[1] {
                    i -= 2;
                    ans += x;
                }
            }
            s.resize(i, 0);
            ans
        }

        let (mut x, mut y) = (x, y);
        let mut s = s.as_bytes().to_vec();
        let mut ans = 0;
        let mut a = "ab".as_bytes();
        let mut b = "ba".as_bytes();
        if x < y {
            std::mem::swap(&mut a, &mut b);
            std::mem::swap(&mut x, &mut y);
        }
        ans += remove(&mut s, a, x);
        ans += remove(&mut s, b, y);
        ans
    }
}

#[test]
fn test() {
    let cases = vec![("cdbcbbaaabab", 4, 5, 19), ("aabbaaxybbaabb", 5, 4, 20)];
    for (s, x, y, expected) in cases {
        assert_eq!(Solution::maximum_gain(s.to_string(), x, y), expected);
    }
}
