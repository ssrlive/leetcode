#![allow(dead_code)]

// 692. Top K Frequent Words
// https://leetcode.com/problems/top-k-frequent-words/
//
// Given an array of strings words and an integer k, return the k most frequent strings.
//
// Return the answer sorted by the frequency from highest to lowest. Sort the words with the same frequency by their lexicographical order.
//
// Example 1:
//
// Input: words = ["i","love","leetcode","i","love","coding"], k = 2
// Output: ["i","love"]
// Explanation: "i" and "love" are the two most frequent words.
// Note that "i" comes before "love" due to a lower alphabetical order.
//
// Example 2:
//
// Input: words = ["the","day","is","sunny","the","the","the","sunny","is","is"], k = 4
// Output: ["the","is","sunny","day"]
// Explanation: "the", "is", "sunny" and "day" are the four most frequent words, with the number of occurrence being 4, 3, 2 and 1 respectively.
//
// Constraints:
//
// - 1 <= words.length <= 500
// - 1 <= words[i].length <= 10
// - words[i] consists of lowercase English letters.
// - k is in the range [1, The number of unique words[i]]
//
// Follow-up: Could you solve it in O(n log(k)) time and O(n) extra space?
//

struct Solution;

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        use std::collections::HashMap;

        let mut map = HashMap::new();
        for word in words {
            *map.entry(word).or_insert(0) += 1;
        }

        let mut heap = BinaryHeap::new();
        for (word, count) in map {
            heap.push((count, Reverse(word)));
        }

        let mut result = Vec::new();
        for _ in 0..k {
            if let Some((_, word)) = heap.pop() {
                result.push(word.0);
            }
        }
        result
    }
}

#[test]
fn test() {
    let words = vec!["i", "love", "leetcode", "i", "love", "coding"];
    let words = words.iter().map(|s| s.to_string()).collect();
    let k = 2;
    let result = vec!["i", "love"];
    let r = Solution::top_k_frequent(words, k);
    assert_eq!(r, result);

    let words = vec!["the", "day", "is", "sunny", "the", "the", "the", "sunny", "is", "is"];
    let words = words.iter().map(|s| s.to_string()).collect();
    let k = 4;
    let result = vec!["the", "is", "sunny", "day"];
    assert_eq!(Solution::top_k_frequent(words, k), result);
}
