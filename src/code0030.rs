#![allow(dead_code)]

// 30. Substring with Concatenation of All Words
// https://leetcode.com/problems/substring-with-concatenation-of-all-words/
// https://leetcode.cn/problems/substring-with-concatenation-of-all-words/
//
// You are given a string s and an array of strings words. All the strings of words are of the same length.
//
// A concatenated substring in s is a substring that contains all the strings of any permutation of words concatenated.
//
// For example, if words = ["ab","cd","ef"], then "abcdef", "abefcd", "cdabef", "cdefab", "efabcd", and "efcdab" are all concatenated strings. "acdbef" is not a concatenated substring because it is not the concatenation of any permutation of words.
// Return the starting indices of all the concatenated substrings in s. You can return the answer in any order.
//
// Example 1:
//
// Input: s = "barfoothefoobarman", words = ["foo","bar"]
// Output: [0,9]
// Explanation: Since words.length == 2 and words[i].length == 3, the concatenated substring has to be of length 6.
// The substring starting at 0 is "barfoo". It is the concatenation of ["bar","foo"] which is a permutation of words.
// The substring starting at 9 is "foobar". It is the concatenation of ["foo","bar"] which is a permutation of words.
// The output order does not matter. Returning [9,0] is fine too.
//
// Example 2:
//
// Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
// Output: []
// Explanation: Since words.length == 4 and words[i].length == 4, the concatenated substring has to be of length 16.
// There is no substring of length 16 is s that is equal to the concatenation of any permutation of words.
// We return an empty array.
//
// Example 3:
//
// Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
// Output: [6,9,12]
// Explanation: Since words.length == 3 and words[i].length == 3, the concatenated substring has to be of length 9.
// The substring starting at 6 is "foobarthe". It is the concatenation of ["foo","bar","the"] which is a permutation of words.
// The substring starting at 9 is "barthefoo". It is the concatenation of ["bar","the","foo"] which is a permutation of words.
// The substring starting at 12 is "thefoobar". It is the concatenation of ["the","foo","bar"] which is a permutation of words.
//
// Constraints:
//
// - 1 <= s.length <= 10^4
// - 1 <= words.length <= 5000
// - 1 <= words[i].length <= 30
// - s and words[i] consist of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let s = s.as_bytes();
        let n = s.len();
        let k = words[0].len();
        if k > n {
            return vec![];
        }
        let mut map = words.iter().fold(
            std::collections::HashMap::<&[u8], (usize, usize)>::new(),
            |mut map, word| {
                map.entry(word.as_bytes()).or_default().0 += 1;
                map
            },
        );
        let mut map_is_reset = true;
        let mut rez = vec![];
        for i in 0..k {
            if !map_is_reset {
                map.iter_mut().for_each(|(_, value)| value.1 = 0);
                map_is_reset = true;
            }
            let (mut lo, mut hi) = (i, i);
            while hi <= n - k {
                match map.get_mut(&s[hi..hi + k]) {
                    None => {
                        hi += k;
                        lo = hi;
                        if !map_is_reset {
                            map.iter_mut().for_each(|(_, value)| value.1 = 0);
                            map_is_reset = true;
                        }
                    }
                    Some(hi_value) => {
                        hi_value.1 += 1;
                        hi += k;
                        map_is_reset = false;
                        if hi_value.1 > hi_value.0 {
                            loop {
                                let lo_value = map.get_mut(&s[lo..lo + k]).unwrap();
                                lo += k;
                                lo_value.1 -= 1;
                                if lo_value.0 == lo_value.1 {
                                    break;
                                }
                            }
                        }
                    }
                }

                if hi - lo == words.len() * k {
                    rez.push(lo as i32);
                    map.get_mut(&s[lo..lo + k]).unwrap().1 -= 1;
                    lo += k;
                }
            }
        }
        rez
    }
}

#[test]
fn test() {
    let s = "barfoothefoobarman".to_string();
    let words = vec!["foo".to_string(), "bar".to_string()];
    let rez = Solution::find_substring(s, words);
    assert_eq!(rez, vec![0, 9]);

    let s = "wordgoodgoodgoodbestword".to_string();
    let words = vec![
        "word".to_string(),
        "good".to_string(),
        "best".to_string(),
        "word".to_string(),
    ];
    let rez = Solution::find_substring(s, words);
    assert_eq!(rez, vec![]);

    let s = "barfoofoobarthefoobarman".to_string();
    let words = vec!["bar".to_string(), "foo".to_string(), "the".to_string()];
    let rez = Solution::find_substring(s, words);
    assert_eq!(rez, vec![6, 9, 12]);
}
