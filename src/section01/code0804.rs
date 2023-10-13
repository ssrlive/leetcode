#![allow(dead_code)]

// 804. Unique Morse Code Words
// https://leetcode.com/problems/unique-morse-code-words/
// https://leetcode.cn/problems/unique-morse-code-words/
//
// International Morse Code defines a standard encoding where each letter is mapped to a series of dots and dashes, as follows:
//
// - 'a' maps to ".-",
// - 'b' maps to "-...",
// - 'c' maps to "-.-.", and so on.
//
// For convenience, the full table for the 26 letters of the English alphabet is given below:
//
// [".-","-...","-.-.","-..",".","..-.","--.","....","..",".---","-.-",".-..","--","-.","---",".--.","--.-",".-.","...","-","..-","...-",".--","-..-","-.--","--.."]
//
// Given an array of strings words where each word can be written as a concatenation of the Morse code of each letter.
//
// For example, "cab" can be written as "-.-..--...", which is the concatenation of "-.-.", ".-",
// and "-...". We will call such a concatenation the transformation of a word.
//
// Return the number of different transformations among all words we have.
//
// Example 1:
//
// Input: words = ["gin","zen","gig","msg"]
// Output: 2
// Explanation: The transformation of each word is:
// "gin" -> "--...-."
// "zen" -> "--...-."
// "gig" -> "--...--."
// "msg" -> "--...--."
// There are 2 different transformations: "--...-." and "--...--.".
//
// Example 2:
//
// Input: words = ["a"]
// Output: 1
//
// Constraints:
//
// - 1 <= words.length <= 100
// - 1 <= words[i].length <= 12
// - words[i] consists of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let morse_codes = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.",
            "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..",
        ];

        let mut morse_set = std::collections::HashSet::new();

        for word in words {
            let mut morse = String::new();
            for c in word.chars() {
                morse.push_str(morse_codes[(c as u8 - b'a') as usize]);
            }
            morse_set.insert(morse);
        }

        morse_set.len() as i32
    }
}

#[test]
fn test() {
    let words = ["gin", "zen", "gig", "msg"];
    let words = words.iter().map(|s| s.to_string()).collect();
    assert_eq!(Solution::unique_morse_representations(words), 2);

    assert_eq!(Solution::unique_morse_representations(vec!["a".to_string()]), 1);
}
