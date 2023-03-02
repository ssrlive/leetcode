#![allow(dead_code)]

/*

// 2030. Smallest K-Length Subsequence With Occurrences of a Letter
// https://leetcode.com/problems/smallest-k-length-subsequence-with-occurrences-of-a-letter/
// https://leetcode.cn/problems/smallest-k-length-subsequence-with-occurrences-of-a-letter/
//
// Hard
//
// You are given a string s, an integer k, a letter letter, and an integer repetition.

Return the lexicographically smallest subsequence of s of length k that has the letter letter appear at least repetition times. The test cases are generated so that the letter appears in s at least repetition times.

A subsequence is a string that can be derived from another string by deleting some or no characters without changing the order of the remaining characters.

A string a is lexicographically smaller than a string b if in the first position where a and b differ, string a has a letter that appears earlier in the alphabet than the corresponding letter in b.

Example 1:

Input: s = "leet", k = 3, letter = "e", repetition = 1
Output: "eet"
Explanation: There are four subsequences of length 3 that have the letter 'e' appear at least 1 time:
- "lee" (from "leet")
- "let" (from "leet")
- "let" (from "leet")
- "eet" (from "leet")
The lexicographically smallest subsequence among them is "eet".

Example 2:
example-2

Input: s = "leetcode", k = 4, letter = "e", repetition = 2
Output: "ecde"
Explanation: "ecde" is the lexicographically smallest subsequence of length 4 that has the letter "e" appear at least 2 times.

Example 3:

Input: s = "bb", k = 2, letter = "b", repetition = 2
Output: "bb"
Explanation: "bb" is the only subsequence of length 2 that has the letter "b" appear at least 2 times.

Constraints:

    1 <= repetition <= k <= s.length <= 5 * 10^4
    s consists of lowercase English letters.
    letter is a lowercase English letter, and appears in s at least repetition times.
*/

struct Solution;

impl Solution {
    pub fn smallest_subsequence(s: String, k: i32, letter: char, repetition: i32) -> String {
        let letter = letter as i32;
        let s = s.as_bytes().iter().map(|&c| c as i32).collect::<Vec<i32>>();
        let mut repetition = repetition;
        let mut extra = s.iter().filter(|&&c| c == letter).count() as i32 - repetition;
        let mut remove = s.len() as i32 - k;
        let mut mono = Vec::new();
        for &ch in s.iter() {
            while !mono.is_empty() && mono[mono.len() - 1] > ch && remove > 0 {
                let mono_back = mono[mono.len() - 1];
                if mono_back == letter && extra == 0 {
                    break;
                }
                extra -= i32::from(mono_back == letter);
                remove -= 1;
                mono.pop();
            }
            mono.push(ch);
        }
        let mut res = Vec::new();
        let mut i = 0;
        while res.len() < k as usize {
            if mono[i] != letter && res.len() + 0.max(repetition) as usize >= k as usize {
                i += 1;
                continue;
            }
            res.push(mono[i] as u8);
            repetition -= i32::from(mono[i] == letter);
            i += 1;
        }
        String::from_utf8(res).unwrap()
    }
}

#[test]
fn test() {
    let cases = vec![
        ("leet", 3, 'e', 1, "eet"),
        ("leetcode", 4, 'e', 2, "ecde"),
        ("bb", 2, 'b', 2, "bb"),
    ];
    for (s, k, letter, repetition, expected) in cases {
        assert_eq!(
            Solution::smallest_subsequence(s.to_string(), k, letter, repetition),
            expected
        );
    }
}
