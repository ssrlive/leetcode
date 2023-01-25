#![allow(dead_code)]

// 1320. Minimum Distance to Type a Word Using Two Fingers
// https://leetcode.com/problems/minimum-distance-to-type-a-word-using-two-fingers/
// https://leetcode.cn/problems/minimum-distance-to-type-a-word-using-two-fingers/
//
// Hard
//
// You have a keyboard layout as shown above in the X-Y plane, where each English uppercase letter is located at some coordinate.
//
// For example, the letter 'A' is located at coordinate (0, 0), the letter 'B' is located at coordinate (0, 1),
// the letter 'P' is located at coordinate (2, 3) and the letter 'Z' is located at coordinate (4, 1).
//
// Given the string word, return the minimum total distance to type such string using only two fingers.
//
// The distance between coordinates (x1, y1) and (x2, y2) is |x1 - x2| + |y1 - y2|.
//
// Note that the initial positions of your two fingers are considered free so do not count towards your total distance,
// also your two fingers do not have to start at the first letter or the first two letters.
//
// Example 1:
//
// Input: word = "CAKE"
// Output: 3
// Explanation: Using two fingers, one optimal way to type "CAKE" is:
// Finger 1 on letter 'C' -> cost = 0
// Finger 1 on letter 'A' -> cost = Distance from letter 'C' to letter 'A' = 2
// Finger 2 on letter 'K' -> cost = 0
// Finger 2 on letter 'E' -> cost = Distance from letter 'K' to letter 'E' = 1
// Total distance = 3
//
// Example 2:
//
// Input: word = "HAPPY"
// Output: 6
// Explanation: Using two fingers, one optimal way to type "HAPPY" is:
// Finger 1 on letter 'H' -> cost = 0
// Finger 1 on letter 'A' -> cost = Distance from letter 'H' to letter 'A' = 2
// Finger 2 on letter 'P' -> cost = 0
// Finger 2 on letter 'P' -> cost = Distance from letter 'P' to letter 'P' = 0
// Finger 1 on letter 'Y' -> cost = Distance from letter 'A' to letter 'Y' = 4
// Total distance = 6
//

struct Solution;

impl Solution {
    pub fn minimum_distance(word: String) -> i32 {
        fn dist(a: i32, b: i32) -> i32 {
            (a / 6 - b / 6).abs() + (a % 6 - b % 6).abs()
        }

        let mut dp = vec![0; 26];
        let mut res = 0;
        let mut save = 0;
        let n = word.len();
        for i in 0..n - 1 {
            let b = (word.as_bytes()[i] - b'A') as i32;
            let c = (word.as_bytes()[i + 1] - b'A') as i32;
            for a in 0..26 {
                dp[b as usize] = dp[b as usize].max(dp[a as usize] + dist(b, c) - dist(a, c));
            }
            save = save.max(dp[b as usize]);
            res += dist(b, c);
        }
        res - save
    }
}

#[test]
fn test() {
    assert_eq!(Solution::minimum_distance("CAKE".to_string()), 3);
    assert_eq!(Solution::minimum_distance("HAPPY".to_string()), 6);
}
