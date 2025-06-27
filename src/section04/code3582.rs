#![allow(dead_code)]

// 3582. Generate Tag for Video Caption
// https://leetcode.com/problems/generate-tag-for-video-caption/
// https://leetcode.cn/problems/generate-tag-for-video-caption/
//
// Easy
//
// You are given a string caption representing the caption for a video.
//
// The following actions must be performed in order to generate a valid tag for the video:
//
// - Combine all words in the string into a single camelCase string prefixed with '#'. A camelCase string is one where the first
//   letter of all words except the first one is capitalized. All characters after the first character in each word must be lowercase.
//
// - Remove all characters that are not an English letter, except the first '#'.
//
// - Truncate the result to a maximum of 100 characters.
//
// Return the tag after performing the actions on caption.
//
// Example 1:
//
// Input: caption = "Leetcode daily streak achieved"
//
// Output: "#leetcodeDailyStreakAchieved"
//
// Explanation:
//
// The first letter for all words except "leetcode" should be capitalized.
//
// Example 2:
//
// Input: caption = "can I Go There"
//
// Output: "#canIGoThere"
//
// Explanation:
//
// The first letter for all words except "can" should be capitalized.
//
// Example 3:
//
// Input: caption = "hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh"
//
// Output: "#hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh"
//
// Explanation:
//
// Since the first word has length 101, we need to truncate the last two letters from the word.
//
// Constraints:
//
//     1 <= caption.length <= 150
//     caption consists only of English letters and ' '.
//

struct Solution;

impl Solution {
    pub fn generate_tag(caption: String) -> String {
        if caption.trim().is_empty() {
            return "#".to_string();
        }

        let words: Vec<String> = caption.split_whitespace().map(|word| word.to_lowercase()).collect();

        if words.is_empty() {
            return "#".to_string();
        }

        let first_word = words[0].clone();
        let mut tag = format!("#{first_word}");

        for word in &words[1..] {
            if tag.len() >= 100 {
                break;
            }
            let capitalized_word = word
                .chars()
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_uppercase().to_string()
                    } else {
                        c.to_lowercase().to_string()
                    }
                })
                .collect::<String>();
            tag.push_str(&capitalized_word);
        }

        tag.chars().take(100).collect()
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::generate_tag("Leetcode daily streak achieved".to_string()),
        "#leetcodeDailyStreakAchieved"
    );
    assert_eq!(Solution::generate_tag("can I Go There".to_string()), "#canIGoThere");
    assert_eq!(
        Solution::generate_tag(
            "hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh".to_string()
        ),
        "#hhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhhh"
    );
}
