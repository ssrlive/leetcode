#![allow(dead_code)]

/*

// 2068. Check Whether Two Strings are Almost Equivalent
// https://leetcode.com/problems/check-whether-two-strings-are-almost-equivalent/
// https://leetcode.cn/problems/check-whether-two-strings-are-almost-equivalent/
//
// Easy
//
// Two strings word1 and word2 are considered almost equivalent if the differences between the frequencies of each letter from 'a' to 'z' between word1 and word2 is at most 3.

Given two strings word1 and word2, each of length n, return true if word1 and word2 are almost equivalent, or false otherwise.

The frequency of a letter x is the number of times it occurs in the string.

Example 1:

Input: word1 = "aaaa", word2 = "bccb"
Output: false
Explanation: There are 4 'a's in "aaaa" but 0 'a's in "bccb".
The difference is 4, which is more than the allowed 3.

Example 2:

Input: word1 = "abcdeef", word2 = "abaaacc"
Output: true
Explanation: The differences between the frequencies of each letter in word1 and word2 are at most 3:
- 'a' appears 1 time in word1 and 4 times in word2. The difference is 3.
- 'b' appears 1 time in word1 and 1 time in word2. The difference is 0.
- 'c' appears 1 time in word1 and 2 times in word2. The difference is 1.
- 'd' appears 1 time in word1 and 0 times in word2. The difference is 1.
- 'e' appears 2 times in word1 and 0 times in word2. The difference is 2.
- 'f' appears 1 time in word1 and 0 times in word2. The difference is 1.

Example 3:

Input: word1 = "cccddabba", word2 = "babababab"
Output: true
Explanation: The differences between the frequencies of each letter in word1 and word2 are at most 3:
- 'a' appears 2 times in word1 and 4 times in word2. The difference is 2.
- 'b' appears 2 times in word1 and 5 times in word2. The difference is 3.
- 'c' appears 3 times in word1 and 0 times in word2. The difference is 3.
- 'd' appears 2 times in word1 and 0 times in word2. The difference is 2.

Constraints:

    n == word1.length == word2.length
    1 <= n <= 100
    word1 and word2 consist only of lowercase English letters.
*/

struct Solution;

impl Solution {
    pub fn check_almost_equivalent(word1: String, word2: String) -> bool {
        let get_count = |s: &str| -> [i32; 26] {
            let mut count = [0; 26];
            s.as_bytes().iter().for_each(|&b| count[(b - b'a') as usize] += 1);
            count
        };
        let count_1 = get_count(&word1);
        let count_2 = get_count(&word2);
        count_1.iter().zip(count_2.iter()).all(|(n1, n2)| (n1 - n2).abs() <= 3)
    }
}

#[test]
fn test() {
    let cases = vec![
        ("aaaa", "bccb", false),
        ("abcdeef", "abaaacc", true),
        ("cccddabba", "babababab", true),
    ];
    for (w1, w2, ex) in cases {
        assert_eq!(Solution::check_almost_equivalent(w1.to_string(), w2.to_string()), ex);
    }
}
