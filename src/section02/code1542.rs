#![allow(dead_code)]

/*

// 1542. Find Longest Awesome Substring
// https://leetcode.com/problems/find-longest-awesome-substring/
// https://leetcode.cn/problems/find-longest-awesome-substring/
//
// Hard
//
// You are given a string s. An awesome substring is a non-empty substring of s such that we can make any number of swaps in order to make it a palindrome.

Return the length of the maximum length awesome substring of s.

Example 1:

Input: s = "3242415"
Output: 5
Explanation: "24241" is the longest awesome substring, we can form the palindrome "24142" with some swaps.

Example 2:

Input: s = "12345678"
Output: 1

Example 3:

Input: s = "213123"
Output: 6
Explanation: "213123" is the longest awesome substring, we can form the palindrome "231132" with some swaps.

Constraints:

    1 <= s.length <= 10^5
    s consists only of digits.
*/

struct Solution;

impl Solution {
    pub fn longest_awesome(s: String) -> i32 {
        use std::collections::HashMap;
        let mut map: HashMap<i32, i32> = HashMap::new();
        let mut mask = 0;
        let mut ans = 0;
        map.insert(0, -1);
        for (i, c) in s.bytes().enumerate() {
            mask ^= 1 << (c - b'0');
            if let Some(&j) = map.get(&mask) {
                ans = ans.max(i as i32 - j);
            }
            for k in 0..10 {
                let mask2 = mask ^ (1 << k);
                if let Some(&j) = map.get(&mask2) {
                    ans = ans.max(i as i32 - j);
                }
            }
            map.entry(mask).or_insert(i as i32);
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![("3242415", 5), ("12345678", 1), ("213123", 6)];
    for (s, expected) in cases {
        assert_eq!(Solution::longest_awesome(s.to_string()), expected);
    }
}
