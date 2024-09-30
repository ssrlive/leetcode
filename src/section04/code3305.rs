#![allow(dead_code)]

// 3305. Count of Substrings Containing Every Vowel and K Consonants I
// https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-i/
// https://leetcode.cn/problems/count-of-substrings-containing-every-vowel-and-k-consonants-i/
//
// Medium
//
// You are given a string word and a non-negative integer k.
//
// Return the total number of substrings of word that contain every vowel ('a', 'e', 'i', 'o', and 'u') at least once and exactly k consonants.
//
// Example 1:
//
// Input: word = "aeioqq", k = 1
//
// Output: 0
//
// Explanation:
//
// There is no substring with every vowel.
//
// Example 2:
//
// Input: word = "aeiou", k = 0
//
// Output: 1
//
// Explanation:
//
// The only substring with every vowel and zero consonants is word[0..4], which is "aeiou".
//
// Example 3:
//
// Input: word = "ieaouqqieaouqq", k = 1
//
// Output: 3
//
// Explanation:
//
// The substrings with every vowel and one consonant are:
//
//     word[0..5], which is "ieaouq".
//     word[6..11], which is "qieaou".
//     word[7..12], which is "ieaouq".
//
// Constraints:
//
//     5 <= word.length <= 250
//     word consists only of lowercase English letters.
//     0 <= k <= word.length - 5
//

struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i32 {
        let word = word.as_bytes();
        let mut ans = 0;
        for i in 0..word.len() {
            let (mut a, mut e, mut m, mut o, mut u, mut c) = (0, 0, 0, 0, 0, 0);
            for &word_j in word.iter().skip(i) {
                match word_j {
                    b'a' => a += 1,
                    b'e' => e += 1,
                    b'i' => m += 1,
                    b'o' => o += 1,
                    b'u' => u += 1,
                    _ => c += 1,
                }

                if a > 0 && e > 0 && m > 0 && o > 0 && u > 0 && c == k {
                    ans += 1;
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_of_substrings("aeioqq".to_string(), 1), 0);
    assert_eq!(Solution::count_of_substrings("aeiou".to_string(), 0), 1);
    assert_eq!(Solution::count_of_substrings("ieaouqqieaouqq".to_string(), 1), 3);
}
