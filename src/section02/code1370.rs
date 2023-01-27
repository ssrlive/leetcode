#![allow(dead_code)]

// 1370. Increasing Decreasing String
// https://leetcode.com/problems/increasing-decreasing-string/
// https://leetcode.cn/problems/increasing-decreasing-string/
//
// Easy
//
// You are given a string s. Reorder the string using the following algorithm:
//
//     Pick the smallest character from s and append it to the result.
//     Pick the smallest character from s which is greater than the last appended character to the result and append it.
//     Repeat step 2 until you cannot pick more characters.
//     Pick the largest character from s and append it to the result.
//     Pick the largest character from s which is smaller than the last appended character to the result and append it.
//     Repeat step 5 until you cannot pick more characters.
//     Repeat the steps from 1 to 6 until you pick all characters from s.
//
// In each step, If the smallest or the largest character appears more than once you can choose any occurrence and append it to the result.
//
// Return the result string after sorting s with this algorithm.
//
// Example 1:
//
// Input: s = "aaaabbbbcccc"
// Output: "abccbaabccba"
// Explanation: After steps 1, 2 and 3 of the first iteration, result = "abc"
// After steps 4, 5 and 6 of the first iteration, result = "abccba"
// First iteration is done. Now s = "aabbcc" and we go back to step 1
// After steps 1, 2 and 3 of the second iteration, result = "abccbaabc"
// After steps 4, 5 and 6 of the second iteration, result = "abccbaabccba"
//
// Example 2:
//
// Input: s = "rat"
// Output: "art"
// Explanation: The word "rat" becomes "art" after re-ordering it with the mentioned algorithm.
//
// Constraints:
//
// -    1 <= s.length <= 500
// -    s consists of only lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn sort_string(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        let mut counter = [0; 26];
        let (mut incr, mut ind) = (1, 0);
        s.as_bytes().iter().for_each(|b| counter[(*b - b'a') as usize] += 1);
        while res.len() < s.len() {
            if counter[ind as usize] > 0 {
                res.push((b'a' + ind as u8) as char);
                counter[ind as usize] -= 1;
            };
            ind += incr;
            if !(0..=25).contains(&ind) {
                ind = if ind < 0 { 0 } else { 25 };
                incr *= -1;
            };
        }
        res
    }
}

#[test]
fn test() {
    let s = "aaaabbbbcccc".to_string();
    let res = "abccbaabccba".to_string();
    assert_eq!(Solution::sort_string(s), res);

    let s = "rat".to_string();
    let res = "art".to_string();
    assert_eq!(Solution::sort_string(s), res);
}
