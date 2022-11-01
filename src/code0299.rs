#![allow(dead_code)]

// 299. Bulls and Cows
// https://leetcode.com/problems/bulls-and-cows/
//
// You are playing the Bulls and Cows game with your friend.
//
// You write down a secret number and ask your friend to guess what the number is.
// When your friend makes a guess, you provide a hint with the following info:
//
// - The number of "bulls", which are digits in the guess that are in the correct position.
// - The number of "cows", which are digits in the guess that are in your secret number but are located in
//   the wrong position. Specifically, the non-bull digits in the guess that could be rearranged such that they become bulls.
//
// Given the secret number secret and your friend's guess guess, return the hint for your friend's guess.
//
// The hint should be formatted as "xAyB", where x is the number of bulls and y is the number of cows.
// Note that both secret and guess may contain duplicate digits.
//
// Example 1:
//
// Input: secret = "1807", guess = "7810"
// Output: "1A3B"
// Explanation: Bulls are connected with a '|' and cows are underlined:
// "1807"
//   |
// "7810"
//
// Example 2:
//
// Input: secret = "1123", guess = "0111"
// Output: "1A1B"
// Explanation: Bulls are connected with a '|' and cows are underlined:
// "1123"        "1123"
//   |      or     |
// "0111"        "0111"
//
// Note that only one of the two unmatched 1s is counted as a cow since the non-bull digits can only
// be rearranged to allow one 1 to be a bull.
//
// Constraints:
//
// 1 <= secret.length, guess.length <= 1000
// secret.length == guess.length
// secret and guess consist of digits only.

struct Solution;

impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bulls = 0;
        let mut cows = 0;
        let mut secret_counts = [0; 10];
        let mut guess_counts = [0; 10];
        for (s, g) in secret.chars().zip(guess.chars()) {
            if s == g {
                bulls += 1;
            } else {
                secret_counts[s as usize - '0' as usize] += 1;
                guess_counts[g as usize - '0' as usize] += 1;
            }
        }
        for i in 0..10 {
            cows += std::cmp::min(secret_counts[i], guess_counts[i]);
        }
        format!("{}A{}B", bulls, cows)
    }
}

#[test]
fn test_get_hint() {
    assert_eq!(
        Solution::get_hint("1807".to_string(), "7810".to_string()),
        "1A3B".to_string()
    );
    assert_eq!(
        Solution::get_hint("1123".to_string(), "0111".to_string()),
        "1A1B".to_string()
    );
}
