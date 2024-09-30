#![allow(dead_code)]

// 3306. Count of Substrings Containing Every Vowel and K Consonants II
// https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-ii/
// https://leetcode.cn/problems/count-of-substrings-containing-every-vowel-and-k-consonants-ii/
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
//     5 <= word.length <= 2 * 10^5
//     word consists only of lowercase English letters.
//     0 <= k <= word.length - 5
//

struct Solution;

impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        let arr: Vec<char> = word.chars().collect();
        Self::count_at_least_k(&arr, k) - Self::count_at_least_k(&arr, k + 1)
    }

    fn count_at_least_k(arr: &[char], k: i32) -> i64 {
        let mut res = 0;
        let mut count = [0; 26];
        let mut mask = 0;
        let mut consonants = 0;
        let mut l = 0;
        let mut r = 0;

        let vowel_mask = Self::vowel_mask();

        while r < arr.len() {
            count[arr[r] as usize - 'a' as usize] += 1;
            if (vowel_mask & (1 << (arr[r] as usize - 'a' as usize))) != 0 {
                mask |= 1 << (arr[r] as usize - 'a' as usize);
            } else {
                consonants += 1;
            }

            while consonants >= k && mask == vowel_mask {
                res += (arr.len() - r) as i64;
                if (vowel_mask & (1 << (arr[l] as usize - 'a' as usize))) == 0 {
                    consonants -= 1;
                } else {
                    count[arr[l] as usize - 'a' as usize] -= 1;
                    if count[arr[l] as usize - 'a' as usize] == 0 {
                        mask ^= 1 << (arr[l] as usize - 'a' as usize);
                    }
                }
                l += 1;
            }
            r += 1;
        }
        res
    }

    fn vowel_mask() -> usize {
        let mut mask = 0;
        mask |= 1; // a
        mask |= 1 << ('e' as usize - 'a' as usize);
        mask |= 1 << ('i' as usize - 'a' as usize);
        mask |= 1 << ('o' as usize - 'a' as usize);
        mask |= 1 << ('u' as usize - 'a' as usize);
        mask
    }
}

#[test]
fn test() {
    assert_eq!(Solution::count_of_substrings("aeioqq".to_string(), 1), 0);
    assert_eq!(Solution::count_of_substrings("aeiou".to_string(), 0), 1);
    assert_eq!(Solution::count_of_substrings("ieaouqqieaouqq".to_string(), 1), 3);
}
