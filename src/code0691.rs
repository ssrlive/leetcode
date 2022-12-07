#![allow(dead_code)]

// 691. Stickers to Spell Word
// https://leetcode.com/problems/stickers-to-spell-word/
//
// We are given n different types of stickers. Each sticker has a lowercase English word on it.
//
// You would like to spell out the given string target by cutting individual letters from your collection of stickers and rearranging them.
// You can use each sticker more than once if you want, and you have infinite quantities of each sticker.
//
// Return the minimum number of stickers that you need to spell out target. If the task is impossible, return -1.
//
// Note: In all test cases, all words were chosen randomly from the 1000 most common US English words,
// and target was chosen as a concatenation of two random words.
//
// Example 1:
//
// Input: stickers = ["with","example","science"], target = "thehat"
// Output: 3
// Explanation:
// We can use 2 "with" stickers, and 1 "example" sticker.
// After cutting and rearrange the letters of those stickers, we can form the target "thehat".
// Also, this is the minimum number of stickers necessary to form the target string.
//
// Example 2:
//
// Input: stickers = ["notice","possible"], target = "basicbasic"
// Output: -1
// Explanation:
// We cannot form the target "basicbasic" from cutting letters from the given stickers.
//
// Constraints:
//
// - n == stickers.length
// - 1 <= n <= 50
// - 1 <= stickers[i].length <= 10
// - 1 <= target.length <= 15
// - stickers[i] and target consist of lowercase English letters.
//

struct Solution;

impl Solution {
    fn solve(
        stickers: &Vec<String>,
        i: usize,
        target: &String,
        mask: i32,
        the_mask: i32,
        matrix: &mut Vec<Vec<i32>>,
    ) -> Option<i32> {
        if mask == the_mask {
            return Some(0);
        }

        if i == stickers.len() {
            return Some(1 << 30);
        }

        let mut result = matrix[mask as usize][i];

        if result != -1 {
            return Some(result);
        }

        let mut __m = vec![0; 26];

        for x in stickers[i].chars() {
            __m[x as usize - 'a' as usize] += 1;
        }

        let mut over_lap = 0;

        for k in 0..target.len() {
            if mask & (1 << k) != 0 {
                continue;
            }
            if __m[target.chars().nth(k)? as usize - 'a' as usize] != 0 {
                over_lap |= 1 << k;
                __m[target.chars().nth(k)? as usize - 'a' as usize] -= 1;
            }
        }

        if over_lap != 0 {
            result = std::cmp::min(
                1 + Self::solve(stickers, i, target, mask | over_lap, the_mask, matrix)?,
                Self::solve(stickers, i + 1, target, mask, the_mask, matrix)?,
            );
            matrix[mask as usize][i] = result;
            return Some(result);
        }

        result = Self::solve(stickers, i + 1, target, mask, the_mask, matrix)?;
        matrix[mask as usize][i] = result;
        Some(result)
    }

    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let the_mask = (1 << target.len()) - 1;
        let mut matrix = vec![vec![-1; 52]; 32780];
        let ans = Self::solve(&stickers, 0, &target, 0, the_mask, &mut matrix).unwrap_or(1 << 30);
        if ans >= 1 << 30 {
            return -1;
        }
        ans
    }
}

#[test]
fn test() {
    let stickers = vec!["with".to_string(), "example".to_string(), "science".to_string()];
    let target = "thehat".to_string();
    assert_eq!(Solution::min_stickers(stickers, target), 3);

    let stickers = vec!["notice".to_string(), "possible".to_string()];
    let target = "basicbasic".to_string();
    assert_eq!(Solution::min_stickers(stickers, target), -1);

    let stickers = vec![
        "control".to_string(),
        "heart".to_string(),
        "interest".to_string(),
        "stream".to_string(),
        "sentence".to_string(),
        "soil".to_string(),
        "wonder".to_string(),
        "them".to_string(),
        "month".to_string(),
        "slip".to_string(),
        "table".to_string(),
        "miss".to_string(),
        "boat".to_string(),
        "speak".to_string(),
        "figure".to_string(),
        "no".to_string(),
        "perhaps".to_string(),
        "twenty".to_string(),
        "throw".to_string(),
        "rich".to_string(),
        "capital".to_string(),
        "save".to_string(),
        "method".to_string(),
        "store".to_string(),
        "meant".to_string(),
        "life".to_string(),
        "oil".to_string(),
        "string".to_string(),
        "song".to_string(),
        "food".to_string(),
        "am".to_string(),
        "who".to_string(),
        "fat".to_string(),
        "if".to_string(),
        "put".to_string(),
        "path".to_string(),
        "come".to_string(),
        "grow".to_string(),
        "box".to_string(),
        "great".to_string(),
        "word".to_string(),
        "object".to_string(),
        "stead".to_string(),
        "common".to_string(),
        "fresh".to_string(),
        "the".to_string(),
        "operate".to_string(),
        "where".to_string(),
        "road".to_string(),
        "mean".to_string(),
    ];
    let target = "stoodcrease".to_string();
    assert_eq!(Solution::min_stickers(stickers, target), 3);
}
