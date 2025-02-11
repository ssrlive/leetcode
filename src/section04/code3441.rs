#![allow(dead_code)]

// 3441. Minimum Cost Good Caption
// https://leetcode.com/problems/minimum-cost-good-caption/
// https://leetcode.cn/problems/minimum-cost-good-caption/
//
// Hard
//
// You are given a string caption of length n. A good caption is a string where every character appears
// in groups of at least 3 consecutive occurrences.
//
// For example:
//
//     "aaabbb" and "aaaaccc" are good captions.
//     "aabbb" and "ccccd" are not good captions.
//
// You can perform the following operation any number of times:
//
// Choose an index i (where 0 <= i < n) and change the character at that index to either:
//
//     The character immediately before it in the alphabet (if caption[i] != 'a').
//     The character immediately after it in the alphabet (if caption[i] != 'z').
//
// Your task is to convert the given caption into a good caption using the minimum number of operations,
// and return it. If there are multiple possible good captions, return the lexicographically smallest
// one among them. If it is impossible to create a good caption, return an empty string "".
//
// Example 1:
//
// Input: caption = "cdcd"
//
// Output: "cccc"
//
// Explanation:
//
// It can be shown that the given caption cannot be transformed into a good caption with fewer than 2 operations.
// The possible good captions that can be created using exactly 2 operations are:
//
//     "dddd": Change caption[0] and caption[2] to their next character 'd'.
//     "cccc": Change caption[1] and caption[3] to their previous character 'c'.
//
// Since "cccc" is lexicographically smaller than "dddd", return "cccc".
//
// Example 2:
//
// Input: caption = "aca"
//
// Output: "aaa"
//
// Explanation:
//
// It can be proven that the given caption requires at least 2 operations to be transformed into a good caption.
// The only good caption that can be obtained with exactly 2 operations is as follows:
//
//     Operation 1: Change caption[1] to 'b'. caption = "aba".
//     Operation 2: Change caption[1] to 'a'. caption = "aaa".
//
// Thus, return "aaa".
//
// Example 3:
//
// Input: caption = "bc"
//
// Output: ""
//
// Explanation:
//
// It can be shown that the given caption cannot be converted to a good caption by using any number of operations.
//
// Constraints:
//
//     1 <= caption.length <= 5 * 10^4
//     caption consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn min_cost_good_caption(caption: String) -> String {
        let caption = caption.as_bytes();
        let mut dp = vec![vec![vec![(0, '~'); 4]; 26]; caption.len() + 1];
        let n = caption.len();
        if n < 3 {
            return "".to_string();
        }
        for last in 0..26 {
            for cnt in 0..=3 {
                dp[n][last][cnt] = if cnt >= 3 { (0, '~') } else { (-1, '~') };
            }
        }
        for i in (0..n).rev() {
            let caption_char = caption[i] - b'a';
            for last in 0..26 {
                for cnt in 0..=3 {
                    let mut best_char = b'~';
                    let mut best_cost = -1;
                    for ch in 0..26 {
                        let change_cost = (caption_char as i32 - ch as i32).abs();
                        if (1..3).contains(&cnt) && ch != last {
                            continue;
                        }
                        let next_cnt = std::cmp::min(3, 1 + (ch == last) as usize * cnt);
                        let res = dp[i + 1][ch][next_cnt];
                        if res.0 == -1 {
                            continue;
                        }
                        if best_cost == -1 || change_cost + res.0 < best_cost {
                            best_cost = change_cost + res.0;
                            best_char = ch as u8;
                        }
                    }
                    dp[i][last][cnt] = (best_cost, best_char as char);
                }
            }
        }
        let mut res = vec![b'0'; n];
        let mut curr = dp[0][0][0];
        let mut cnt = 0;
        for i in 0..n {
            res[i] = curr.1 as u8;
            if i != n - 1 {
                let next_cnt = std::cmp::min(3, 1 + (i > 0 && res[i] == res[i - 1]) as usize * cnt);
                cnt = next_cnt;
                curr = dp[i + 1][res[i] as usize][next_cnt];
            }
        }
        for c in res.iter_mut() {
            *c += b'a';
        }
        String::from_utf8(res).unwrap()
    }
}

#[test]
fn test() {
    let caption = "cdcd".to_string();
    let res = "cccc".to_string();
    assert_eq!(Solution::min_cost_good_caption(caption), res);

    let caption = "aca".to_string();
    let res = "aaa".to_string();
    assert_eq!(Solution::min_cost_good_caption(caption), res);

    let caption = "bc".to_string();
    let res = "".to_string();
    assert_eq!(Solution::min_cost_good_caption(caption), res);
}
