#![allow(dead_code)]

// 2452. Words Within Two Edits of Dictionary
// https://leetcode.com/problems/words-within-two-edits-of-dictionary/
// https://leetcode.cn/problems/words-within-two-edits-of-dictionary/
//
// You are given two string arrays, queries and dictionary.
// All words in each array comprise of lowercase English letters and have the same length.
//
// In one edit you can take a word from queries, and change any letter in it to any other letter. Find all words from queries that,
// after a maximum of two edits, equal some word from dictionary.
//
// Return a list of all words from queries, that match with some word from dictionary after a maximum of two edits.
// Return the words in the same order they appear in queries.
//
// Example 1:
//
// Input: queries = ["word","note","ants","wood"], dictionary = ["wood","joke","moat"]
// Output: ["word","note","wood"]
// Explanation:
// - Changing the 'r' in "word" to 'o' allows it to equal the dictionary word "wood".
// - Changing the 'n' to 'j' and the 't' to 'k' in "note" changes it to "joke".
// - It would take more than 2 edits for "ants" to equal a dictionary word.
// - "wood" can remain unchanged (0 edits) and match the corresponding dictionary word.
// Thus, we return ["word","note","wood"].
//
// Example 2:
//
// Input: queries = ["yes"], dictionary = ["not"]
// Output: []
// Explanation:
// Applying any two edits to "yes" cannot make it equal to "not". Thus, we return an empty array.
//
// Constraints:
//
// - 1 <= queries.length, dictionary.length <= 100
// - n == queries[i].length == dictionary[j].length
// - 1 <= n <= 100
// - All queries[i] and dictionary[j] are composed of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn two_edit_words(queries: Vec<String>, dictionary: Vec<String>) -> Vec<String> {
        fn is_two_edit_word(query: &str, word: &str) -> bool {
            let mut i = 0;
            let mut j = 0;
            let mut diff = 0;
            while i < query.len() && j < word.len() {
                if query.as_bytes()[i] != word.as_bytes()[j] {
                    diff += 1;
                    if diff > 2 {
                        return false;
                    }
                    match query.len().cmp(&word.len()) {
                        std::cmp::Ordering::Greater => i += 1,
                        std::cmp::Ordering::Less => j += 1,
                        std::cmp::Ordering::Equal => {
                            i += 1;
                            j += 1;
                        }
                    }
                } else {
                    i += 1;
                    j += 1;
                }
            }
            diff + (query.len() - i) + (word.len() - j) <= 2
        }

        let mut result = Vec::new();
        for query in queries {
            for word in &dictionary {
                if is_two_edit_word(&query, word) {
                    result.push(query.clone());
                    break;
                }
            }
        }
        result
    }
}

#[test]
fn test() {
    let queries = vec!["word", "note", "ants", "wood"];
    let queries = queries.into_iter().map(|s| s.to_string()).collect();
    let dictionary = vec!["wood", "joke", "moat"];
    let dictionary = dictionary.into_iter().map(|s| s.to_string()).collect();
    let expected = vec!["word", "note", "wood"];
    assert_eq!(Solution::two_edit_words(queries, dictionary), expected);

    let queries = vec!["yes"];
    let queries = queries.into_iter().map(|s| s.to_string()).collect();
    let dictionary = vec!["not"];
    let dictionary = dictionary.into_iter().map(|s| s.to_string()).collect();
    let expected = Vec::<String>::new();
    assert_eq!(Solution::two_edit_words(queries, dictionary), expected);
}
