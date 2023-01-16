#![allow(dead_code)]

// 920. Number of Music Playlists
// https://leetcode.com/problems/number-of-music-playlists/
// https://leetcode.cn/problems/number-of-music-playlists/
//
// Your music player contains n different songs. You want to listen to goal songs (not necessarily different) during your trip.
// To avoid boredom, you will create a playlist so that:
//
// Every song is played at least once.
// A song can only be played again only if k other songs have been played.
// Given n, goal, and k, return the number of possible playlists that you can create. Since the answer can be very large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: n = 3, goal = 3, k = 1
// Output: 6
// Explanation: There are 6 possible playlists: [1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], and [3, 2, 1].
//
// Example 2:
//
// Input: n = 2, goal = 3, k = 0
// Output: 6
// Explanation: There are 6 possible playlists: [1, 1, 2], [1, 2, 1], [2, 1, 1], [2, 2, 1], [2, 1, 2], and [1, 2, 2].
//
// Example 3:
//
// Input: n = 2, goal = 3, k = 1
// Output: 2
// Explanation: There are 2 possible playlists: [1, 2, 1] and [2, 1, 2].
//
// Constraints:
//
// - 0 <= k < n <= goal <= 100
//

struct Solution;

impl Solution {
    pub fn num_music_playlists(n: i32, goal: i32, k: i32) -> i32 {
        const MODULO: u128 = 1000000007;
        let mut num_us: usize;
        let mut dp: Vec<u128> = vec![0; n as usize + 1];
        dp[1] = n as u128;
        for l in 2..goal + 1 {
            num_us = i32::min(n, l) as usize;
            for s in (1..num_us + 1).rev() {
                dp[s] = (dp[s - 1] * (n - s as i32 + 1) as u128 + dp[s] * i32::max(s as i32 - k, 0) as u128) % MODULO;
            }
        }
        dp[n as usize] as i32
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_music_playlists(3, 3, 1), 6);
    assert_eq!(Solution::num_music_playlists(2, 3, 0), 6);
    assert_eq!(Solution::num_music_playlists(2, 3, 1), 2);
    assert_eq!(Solution::num_music_playlists(1, 1, 0), 1);
    assert_eq!(Solution::num_music_playlists(1, 1, 1), 1);
    assert_eq!(Solution::num_music_playlists(1, 2, 1), 0);
    assert_eq!(Solution::num_music_playlists(2, 2, 1), 2);
    assert_eq!(Solution::num_music_playlists(2, 2, 0), 2);
    assert_eq!(Solution::num_music_playlists(3, 2, 1), 0);
}
