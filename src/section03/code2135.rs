#![allow(dead_code)]

/*

// 2135. Count Words Obtained After Adding a Letter
// https://leetcode.com/problems/count-words-obtained-after-adding-a-letter/
// https://leetcode.cn/problems/count-words-obtained-after-adding-a-letter/
//
// Medium
//
// You are given two 0-indexed arrays of strings startWords and targetWords. Each string consists of lowercase English letters only.

For each string in targetWords, check if it is possible to choose a string from startWords and perform a conversion operation on it to be equal to that from targetWords.

The conversion operation is described in the following two steps:

    Append any lowercase letter that is not present in the string to its end.
        For example, if the string is "abc", the letters 'd', 'e', or 'y' can be added to it, but not 'a'. If 'd' is added, the resulting string will be "abcd".
    Rearrange the letters of the new string in any arbitrary order.
        For example, "abcd" can be rearranged to "acbd", "bacd", "cbda", and so on. Note that it can also be rearranged to "abcd" itself.

Return the number of strings in targetWords that can be obtained by performing the operations on any string of startWords.

Note that you will only be verifying if the string in targetWords can be obtained from a string in startWords by performing the operations. The strings in startWords do not actually change during this process.

Example 1:

Input: startWords = ["ant","act","tack"], targetWords = ["tack","act","acti"]
Output: 2
Explanation:
- In order to form targetWords[0] = "tack", we use startWords[1] = "act", append 'k' to it, and rearrange "actk" to "tack".
- There is no string in startWords that can be used to obtain targetWords[1] = "act".
  Note that "act" does exist in startWords, but we must append one letter to the string before rearranging it.
- In order to form targetWords[2] = "acti", we use startWords[1] = "act", append 'i' to it, and rearrange "acti" to "acti" itself.

Example 2:

Input: startWords = ["ab","a"], targetWords = ["abc","abcd"]
Output: 1
Explanation:
- In order to form targetWords[0] = "abc", we use startWords[0] = "ab", add 'c' to it, and rearrange it to "abc".
- There is no string in startWords that can be used to obtain targetWords[1] = "abcd".

Constraints:

    1 <= startWords.length, targetWords.length <= 5 * 10^4
    1 <= startWords[i].length, targetWords[j].length <= 26
    Each string of startWords and targetWords consists of lowercase English letters only.
    No letter occurs more than once in any string of startWords or targetWords.
*/

struct Solution;

impl Solution {
    pub fn word_count(start_words: Vec<String>, target_words: Vec<String>) -> i32 {
        use std::collections::HashSet;
        fn freq(word: &str) -> [u8; (b'z' - b'a' + 1) as usize] {
            let mut f = [0u8; (b'z' - b'a' + 1) as usize];
            for &ch in word.as_bytes() {
                f[(ch - b'a') as usize] += 1;
            }
            f
        }

        let mut start_freq = HashSet::new();
        start_words.into_iter().for_each(|w| {
            start_freq.insert(freq(&w));
        });
        let mut answer = 0;
        target_words.into_iter().for_each(|w| {
            let mut f = freq(&w);
            for &ch in w.as_bytes() {
                let idx = (ch - b'a') as usize;
                f[idx] = 0;
                if start_freq.contains(&f) {
                    answer += 1;
                    break;
                }
                f[idx] = 1;
            }
        });
        answer
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec!["ant".to_string(), "act".to_string(), "tack".to_string()],
            vec!["tack".to_string(), "act".to_string(), "acti".to_string()],
            2,
        ),
        (
            vec!["ab".to_string(), "a".to_string()],
            vec!["abc".to_string(), "abcd".to_string()],
            1,
        ),
    ];
    for (start_words, target_words, expected) in cases {
        assert_eq!(Solution::word_count(start_words, target_words), expected);
    }
}
