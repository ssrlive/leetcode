#![allow(dead_code)]

// 1371. Find the Longest Substring Containing Vowels in Even Counts
// https://leetcode.com/problems/find-the-longest-substring-containing-vowels-in-even-counts/
// https://leetcode.cn/problems/find-the-longest-substring-containing-vowels-in-even-counts/
//
// Medium
//
// Given the string s, return the size of the longest substring containing each vowel an even number of times.
// That is, 'a', 'e', 'i', 'o', and 'u' must appear an even number of times.
//
// Example 1:
//
// Input: s = "eleetminicoworoep"
// Output: 13
// Explanation: The longest substring is "leetminicowor" which contains two each of the vowels: e, i and o and zero of the vowels: a and u.
//
// Example 2:
//
// Input: s = "leetcodeisgreat"
// Output: 5
// Explanation: The longest substring is "leetc" which contains two e's.
//
// Example 3:
//
// Input: s = "bcbcbc"
// Output: 6
// Explanation: In this case, the given string "bcbcbc" is the longest because all vowels: a, e, i, o and u appear zero times.
//
// Constraints:
//
// -    1 <= s.length <= 5 x 10^5
// -    s contains only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut res = 0;
        let mut state = 0;
        let mut seen = [-1; 1 << 5];
        seen[0] = 0;
        for (i, c) in s.chars().enumerate() {
            match c {
                'a' => state ^= 1 << 0,
                'e' => state ^= 1 << 1,
                'i' => state ^= 1 << 2,
                'o' => state ^= 1 << 3,
                'u' => state ^= 1 << 4,
                _ => {}
            }
            if seen[state] >= 0 {
                res = res.max(i + 1 - seen[state] as usize);
            } else {
                seen[state] = i as i32 + 1;
            }
        }
        res as i32
    }
}

#[test]
fn test() {
    let s = "eleetminicoworoep".to_string();
    let res = 13;
    assert_eq!(Solution::find_the_longest_substring(s), res);

    let s = "leetcodeisgreat".to_string();
    let res = 5;
    assert_eq!(Solution::find_the_longest_substring(s), res);

    let s = "bcbcbc".to_string();
    let res = 6;
    assert_eq!(Solution::find_the_longest_substring(s), res);
}
