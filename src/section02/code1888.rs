#![allow(dead_code)]

/*

// 1888. Minimum Number of Flips to Make the Binary String Alternating
// https://leetcode.com/problems/minimum-number-of-flips-to-make-the-binary-string-alternating/
// https://leetcode.cn/problems/minimum-number-of-flips-to-make-the-binary-string-alternating/
//
// Medium
//
// You are given a binary string s. You are allowed to perform two types of operations on the string in any sequence:

    Type-1: Remove the character at the start of the string s and append it to the end of the string.
    Type-2: Pick any character in s and flip its value, i.e., if its value is '0' it becomes '1' and vice-versa.

Return the minimum number of type-2 operations you need to perform such that s becomes alternating.

The string is called alternating if no two adjacent characters are equal.

    For example, the strings "010" and "1010" are alternating, while the string "0100" is not.

Example 1:

Input: s = "111000"
Output: 2
Explanation: Use the first operation two times to make s = "100011".
Then, use the second operation on the third and sixth elements to make s = "101010".

Example 2:

Input: s = "010"
Output: 0
Explanation: The string is already alternating.

Example 3:

Input: s = "1110"
Output: 1
Explanation: Use the second operation on the second element to make s = "1010".

Constraints:

    1 <= s.length <= 10^5
    s[i] is either '0' or '1'.
*/

struct Solution;

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        use std::cmp::min;
        let len = s.len();
        let chars = s.chars().collect::<Vec<char>>();
        let ch = ['0', '1'];
        let mut cnt = 0;
        for i in 0..len {
            if chars[i] != ch[i % 2] {
                cnt += 1;
            }
        }
        let mut ans = min(cnt, len as i32 - cnt);
        for i in 0..len {
            if chars[i] != ch[i % 2] {
                cnt -= 1;
            }
            if chars[i] != ch[(i + len) % 2] {
                cnt += 1;
            }
            ans = ans.min(min(cnt, len as i32 - cnt));
        }
        ans
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("111000", 2),
        ("010", 0),
        ("1110", 1),
    ];
    for (s, expect) in cases {
        assert_eq!(Solution::min_flips(s.to_string()), expect);
    }
}
