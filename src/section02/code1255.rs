#![allow(dead_code)]

// 1255. Maximum Score Words Formed by Letters
// https://leetcode.com/problems/maximum-score-words-formed-by-letters/
// https://leetcode.cn/problems/maximum-score-words-formed-by-letters/
//
// Hard
//
// Given a list of words, list of  single letters (might be repeating) and score of every character.
//
// Return the maximum score of any valid set of words formed by using the given letters (words[i] cannot be used two or more times).
//
// It is not necessary to use all characters in letters and each letter can only be used once.
// Score of letters 'a', 'b', 'c', ... ,'z' is given by score[0], score[1], ... , score[25] respectively.
//
// Example 1:
//
// Input: words = ["dog","cat","dad","good"], letters = ["a","a","c","d","d","d","g","o","o"], score = [1,0,9,5,0,0,3,0,0,0,0,0,0,0,2,0,0,0,0,0,0,0,0,0,0,0]
// Output: 23
// Explanation:
// Score  a=1, c=9, d=5, g=3, o=2
// Given letters, we can form the words "dad" (5+1+5) and "good" (3+2+2+5) with a score of 23.
// Words "dad" and "dog" only get a score of 21.
//
// Example 2:
//
// Input: words = ["xxxz","ax","bx","cx"], letters = ["z","a","b","c","x","x","x"], score = [4,4,4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,5,0,10]
// Output: 27
// Explanation:
// Score  a=4, b=4, c=4, x=5, z=10
// Given letters, we can form the words "ax" (4+5), "bx" (4+5) and "cx" (4+5) with a score of 27.
// Word "xxxz" only get a score of 25.
//
// Example 3:
//
// Input: words = ["leetcode"], letters = ["l","e","t","c","o","d"], score = [0,0,1,1,1,0,0,0,0,0,0,1,0,0,1,0,0,0,0,1,0,0,0,0,0,0]
// Output: 0
// Explanation:
// Letter "e" can only be used once.
//
// Constraints:
//
// -    1 <= words.length <= 14
// -    1 <= words[i].length <= 15
// -    1 <= letters.length <= 100
// -    letters[i].length == 1
// -    score.length == 26
// -    0 <= score[i] <= 10
// -    words[i], letters[i] contains only lower case English letters.
//

struct Solution;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        type CounterMap = HashMap<char, u8>;

        pub fn max_score_words_local(words: Vec<(CounterMap, i32)>, letters: &CounterMap, _score: &CounterMap) -> i32 {
            let mut max_score: i32 = 0;
            for (w_i, (w, w_score)) in words.iter().enumerate() {
                let mut letters = letters.clone();
                let mut invalid = false;
                for (c, count_needed) in w.iter() {
                    let letters_remaining = if letters.contains_key(c) {
                        *letters.get(c).unwrap()
                    } else {
                        0
                    };
                    if &letters_remaining < count_needed {
                        invalid = true;
                        break;
                    }
                    letters.insert(*c, letters_remaining - count_needed);
                }
                if invalid {
                    continue;
                }
                let mut new_words = words.clone();
                new_words.remove(w_i);
                let w_recurse_score = w_score + max_score_words_local(new_words, &letters, _score);
                max_score = max_score.max(w_recurse_score);
            }
            max_score
        }

        let mut scores_map = CounterMap::new();
        for i in 0..26 {
            let c = char::from_u32(('a' as u32) + i).unwrap();
            scores_map.insert(c, score[i as usize] as u8);
        }

        let words = words
            .iter()
            .map(|word| {
                let mut map = CounterMap::new();
                for w in word.chars() {
                    *map.entry(w).or_insert(0) += 1;
                }

                let w_score = map
                    .iter()
                    .map(|(c, count)| count * scores_map.get(c).unwrap())
                    .sum::<u8>() as i32;

                (map, w_score)
            })
            .collect::<Vec<(CounterMap, i32)>>();

        let mut letters_map = CounterMap::new();
        for letter in letters {
            *letters_map.entry(letter).or_insert(0) += 1;
        }

        max_score_words_local(words, &letters_map, &scores_map)
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec!["dog", "cat", "dad", "good"],
            vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
            vec![
                1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            23,
        ),
        (
            vec!["xxxz", "ax", "bx", "cx"],
            vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'],
            vec![
                4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10,
            ],
            27,
        ),
        (
            vec!["leetcode"],
            vec!['l', 'e', 't', 'c', 'o', 'd'],
            vec![
                0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
            ],
            0,
        ),
    ];
    for (words, letters, score, expected) in cases {
        let words = words.iter().map(|s| s.to_string()).collect();
        let letters = letters.iter().map(|c| *c).collect();
        assert_eq!(Solution::max_score_words(words, letters, score), expected);
    }
}
