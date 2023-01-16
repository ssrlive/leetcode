#![allow(dead_code)]

// 318. Maximum Product of Word Lengths
// https://leetcode.com/problems/maximum-product-of-word-lengths/
// https://leetcode.cn/problems/maximum-product-of-word-lengths/
//
// Given a string array words, return the maximum value of length(word[i]) * length(word[j])
// where the two words do not share common letters. If no such two words exist, return 0.
//
// Example 1:
//
// Input: words = ["abcw","baz","foo","bar","xtfn","abcdef"]
// Output: 16
// Explanation: The two words can be "abcw", "xtfn".
//
// Example 2:
//
// Input: words = ["a","ab","abc","d","cd","bcd","abcd"]
// Output: 4
// Explanation: The two words can be "ab", "cd".
//
// Example 3:
//
// Input: words = ["a","aa","aaa","aaaa"]
// Output: 0
// Explanation: No such pair of words.
//
// Constraints:
//
// - 2 <= words.length <= 1000
// - 1 <= words[i].length <= 1000
// - words[i] consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        fn _max_product(words: Vec<String>) -> Option<i32> {
            let mut map = std::collections::HashMap::new();
            for word in words {
                let mut mask = 0;
                for c in word.chars() {
                    mask |= 1 << (c as u8 - b'a');
                }
                map.entry(mask).or_insert(0);
                *map.get_mut(&mask)? = std::cmp::max(*map.get(&mask)?, word.len());
            }
            let mut max_product = 0;
            for (mask1, len1) in map.iter() {
                for (mask2, len2) in map.iter() {
                    if mask1 & mask2 == 0 {
                        max_product = std::cmp::max(max_product, len1 * len2);
                    }
                }
            }
            Some(max_product as i32)
        }
        _max_product(words).unwrap_or(0)
    }
}

#[test]
fn test_max_product() {
    let words = vec![
        "abcw".to_string(),
        "baz".to_string(),
        "foo".to_string(),
        "bar".to_string(),
        "xtfn".to_string(),
        "abcdef".to_string(),
    ];
    assert_eq!(Solution::max_product(words), 16);

    let words = vec![
        "a".to_string(),
        "ab".to_string(),
        "abc".to_string(),
        "d".to_string(),
        "cd".to_string(),
        "bcd".to_string(),
        "abcd".to_string(),
    ];
    assert_eq!(Solution::max_product(words), 4);

    let words = vec!["a".to_string(), "aa".to_string(), "aaa".to_string(), "aaaa".to_string()];
    assert_eq!(Solution::max_product(words), 0);
}
