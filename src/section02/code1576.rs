#![allow(dead_code)]

/*

// 1576. Replace All ?'s to Avoid Consecutive Repeating Characters
Easy
477
156
Companies

Given a string s containing only lowercase English letters and the '?' character, convert all the '?' characters into lowercase letters such that the final string does not contain any consecutive repeating characters. You cannot modify the non '?' characters.

It is guaranteed that there are no consecutive repeating characters in the given string except for '?'.

Return the final string after all the conversions (possibly zero) have been made. If there is more than one solution, return any of them. It can be shown that an answer is always possible with the given constraints.

Example 1:

Input: s = "?zs"
Output: "azs"
Explanation: There are 25 solutions for this problem. From "azs" to "yzs", all are valid. Only "z" is an invalid modification as the string will consist of consecutive repeating characters in "zzs".

Example 2:

Input: s = "ubv?w"
Output: "ubvaw"
Explanation: There are 24 solutions for this problem. Only "v" and "w" are invalid modifications as the strings will consist of consecutive repeating characters in "ubvvw" and "ubvww".

Constraints:

    1 <= s.length <= 100
    s consist of lowercase English letters and '?'.
*/

struct Solution;

impl Solution {
    pub fn modify_string(s: String) -> String {
        let mut s = s.into_bytes();
        let n = s.len();
        for i in 0..n {
            if s[i] == b'?' {
                for c in b'a'..=b'z' {
                    if (i == 0 || s[i - 1] != c) && (i == n - 1 || s[i + 1] != c) {
                        s[i] = c;
                        break;
                    }
                }
            }
        }
        String::from_utf8(s).unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::modify_string("?zs".to_string()), "azs");
    assert_eq!(Solution::modify_string("ubv?w".to_string()), "ubvaw");
    assert_eq!(Solution::modify_string("j?qg??b".to_string()), "jaqgacb");
    assert_eq!(Solution::modify_string("??yw?ipkj?".to_string()), "abywaipkja");
}
