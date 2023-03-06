#![allow(dead_code)]

/*

// 2209. Minimum White Tiles After Covering With Carpets
// https://leetcode.com/problems/minimum-white-tiles-after-covering-with-carpets/
// https://leetcode.cn/problems/minimum-white-tiles-after-covering-with-carpets/
//
// Hard
//
// You are given a 0-indexed binary string floor, which represents the colors of tiles on a floor:

    floor[i] = '0' denotes that the ith tile of the floor is colored black.
    On the other hand, floor[i] = '1' denotes that the ith tile of the floor is colored white.

You are also given numCarpets and carpetLen. You have numCarpets black carpets, each of length carpetLen tiles. Cover the tiles with the given carpets such that the number of white tiles still visible is minimum. Carpets may overlap one another.

Return the minimum number of white tiles still visible.

Example 1:

Input: floor = "10110101", numCarpets = 2, carpetLen = 2
Output: 2
Explanation:
The figure above shows one way of covering the tiles with the carpets such that only 2 white tiles are visible.
No other way of covering the tiles with the carpets can leave less than 2 white tiles visible.

Example 2:

Input: floor = "11111", numCarpets = 2, carpetLen = 3
Output: 0
Explanation:
The figure above shows one way of covering the tiles with the carpets such that no white tiles are visible.
Note that the carpets are able to overlap one another.

Constraints:

    1 <= carpetLen <= floor.length <= 1000
    floor[i] is either '0' or '1'.
    1 <= numCarpets <= 1000
*/

struct Solution;

impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        let floor = floor.as_bytes();
        let n = floor.len();
        let mut dp = vec![vec![0; num_carpets as usize + 1]; n + 1];
        for i in 1..=n {
            for k in 0..=num_carpets as usize {
                let jump = dp[i - 1][k] + (floor[i - 1] - b'0') as i32;
                let cover = if k > 0 {
                    dp[i.saturating_sub(carpet_len as usize)][k - 1]
                } else {
                    1000
                };
                dp[i][k] = jump.min(cover);
            }
        }
        dp[n][num_carpets as usize]
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        ("10110101", 2, 2, 2),
        ("11111", 2, 3, 0),
    ];
    for (f, n, c, e) in cases {
        assert_eq!(Solution::minimum_white_tiles(f.to_string(), n, c), e);
    }
}
