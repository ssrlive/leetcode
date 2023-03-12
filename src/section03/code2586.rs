#![allow(dead_code)]

/*

// 2586. Count the Number of Vowel Strings in Range
// https://leetcode.com/problems/count-the-number-of-vowel-strings-in-range/
// https://leetcode.cn/problems/count-the-number-of-vowel-strings-in-range/
//
// Easy
//
// You are given a 0-indexed array of string words and two integers left and right.

A string is called a vowel string if it starts with a vowel character and ends with a vowel character where vowel characters are 'a', 'e', 'i', 'o', and 'u'.

Return the number of vowel strings words[i] where i belongs to the inclusive range [left, right].

Example 1:

Input: words = ["are","amy","u"], left = 0, right = 2
Output: 2
Explanation:
- "are" is a vowel string because it starts with 'a' and ends with 'e'.
- "amy" is not a vowel string because it does not end with a vowel.
- "u" is a vowel string because it starts with 'u' and ends with 'u'.
The number of vowel strings in the mentioned range is 2.

Example 2:

Input: words = ["hey","aeo","mu","ooo","artro"], left = 1, right = 4
Output: 3
Explanation:
- "aeo" is a vowel string because it starts with 'a' and ends with 'o'.
- "mu" is not a vowel string because it does not start with a vowel.
- "ooo" is a vowel string because it starts with 'o' and ends with 'o'.
- "artro" is a vowel string because it starts with 'a' and ends with 'o'.
The number of vowel strings in the mentioned range is 3.

Constraints:

    1 <= words.length <= 1000
    1 <= words[i].length <= 10
    words[i] consists of only lowercase English letters.
    0 <= left <= right < words.length
*/

struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, left: i32, right: i32) -> i32 {
        let mut ans = 0;
        for i in left..=right {
            let word = words[i as usize].chars().collect::<Vec<char>>();
            let v = ['a', 'e', 'i', 'o', 'u'];
            if v.contains(&word[0]) && v.contains(&word[word.len() - 1]) {
                ans += 1;
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["are", "amy", "u"], 0, 2, 2),
        (vec!["hey", "aeo", "mu", "ooo", "artro"], 1, 4, 3),
    ];
    for (words, left, right, expected) in cases {
        let words = words.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(Solution::vowel_strings(words, left, right), expected);
    }
}
