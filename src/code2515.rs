#![allow(dead_code)]

// 2515. Shortest Distance to Target String in a Circular Array
// https://leetcode.com/problems/shortest-distance-to-target-string-in-a-circular-array/
// https://leetcode.cn/problems/shortest-distance-to-target-string-in-a-circular-array/
//
// You are given a 0-indexed circular string array words and a string target. A circular array means that the array's end connects to the array's beginning.
//
// Formally, the next element of words[i] is words[(i + 1) % n] and the previous element of words[i] is words[(i - 1 + n) % n], where n is the length of words.
// Starting from startIndex, you can move to either the next word or the previous word with 1 step at a time.
//
// Return the shortest distance needed to reach the string target. If the string target does not exist in words, return -1.
//
// Example 1:
//
// Input: words = ["hello","i","am","leetcode","hello"], target = "hello", startIndex = 1
// Output: 1
// Explanation: We start from index 1 and can reach "hello" by
// - moving 3 units to the right to reach index 4.
// - moving 2 units to the left to reach index 4.
// - moving 4 units to the right to reach index 0.
// - moving 1 unit to the left to reach index 0.
// The shortest distance to reach "hello" is 1.
//
// Example 2:
//
// Input: words = ["a","b","leetcode"], target = "leetcode", startIndex = 0
// Output: 1
// Explanation: We start from index 0 and can reach "leetcode" by
// - moving 2 units to the right to reach index 3.
// - moving 1 unit to the left to reach index 3.
// The shortest distance to reach "leetcode" is 1.
//
// Example 3:
//
// Input: words = ["i","eat","leetcode"], target = "ate", startIndex = 0
// Output: -1
// Explanation: Since "ate" does not exist in words, we return -1.
//
// Constraints:
//
// - 1 <= words.length <= 100
// - 1 <= words[i].length <= 100
// - words[i] and target consist of only lowercase English letters.
// - 0 <= startIndex < words.length
//

struct Solution;

impl Solution {
    pub fn closet_target(words: Vec<String>, target: String, start_index: i32) -> i32 {
        let mut result = -1;
        for i in words
            .iter()
            .enumerate()
            .filter_map(|(i, w)| (w == &target).then_some(i as i32))
        {
            let min_pos = i.min(start_index);
            let max_pos = i.max(start_index);
            let dist = (max_pos - min_pos).min((words.len() as i32 - max_pos) + min_pos);
            result = if result == -1 { dist } else { result.min(dist) }
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec!["hello", "i", "am", "leetcode", "hello"], "hello", 1, 1),
        (vec!["a", "b", "leetcode"], "leetcode", 0, 1),
        (vec!["i", "eat", "leetcode"], "ate", 0, -1),
    ];
    for (words, target, start_index, expected) in cases {
        let words = words.into_iter().map(|s| s.to_string()).collect();
        assert_eq!(
            Solution::closet_target(words, target.to_string(), start_index),
            expected
        );
    }
}
