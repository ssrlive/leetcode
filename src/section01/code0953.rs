#![allow(dead_code)]

// 953. Verifying an Alien Dictionary
// https://leetcode.com/problems/verifying-an-alien-dictionary/
// https://leetcode.cn/problems/verifying-an-alien-dictionary/
//
// In an alien language, surprisingly, they also use English lowercase letters, but possibly in a different order. The order of the alphabet is some permutation of lowercase letters.
//
// Given a sequence of words written in the alien language, and the order of the alphabet, return true if and only if the given words are sorted lexicographically in this alien language.
//
// Example 1:
//
// Input: words = ["hello","leetcode"], order = "hlabcdefgijkmnopqrstuvwxyz"
// Output: true
// Explanation: As 'h' comes before 'l' in this language, then the sequence is sorted.
//
// Example 2:
//
// Input: words = ["word","world","row"], order = "worldabcefghijkmnpqstuvxyz"
// Output: false
// Explanation: As 'd' comes after 'l' in this language, then words[0] > words[1], hence the sequence is unsorted.
//
// Example 3:
//
// Input: words = ["apple","app"], order = "abcdefghijklmnopqrstuvwxyz"
// Output: false
// Explanation: The first three characters "app" match, and the second string is shorter (in size.)
//   According to lexicographical rules "apple" > "app", because 'l' > '∅', where '∅' is defined as the blank character
//   which is less than any other character (More info).
//
// Constraints:
//
// - 1 <= words.length <= 100
// - 1 <= words[i].length <= 20
// - order.length == 26
// - All characters in words[i] and order are English lowercase letters.
//

struct Solution;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let pos = |c: char| -> usize { (c as u8 - b'a') as usize };
        let mut ord = [0; 26];
        order.char_indices().for_each(|(ind, c)| ord[pos(c)] = ind);
        words
            .windows(2)
            .map(|pair| {
                let (w0, w1) = (&pair[0], &pair[1]);
                w1.chars().map(|c| ord[pos(c)]).ge(w0.chars().map(|c| ord[pos(c)]))
            })
            .all(|x| x)
    }
}

#[test]
fn test() {
    let words = vec!["hello".to_string(), "leetcode".to_string()];
    let order = "hlabcdefgijkmnopqrstuvwxyz".to_string();
    assert!(Solution::is_alien_sorted(words, order));

    let words = vec!["word".to_string(), "world".to_string(), "row".to_string()];
    let order = "worldabcefghijkmnpqstuvxyz".to_string();
    assert!(!Solution::is_alien_sorted(words, order));

    let words = vec!["apple".to_string(), "app".to_string()];
    let order = "abcdefghijklmnopqrstuvwxyz".to_string();
    assert!(!Solution::is_alien_sorted(words, order));
}
