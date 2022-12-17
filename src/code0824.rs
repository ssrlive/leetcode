#![allow(dead_code)]

// 824. Goat Latin
// https://leetcode.com/problems/goat-latin/
//
// You are given a string sentence that consist of words separated by spaces. Each word consists of lowercase and uppercase letters only.
//
// We would like to convert the sentence to "Goat Latin" (a made-up language similar to Pig Latin.) The rules of Goat Latin are as follows:
//
// - If a word begins with a vowel ('a', 'e', 'i', 'o', or 'u'), append "ma" to the end of the word.
//   - For example, the word "apple" becomes "applema".
// - If a word begins with a consonant (i.e., not a vowel), remove the first letter and append it to the end, then add "ma".
//   - For example, the word "goat" becomes "oatgma".
// - Add one letter 'a' to the end of each word per its word index in the sentence, starting with 1.
//   - For example, the first word gets "a" added to the end, the second word gets "aa" added to the end, and so on.
//
// Return the final sentence representing the conversion from sentence to Goat Latin.
//
// Example 1:
//
// Input: sentence = "I speak Goat Latin"
// Output: "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
//
// Example 2:
//
// Input: sentence = "The quick brown fox jumped over the lazy dog"
// Output: "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"
//
// Constraints:
//
// - 1 <= sentence.length <= 150
// - sentence consists of English letters and spaces.
// - sentence has no leading or trailing spaces.
// - All the words in sentence are separated by a single space.
//

struct Solution;

impl Solution {
    pub fn to_goat_latin(sentence: String) -> String {
        let mut result = String::new();
        let mut a = String::new();
        for word in sentence.split(' ') {
            a.push('a');
            if word.starts_with(|c: char| "aeiouAEIOU".contains(c)) {
                result.push_str(word);
            } else {
                result.push_str(&word[1..]);
                result.push(word.chars().next().unwrap());
            }
            result.push_str("ma");
            result.push_str(&a);
            result.push(' ');
        }
        result.pop();
        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::to_goat_latin("I speak Goat Latin".to_string()),
        "Imaa peaksmaaa oatGmaaaa atinLmaaaaa".to_string()
    );
    assert_eq!(
        Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string()),
        "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"
            .to_string()
    );
}
