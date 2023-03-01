#![allow(dead_code)]

/*

// 1957. Delete Characters to Make Fancy String
// https://leetcode.com/problems/delete-characters-to-make-fancy-string/
// https://leetcode.cn/problems/delete-characters-to-make-fancy-string/
//
// Easy
//
// A fancy string is a string where no three consecutive characters are equal.

Given a string s, delete the minimum possible number of characters from s to make it fancy.

Return the final string after the deletion. It can be shown that the answer will always be unique.

Example 1:

Input: s = "leeetcode"
Output: "leetcode"
Explanation:
Remove an 'e' from the first group of 'e's to create "leetcode".
No three consecutive characters are equal, so return "leetcode".

Example 2:

Input: s = "aaabaaaa"
Output: "aabaa"
Explanation:
Remove an 'a' from the first group of 'a's to create "aabaaaa".
Remove two 'a's from the second group of 'a's to create "aabaa".
No three consecutive characters are equal, so return "aabaa".

Example 3:

Input: s = "aab"
Output: "aab"
Explanation: No three consecutive characters are equal, so return "aab".

Constraints:

    1 <= s.length <= 10^5
    s consists only of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        let (mut count, mut prev) = (0, ' ');
        s.chars().for_each(|c| {
            match c == prev {
                true => count += 1,
                false => {
                    count = 1;
                    prev = c;
                }
            };
            if count < 3 {
                res.push(c);
            }
        });
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        ("leeetcode", "leetcode"),
        ("aaabaaaa", "aabaa"),
        ("aab", "aab"),
        ("aaaaa", "aa"),
    ];
    for (s, expect) in cases {
        assert_eq!(Solution::make_fancy_string(s.to_string()), expect);
    }
}
