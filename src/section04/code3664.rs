#![allow(dead_code)]

// 3664. Two-Letter Card Game
// https://leetcode.com/problems/two-letter-card-game/
// https://leetcode.cn/problems/two-letter-card-game/
//
// Medium
//
// You are given a deck of cards represented by a string array cards, and each card displays two lowercase letters.
//
// You are also given a letter x. You play a game with the following rules:
//
// Start with 0 points.
// On each turn, you must find two compatible cards from the deck that both contain the letter x in any position.
// Remove the pair of cards and earn 1 point.
// The game ends when you can no longer find a pair of compatible cards.
// Return the maximum number of points you can gain with optimal play.
//
// Two cards are compatible if the strings differ in exactly 1 position.
//
// Example 1:
//
// Input: cards = ["aa","ab","ba","ac"], x = "a"
//
// Output: 2
//
// Explanation:
//
// On the first turn, select and remove cards "ab" and "ac", which are compatible because they differ at only index 1.
// On the second turn, select and remove cards "aa" and "ba", which are compatible because they differ at only index 0.
// Because there are no more compatible pairs, the total score is 2.
//
// Example 2:
//
// Input: cards = ["aa","ab","ba"], x = "a"
//
// Output: 1
//
// Explanation:
//
// On the first turn, select and remove cards "aa" and "ba".
// Because there are no more compatible pairs, the total score is 1.
//
// Example 3:
//
// Input: cards = ["aa","ab","ba","ac"], x = "b"
//
// Output: 0
//
// Explanation:
//
// The only cards that contain the character 'b' are "ab" and "ba". However, they differ in both indices, so they are not compatible. Thus, the output is 0.
//
// Constraints:
//
// 2 <= cards.length <= 10^5
// cards[i].length == 2
// Each cards[i] is composed of only lowercase English letters between 'a' and 'j'.
// x is a lowercase English letter between 'a' and 'j'.
//

struct Solution;

impl Solution {
    pub fn score(cards: Vec<String>, x: char) -> i32 {
        let mut ans = 0;
        let mut f = 0;
        let mut tf = 0;
        let mut s = 0;
        let mut ts = 0;
        let mut xx = 0;
        let mut used = 0;
        let mut dict1 = [0; 26];
        let mut dict2 = [0; 26];

        for card in cards {
            let c1 = card.chars().nth(0).unwrap();
            let c2 = card.chars().nth(1).unwrap();

            if c1 == x && c2 == x {
                xx += 1;
            } else if c1 == x {
                dict1[(c2 as u8 - b'a') as usize] += 1;
                f = f.max(dict1[(c2 as u8 - b'a') as usize]);
                tf += 1;
            } else if c2 == x {
                dict2[(c1 as u8 - b'a') as usize] += 1;
                s = s.max(dict2[(c1 as u8 - b'a') as usize]);
                ts += 1;
            }
        }

        if f > (tf - f) {
            ans += tf - f;
            tf = f - (tf - f);
        } else {
            ans += tf / 2;
            tf %= 2;
        }

        if s > (ts - s) {
            ans += ts - s;
            ts = s - (ts - s);
        } else {
            ans += ts / 2;
            ts %= 2;
        }

        used += (tf + ts).min(xx);
        xx -= (tf + ts).min(xx);
        ans = (ans * 2).min(ans + xx / 2);
        ans += used;

        ans
    }
}

#[test]
fn test() {
    let cards = vec!["aa".to_string(), "ab".to_string(), "ba".to_string(), "ac".to_string()];
    let x = 'a';
    assert_eq!(Solution::score(cards, x), 2);

    let cards = vec!["aa".to_string(), "ab".to_string(), "ba".to_string()];
    let x = 'a';
    assert_eq!(Solution::score(cards, x), 1);

    let cards = vec!["aa".to_string(), "ab".to_string(), "ba".to_string(), "ac".to_string()];
    let x = 'b';
    assert_eq!(Solution::score(cards, x), 0);
}
