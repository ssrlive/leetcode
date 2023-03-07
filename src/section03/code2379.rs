#![allow(dead_code)]

/*

// 2379. Minimum Recolors to Get K Consecutive Black Blocks
// https://leetcode.com/problems/minimum-recolors-to-get-k-consecutive-black-blocks/
// https://leetcode.cn/problems/minimum-recolors-to-get-k-consecutive-black-blocks/
//
// Easy
//
// You are given a 0-indexed string blocks of length n, where blocks[i] is either 'W' or 'B', representing the color of the ith block. The characters 'W' and 'B' denote the colors white and black, respectively.

You are also given an integer k, which is the desired number of consecutive black blocks.

In one operation, you can recolor a white block such that it becomes a black block.

Return the minimum number of operations needed such that there is at least one occurrence of k consecutive black blocks.

Example 1:

Input: blocks = "WBBWWBBWBW", k = 7
Output: 3
Explanation:
One way to achieve 7 consecutive black blocks is to recolor the 0th, 3rd, and 4th blocks
so that blocks = "BBBBBBBWBW".
It can be shown that there is no way to achieve 7 consecutive black blocks in less than 3 operations.
Therefore, we return 3.

Example 2:

Input: blocks = "WBWBBBW", k = 2
Output: 0
Explanation:
No changes need to be made, since 2 consecutive black blocks already exist.
Therefore, we return 0.

Constraints:

    n == blocks.length
    1 <= n <= 100
    blocks[i] is either 'W' or 'B'.
    1 <= k <= n
*/

struct Solution;

impl Solution {
    pub fn minimum_recolors(blocks: String, k: i32) -> i32 {
        let idx = |b: u8| -> usize { (b > b'B') as _ };
        let mut counter = [0; 2];
        let mut res = i32::MAX;
        let bb = blocks.as_bytes();
        let ku = k as usize;
        for (i, w) in bb.windows(ku).enumerate() {
            match i > 0 {
                true => {
                    counter[idx(w[ku - 1])] += 1;
                    counter[idx(bb[i - 1])] -= 1;
                }
                false => w.iter().for_each(|&b| counter[idx(b)] += 1),
            };
            match counter[1] {
                0 => return 0,
                n => res = res.min(n),
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        ("WBBWWBBWBW", 7, 3),
        ("WBWBBBW", 2, 0),
        ("WBWBWBBWBWBBBWBWBWBBWBWBBBWBWBWBBWBWBBB", 3, 0),
    ];
    for (blocks, k, expected) in cases {
        assert_eq!(Solution::minimum_recolors(blocks.to_string(), k), expected);
    }
}
