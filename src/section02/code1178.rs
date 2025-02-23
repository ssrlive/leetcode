#![allow(dead_code)]

// 1178. Number of Valid Words for Each Puzzle
// https://leetcode.com/problems/number-of-valid-words-for-each-puzzle/
// https://leetcode.cn/problems/number-of-valid-words-for-each-puzzle/
//
// With respect to a given puzzle string, a word is valid if both the following conditions are satisfied:
// word contains the first letter of puzzle.
// For each letter in word, that letter is in puzzle.
// For example, if the puzzle is "abcdefg", then valid words are "faced", "cabbage", and "baggage", while
// invalid words are "beefed" (does not include 'a') and "based" (includes 's' which is not in the puzzle).
// Return an array answer, where answer[i] is the number of words in the given word list words
// that is valid with respect to the puzzle puzzles[i].
//
// Example 1:
//
// Input: words = ["aaaa","asas","able","ability","actt","actor","access"], puzzles = ["aboveyz","abrodyz","abslute","absoryz","actresz","gaswxyz"]
// Output: [1,1,3,2,4,0]
// Explanation:
// 1 valid word for "aboveyz" : "aaaa"
// 1 valid word for "abrodyz" : "aaaa"
// 3 valid words for "abslute" : "aaaa", "asas", "able"
// 2 valid words for "absoryz" : "aaaa", "asas"
// 4 valid words for "actresz" : "aaaa", "asas", "actt", "access"
// There are no valid words for "gaswxyz" cause none of the words in the list contains letter 'g'.
//
// Example 2:
//
// Input: words = ["apple","pleas","please"], puzzles = ["aelwxyz","aelpxyz","aelpsxy","saelpxy","xaelpsy"]
// Output: [0,1,3,2,0]
//
// Constraints:
//
// - 1 <= words.length <= 10^5
// - 4 <= words[i].length <= 50
// - 1 <= puzzles.length <= 10^4
// - puzzles[i].length == 7
// - words[i] and puzzles[i] consist of lowercase English letters.
// - Each puzzles[i] does not contain repeated characters.
//

struct Solution;

impl Solution {
    pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
        use std::collections::HashMap;
        let mut hm = HashMap::new();
        for w in &words {
            *hm.entry(w.bytes().fold(0_u32, |acc, u| acc | (1 << (u - b'a')))).or_insert(0) += 1;
        }
        let f = |p: &String| {
            let first = 1 << (p.as_bytes()[0] - b'a');
            let mask = p.bytes().skip(1).fold(0, |acc, u| acc | (1 << (u - b'a')));
            let mut count = *hm.get(&first).unwrap_or(&0);
            let mut submask = mask;
            while submask > 0 {
                count += hm.get(&(submask | first)).unwrap_or(&0);
                submask = (submask - 1) & mask;
            }
            count
        };
        puzzles.iter().map(f).collect()
    }
}

#[test]
fn test() {
    let words = vec![
        "aaaa".to_string(),
        "asas".to_string(),
        "able".to_string(),
        "ability".to_string(),
        "actt".to_string(),
        "actor".to_string(),
        "access".to_string(),
    ];
    let puzzles = vec![
        "aboveyz".to_string(),
        "abrodyz".to_string(),
        "abslute".to_string(),
        "absoryz".to_string(),
        "actresz".to_string(),
        "gaswxyz".to_string(),
    ];
    let res = vec![1, 1, 3, 2, 4, 0];
    assert_eq!(Solution::find_num_of_valid_words(words, puzzles), res);

    let words = vec!["apple".to_string(), "pleas".to_string(), "please".to_string()];
    let puzzles = vec![
        "aelwxyz".to_string(),
        "aelpxyz".to_string(),
        "aelpsxy".to_string(),
        "saelpxy".to_string(),
        "xaelpsy".to_string(),
    ];
    let res = vec![0, 1, 3, 2, 0];
    assert_eq!(Solution::find_num_of_valid_words(words, puzzles), res);
}
