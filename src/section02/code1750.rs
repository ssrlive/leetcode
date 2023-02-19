#![allow(dead_code)]

/*

// 1750. Minimum Length of String After Deleting Similar Ends
// https://leetcode.com/problems/minimum-length-of-string-after-deleting-similar-ends/
// https://leetcode.cn/problems/minimum-length-of-string-after-deleting-similar-ends/
//
// Medium
//
// Given a string s consisting only of characters 'a', 'b', and 'c'. You are asked to apply the following algorithm on the string any number of times:

    Pick a non-empty prefix from the string s where all the characters in the prefix are equal.
    Pick a non-empty suffix from the string s where all the characters in this suffix are equal.
    The prefix and the suffix should not intersect at any index.
    The characters from the prefix and suffix must be the same.
    Delete both the prefix and the suffix.

Return the minimum length of s after performing the above operation any number of times (possibly zero times).

Example 1:

Input: s = "ca"
Output: 2
Explanation: You can't remove any characters, so the string stays as is.

Example 2:

Input: s = "cabaabac"
Output: 0
Explanation: An optimal sequence of operations is:
- Take prefix = "c" and suffix = "c" and remove them, s = "abaaba".
- Take prefix = "a" and suffix = "a" and remove them, s = "baab".
- Take prefix = "b" and suffix = "b" and remove them, s = "aa".
- Take prefix = "a" and suffix = "a" and remove them, s = "".

Example 3:

Input: s = "aabccabba"
Output: 3
Explanation: An optimal sequence of operations is:
- Take prefix = "aa" and suffix = "a" and remove them, s = "bccabb".
- Take prefix = "b" and suffix = "bb" and remove them, s = "cca".

Constraints:

    1 <= s.length <= 10^5
    s only consists of characters 'a', 'b', and 'c'.
*/

struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let s = s.as_bytes();
        if s.is_empty() {
            return 0;
        }
        let mut l = 0;
        let mut r = s.len() - 1;
        while l < r {
            if s[l] != s[r] {
                break;
            }
            while l < r && s[l + 1] == s[l] {
                l += 1;
            }
            while l < r && s[r - 1] == s[r] {
                r -= 1;
            }
            l += 1;
            r -= 1;
        }
        if l > r {
            return 0;
        }
        s[l..r + 1].len() as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_length("ca".to_string()), 2);
    assert_eq!(Solution::minimum_length("cabaabac".to_string()), 0);
    assert_eq!(Solution::minimum_length("aabccabba".to_string()), 3);
}
