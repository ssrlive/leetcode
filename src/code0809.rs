#![allow(dead_code)]

// 809. Expressive Words
// https://leetcode.com/problems/expressive-words/
//
// Sometimes people repeat letters to represent extra feeling. For example:
//
// - "hello" -> "heeellooo"
// - "hi" -> "hiiii"
//
// In these strings like "heeellooo", we have groups of adjacent letters that are all the same: "h", "eee", "ll", "ooo".
//
// You are given a string s and an array of query strings words. A query word is stretchy if it can be made to be equal to s by any number of applications of the following
// extension operation: choose a group consisting of characters c, and add some number of characters c to the group so that the size of the group is three or more.
//
// - For example, starting with "hello", we could do an extension on the group "o" to get "hellooo", but we cannot get "helloo" since the group "oo" has a size less than three.
//   Also, we could do another extension like "ll" -> "lllll" to get "helllllooo". If s = "helllllooo", then the query word "hello" would be stretchy because of these
//   two extension operations: query = "hello" -> "hellooo" -> "helllllooo" = s.
//
// Return the number of query strings that are stretchy.
//
// Example 1:
//
// Input: s = "heeellooo", words = ["hello", "hi", "helo"]
// Output: 1
// Explanation:
// We can extend "e" and "o" in the word "hello" to get "heeellooo".
// We can't extend "helo" to get "heeellooo" because the group "ll" is not size 3 or more.
//
// Example 2:
//
// Input: s = "zzzzzyyyyy", words = ["zzyy","zy","zyy"]
// Output: 3
//
// Constraints:
//
// - 1 <= s.length, words.length <= 100
// - 1 <= words[i].length <= 100
// - s and words[i] consist of lowercase letters.
//

struct Solution;

impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        fn get_freq(s: &str) -> Vec<(char, usize)> {
            let s = s.as_bytes();
            let mut freq = Vec::new();
            let mut count = 1;
            for i in 1..=s.len() {
                if i == s.len() || s[i] != s[i - 1] {
                    freq.push((s[i - 1] as char, count));
                    count = 1;
                } else {
                    count += 1;
                }
            }
            freq
        }

        let orig_freq = get_freq(&s);
        println!("orig_freq = {:?}", orig_freq);
        let mut num_of_expressive_words = 0;
        for word in words.iter() {
            let query_freq = get_freq(word);
            println!("query_freq = {:?}", query_freq);
            if orig_freq.len() != query_freq.len() {
                continue;
            }
            let mut ii = 0;
            for i in 0..orig_freq.len() {
                ii = i;
                if orig_freq[i] == query_freq[i] {
                    ii += 1;
                    continue;
                }
                let is_query_stretchable = query_freq[i].1 < orig_freq[i].1 && orig_freq[i].1 >= 3;
                if orig_freq[i].0 != query_freq[i].0 || !is_query_stretchable {
                    break;
                }
                ii += 1;
            }
            if ii == orig_freq.len() {
                num_of_expressive_words += 1;
            }
        }
        num_of_expressive_words
    }
}

#[test]
fn test() {
    let words = vec!["aaaa".to_string()];
    assert_eq!(Solution::expressive_words("aaa".to_string(), words), 0);

    let words = vec!["hello".to_string(), "hi".to_string(), "helo".to_string()];
    assert_eq!(Solution::expressive_words("heeellooo".to_string(), words), 1);

    let words = vec!["zzyy".to_string(), "zy".to_string(), "zyy".to_string()];
    assert_eq!(Solution::expressive_words("zzzzzyyyyy".to_string(), words), 3);
}
