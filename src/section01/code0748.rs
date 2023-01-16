#![allow(dead_code)]

// 748. Shortest Completing Word
// https://leetcode.com/problems/shortest-completing-word/
// https://leetcode.cn/problems/shortest-completing-word/
//
// Given a string licensePlate and an array of strings words, find the shortest completing word in words.
//
// A completing word is a word that contains all the letters in licensePlate. Ignore numbers and spaces in licensePlate, and treat letters as case insensitive.
// If a letter appears more than once in licensePlate, then it must appear in the word the same number of times or more.
//
// For example, if licensePlate = "aBc 12c", then it contains letters 'a', 'b' (ignoring case), and 'c' twice.
// Possible completing words are "abccdef", "caaacab", and "cbca".
//
// Return the shortest completing word in words. It is guaranteed an answer exists.
// If there are multiple shortest completing words, return the first one that occurs in words.
//
// Example 1:
//
// Input: licensePlate = "1s3 PSt", words = ["step","steps","stripe","stepple"]
// Output: "steps"
// Explanation: licensePlate contains letters 's', 'p', 's' (ignoring case), and 't'.
// "step" contains 't' and 'p', but only contains 1 's'.
// "steps" contains 't', 'p', and both 's' characters.
// "stripe" is missing an 's'.
// "stepple" is missing an 's'.
// Since "steps" is the only word containing all the letters, that is the answer.
//
// Example 2:
//
// Input: licensePlate = "1s3 456", words = ["looks","pest","stew","show"]
// Output: "pest"
// Explanation: licensePlate only contains the letter 's'. All the words contain 's', but among these "pest", "stew", and "show" are shortest.
//              The answer is "pest" because it is the word that appears earliest of the 3.
//
// Constraints:
//
// - 1 <= licensePlate.length <= 7
// - licensePlate contains digits, letters (uppercase or lowercase), or space ' '.
// - 1 <= words.length <= 1000
// - 1 <= words[i].length <= 15
// - words[i] consists of lower case English letters.
//

struct Solution;

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let chs = license_plate.chars();
        let mut letters = vec![];

        for ch in chs {
            if ch.is_alphabetic() {
                letters.push(ch.to_lowercase().to_string());
            }
        }

        let mut res = (words.len(), "abcdefghijklmnopqrstuvwxyz".to_string());

        for (i, item) in words.iter().enumerate() {
            let mut w = item.clone();
            let mut j = 0;
            while j < letters.len() {
                if let Some(x) = w.find(&letters[j]) {
                    w.remove(x);
                } else {
                    break;
                }
                j += 1;
            }

            if j == letters.len() && (item.len() < res.1.len() || (item.len() == res.1.len() && i < res.0)) {
                res = (i, item.clone());
            }
        }

        res.1
    }
}

#[test]
fn test() {
    let license_plate = "1s3 PSt".to_string();
    let words = vec!["step", "steps", "stripe", "stepple"];
    let words = words.into_iter().map(|s| s.to_string()).collect();
    let expected = "steps";
    assert_eq!(Solution::shortest_completing_word(license_plate, words), expected);

    let license_plate = "1s3 456".to_string();
    let words = vec!["looks", "pest", "stew", "show"];
    let words = words.into_iter().map(|s| s.to_string()).collect();
    let expected = "pest";
    assert_eq!(Solution::shortest_completing_word(license_plate, words), expected);
}
