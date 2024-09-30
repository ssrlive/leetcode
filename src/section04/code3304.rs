#![allow(dead_code)]

// 3304. Find the K-th Character in String Game I
// https://leetcode.com/problems/find-the-k-th-character-in-string-game-i/
// https://leetcode.cn/problems/find-the-k-th-character-in-string-game-i/
//
// Easy
//
// Alice and Bob are playing a game. Initially, Alice has a string word = "a".
//
// You are given a positive integer k.
//
// Now Bob will ask Alice to perform the following operation forever:
//
//     Generate a new string by changing each character in word to its next character in the English alphabet, and append it to the original word.
//
// For example, performing the operation on "c" generates "cd" and performing the operation on "zb" generates "zbac".
//
// Return the value of the kth character in word, after enough operations have been done for word to have at least k characters.
//
// Note that the character 'z' can be changed to 'a' in the operation.
//
// Example 1:
//
// Input: k = 5
//
// Output: "b"
//
// Explanation:
//
// Initially, word = "a". We need to do the operation three times:
//
//     Generated string is "b", word becomes "ab".
//     Generated string is "bc", word becomes "abbc".
//     Generated string is "bccd", word becomes "abbcbccd".
//
// Example 2:
//
// Input: k = 10
//
// Output: "c"
//
// Constraints:
//
//     1 <= k <= 500
//

struct Solution;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let k = k as usize;
        let mut res = "a".to_string();
        while res.len() < k {
            res = res.clone() + &res.chars().map(|c| ((c as u8 - 96) % 26 + 97) as char).collect::<String>();
        }
        res.chars().nth(k - 1).unwrap()
    }
}

#[test]
fn test() {
    assert_eq!(Solution::kth_character(5), 'b');
    assert_eq!(Solution::kth_character(10), 'c');
}
