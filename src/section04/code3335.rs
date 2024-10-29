#![allow(dead_code)]

// 3335. Total Characters in String After Transformations I
// https://leetcode.com/problems/total-characters-in-string-after-transformations-i/
// https://leetcode.cn/problems/total-characters-in-string-after-transformations-i/
//
// Medium
//
// You are given a string s and an integer t, representing the number of transformations to perform.
// In one transformation, every character in s is replaced according to the following rules:
//
// - If the character is 'z', replace it with the string "ab".
// - Otherwise, replace it with the next character in the alphabet.
//   For example, 'a' is replaced with 'b', 'b' is replaced with 'c', and so on.
//
// Return the length of the resulting string after exactly t transformations.
//
// Since the answer may be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: s = "abcyy", t = 2
//
// Output: 7
//
// Explanation:
//
//     First Transformation (t = 1):
//         'a' becomes 'b'
//         'b' becomes 'c'
//         'c' becomes 'd'
//         'y' becomes 'z'
//         'y' becomes 'z'
//         String after the first transformation: "bcdzz"
//     Second Transformation (t = 2):
//         'b' becomes 'c'
//         'c' becomes 'd'
//         'd' becomes 'e'
//         'z' becomes "ab"
//         'z' becomes "ab"
//         String after the second transformation: "cdeabab"
//     Final Length of the string: The string is "cdeabab", which has 7 characters.
//
// Example 2:
//
// Input: s = "azbk", t = 1
//
// Output: 5
//
// Explanation:
//
//     First Transformation (t = 1):
//         'a' becomes 'b'
//         'z' becomes "ab"
//         'b' becomes 'c'
//         'k' becomes 'l'
//         String after the first transformation: "babcl"
//     Final Length of the string: The string is "babcl", which has 5 characters.
//
// Constraints:
//
//     1 <= s.length <= 10^5
//     s consists only of lowercase English letters.
//     1 <= t <= 10^5
//

struct Solution;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        let module = 10_i32.pow(9) + 7;
        // use a hashmap to update the frequency of each letter
        let mut frequency_map: std::collections::HashMap<char, i32> = ('a'..='z').map(|c| (c, 0)).collect();
        for ch in s.chars() {
            *frequency_map.get_mut(&ch).unwrap() += 1;
        }
        for _ in 0..t {
            // step 1: remember the frequency of z, as it will be overwritten later
            let z_freq = *frequency_map.get(&'z').unwrap() % module;
            // step 2: update each non-z character from y to b
            for ch in ('a'..='y').rev() {
                let ch_freq = frequency_map.get(&ch).unwrap();
                let next_ch = (ch as u8 + 1) as char;
                *frequency_map.get_mut(&next_ch).unwrap() = *ch_freq;
            }
            // step 3: transform 'z'
            *frequency_map.get_mut(&'a').unwrap() = z_freq;
            // avoid add overflow
            let tmp = *frequency_map.get(&'b').unwrap() % module;
            *frequency_map.get_mut(&'b').unwrap() = (tmp + z_freq) % module;
        }
        frequency_map.values().fold(0_i32, |acc, &x| (acc + x) % module)
    }
}

#[test]
fn test() {
    let s = "abcyy".to_string();
    let t = 2;
    let output = 7;
    assert_eq!(Solution::length_after_transformations(s, t), output);

    let s = "azbk".to_string();
    let t = 1;
    let output = 5;
    assert_eq!(Solution::length_after_transformations(s, t), output);
}
