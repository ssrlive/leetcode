#![allow(dead_code)]

/*

// 2271. Maximum White Tiles Covered by a Carpet
// https://leetcode.com/problems/maximum-white-tiles-covered-by-a-carpet/
// https://leetcode.cn/problems/maximum-white-tiles-covered-by-a-carpet/
//
// Medium
//
// You are given a 2D integer array tiles where tiles[i] = [li, ri] represents that every tile j in the range li <= j <= ri is colored white.

You are also given an integer carpetLen, the length of a single carpet that can be placed anywhere.

Return the maximum number of white tiles that can be covered by the carpet.

Example 1:

Input: tiles = [[1,5],[10,11],[12,18],[20,25],[30,32]], carpetLen = 10
Output: 9
Explanation: Place the carpet starting on tile 10.
It covers 9 white tiles, so we return 9.
Note that there may be other places where the carpet covers 9 white tiles.
It can be shown that the carpet cannot cover more than 9 white tiles.

Example 2:

Input: tiles = [[10,11],[1,1]], carpetLen = 2
Output: 2
Explanation: Place the carpet starting on tile 10.
It covers 2 white tiles, so we return 2.

Constraints:

    1 <= tiles.length <= 5 * 10^4
    tiles[i].length == 2
    1 <= li <= ri <= 10^9
    1 <= carpetLen <= 10^9
    The tiles are non-overlapping.
*/

struct Solution;

impl Solution {
    pub fn maximum_white_tiles(tiles: Vec<Vec<i32>>, carpet_len: i32) -> i32 {
        let mut res = 0;
        let mut j = 0;
        let mut cover = 0;
        let mut tiles_ = tiles;
        tiles_.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut i = 0;
        while i < tiles_.len() && res < carpet_len {
            let start = tiles_[i][0];
            let end = tiles_[i][1];
            if j == i || tiles_[j][0] + carpet_len > end {
                cover += carpet_len.min(end - start + 1);
                res = res.max(cover);
                i += 1;
            } else {
                let partial = 0.max(tiles_[j][0] + carpet_len - tiles_[i][0]);
                res = res.max(cover + partial);
                cover -= tiles_[j][1] - tiles_[j][0] + 1;
                j += 1;
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![1, 5], vec![10, 11], vec![12, 18], vec![20, 25], vec![30, 32]],
            10,
            9,
        ),
        (vec![vec![10, 11], vec![1, 1]], 2, 2),
    ];
    for (tiles, carpet_len, expected) in cases {
        assert_eq!(Solution::maximum_white_tiles(tiles, carpet_len), expected);
    }
}
