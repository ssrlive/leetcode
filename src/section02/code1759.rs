#![allow(dead_code)]

/*

// 1759. Count Number of Homogenous Substrings
// https://leetcode.com/problems/count-number-of-homogenous-substrings/
// https://leetcode.cn/problems/count-number-of-homogenous-substrings/
//
// Medium
//
// Given a string s, return the number of homogenous substrings of s. Since the answer may be too large, return it modulo 109 + 7.

A string is homogenous if all the characters of the string are the same.

A substring is a contiguous sequence of characters within a string.

Example 1:

Input: s = "abbcccaa"
Output: 13
Explanation: The homogenous substrings are listed as below:
"a"   appears 3 times.
"aa"  appears 1 time.
"b"   appears 2 times.
"bb"  appears 1 time.
"c"   appears 3 times.
"cc"  appears 2 times.
"ccc" appears 1 time.
3 + 1 + 2 + 1 + 3 + 2 + 1 = 13.

Example 2:

Input: s = "xy"
Output: 2
Explanation: The homogenous substrings are "x" and "y".

Example 3:

Input: s = "zzzzz"
Output: 15

Constraints:

    1 <= s.length <= 10^5
    s consists of lowercase letters.
*/

struct Solution;

impl Solution {
    pub fn count_homogenous(s: String) -> i32 {
        fn natural_sum(n: i64) -> i64 {
            (n * (n + 1)) / 2
        }

        s.chars()
            .chain("\0".chars())
            .fold((0, 0, None), |(acc, count, prev), e| match (Some(e) == prev, prev) {
                (_, None) => (0, 1, Some(e)),
                (true, Some(_)) => (acc, count + 1, Some(e)),
                (false, Some(_)) => (acc + natural_sum(count) % 1000000007, 1, Some(e)),
            })
            .0 as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_homogenous("abbcccaa".to_string()), 13);
    assert_eq!(Solution::count_homogenous("xy".to_string()), 2);
    assert_eq!(Solution::count_homogenous("zzzzz".to_string()), 15);
}
