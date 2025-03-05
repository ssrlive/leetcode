#![allow(dead_code)]

/*

// 2146. K Highest Ranked Items Within a Price Range
// https://leetcode.com/problems/k-highest-ranked-items-within-a-price-range/
// https://leetcode.cn/problems/k-highest-ranked-items-within-a-price-range/
//
// Medium
//
// You are given a 0-indexed 2D integer array grid of size m x n that represents a map of the items in a shop. The integers in the grid represent the following:

    0 represents a wall that you cannot pass through.
    1 represents an empty cell that you can freely move to and from.
    All other positive integers represent the price of an item in that cell. You may also freely move to and from these item cells.

It takes 1 step to travel between adjacent grid cells.

You are also given integer arrays pricing and start where pricing = [low, high] and start = [row, col] indicates that you start at the position (row, col) and are interested only in items with a price in the range of [low, high] (inclusive). You are further given an integer k.

You are interested in the positions of the k highest-ranked items whose prices are within the given price range. The rank is determined by the first of these criteria that is different:

    Distance, defined as the length of the shortest path from the start (shorter distance has a higher rank).
    Price (lower price has a higher rank, but it must be in the price range).
    The row number (smaller row number has a higher rank).
    The column number (smaller column number has a higher rank).

Return the k highest-ranked items within the price range sorted by their rank (highest to lowest). If there are fewer than k reachable items within the price range, return all of them.

Example 1:

Input: grid = [[1,2,0,1],[1,3,0,1],[0,2,5,1]], pricing = [2,5], start = [0,0], k = 3
Output: [[0,1],[1,1],[2,1]]
Explanation: You start at (0,0).
With a price range of [2,5], we can take items from (0,1), (1,1), (2,1) and (2,2).
The ranks of these items are:
- (0,1) with distance 1
- (1,1) with distance 2
- (2,1) with distance 3
- (2,2) with distance 4
Thus, the 3 highest ranked items in the price range are (0,1), (1,1), and (2,1).

Example 2:

Input: grid = [[1,2,0,1],[1,3,3,1],[0,2,5,1]], pricing = [2,3], start = [2,3], k = 2
Output: [[2,1],[1,2]]
Explanation: You start at (2,3).
With a price range of [2,3], we can take items from (0,1), (1,1), (1,2) and (2,1).
The ranks of these items are:
- (2,1) with distance 2, price 2
- (1,2) with distance 2, price 3
- (1,1) with distance 3
- (0,1) with distance 4
Thus, the 2 highest ranked items in the price range are (2,1) and (1,2).

Example 3:

Input: grid = [[1,1,1],[0,0,1],[2,3,4]], pricing = [2,3], start = [0,0], k = 3
Output: [[2,1],[2,0]]
Explanation: You start at (0,0).
With a price range of [2,3], we can take items from (2,0) and (2,1).
The ranks of these items are:
- (2,1) with distance 5
- (2,0) with distance 6
Thus, the 2 highest ranked items in the price range are (2,1) and (2,0).
Note that k = 3 but there are only 2 reachable items within the price range.

Constraints:

    m == grid.length
    n == grid[i].length
    1 <= m, n <= 10^5
    1 <= m * n <= 10^5
    0 <= grid[i][j] <= 10^5
    pricing.length == 2
    2 <= low <= high <= 10^5
    start.length == 2
    0 <= row <= m - 1
    0 <= col <= n - 1
    grid[row][col] > 0
    1 <= k <= m * n
*/

struct Solution;

impl Solution {
    pub fn highest_ranked_k_items(grid: Vec<Vec<i32>>, pricing: Vec<i32>, start: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        use std::cmp::Ordering;
        use std::collections::*;

        #[allow(clippy::too_many_arguments)]
        fn helper(
            grid: &[Vec<i32>],
            memo: &mut [Vec<i32>],
            memo2: &mut HashMap<(usize, usize), (i32, i32)>,
            stack: &mut Vec<(usize, usize, i32)>,
            r: usize,
            c: usize,
            lv: i32,
            rv: i32,
            cv: i32,
        ) {
            let nv = cv + 1;
            let gv = grid[r][c];
            if gv >= 1 && nv < memo[r][c] {
                memo[r][c] = nv;
                stack.push((r, c, nv));

                if lv <= gv && gv <= rv {
                    memo2.insert((r, c), (nv, gv));
                }
            }
        }

        let n = grid.len();
        let m = grid[0].len();
        let sri = start[0] as usize;
        let sci = start[1] as usize;
        let inf = 1_000_000_000;
        let mut memo = vec![vec![inf; m]; n];
        memo[sri][sci] = 0;
        let mut memo2 = HashMap::new();
        let mut stack = vec![(sri, sci, 0)];
        let lv = pricing[0];
        let rv = pricing[1];

        if lv <= grid[sri][sci] && grid[sri][sci] <= rv {
            memo2.insert((sri, sci), (0, grid[sri][sci]));
        }

        while !stack.is_empty() {
            let mut new_stack = vec![];
            while let Some((r, c, v)) = stack.pop() {
                if 0 < r {
                    helper(&grid, &mut memo, &mut memo2, &mut new_stack, r - 1, c, lv, rv, v);
                }
                if r < n - 1 {
                    helper(&grid, &mut memo, &mut memo2, &mut new_stack, r + 1, c, lv, rv, v);
                }
                if 0 < c {
                    helper(&grid, &mut memo, &mut memo2, &mut new_stack, r, c - 1, lv, rv, v);
                }
                if c < m - 1 {
                    helper(&grid, &mut memo, &mut memo2, &mut new_stack, r, c + 1, lv, rv, v);
                }
            }
            stack = new_stack;
        }

        let mut memo2 = memo2
            .into_iter()
            .map(|(key, val)| (val.0, val.1, key.0, key.1))
            .collect::<Vec<(i32, i32, usize, usize)>>();
        memo2.sort_by(|a, b| {
            let v1 = a.0.cmp(&b.0);
            if v1 == Ordering::Equal {
                let v2 = a.1.cmp(&b.1);
                if v2 == Ordering::Equal {
                    let v3 = a.2.cmp(&b.2);
                    if v3 == Ordering::Equal { a.3.cmp(&b.3) } else { v3 }
                } else {
                    v2
                }
            } else {
                v1
            }
        });
        let mut result = vec![];
        for i in 0..k.min(memo2.len() as i32) {
            result.push(vec![memo2[i as usize].2 as i32, memo2[i as usize].3 as i32]);
        }
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![1, 2, 0, 1], vec![1, 3, 3, 1], vec![0, 2, 5, 1]],
            vec![2, 3],
            vec![2, 3],
            2,
            vec![vec![2, 1], vec![1, 2]],
        ),
        (
            vec![vec![1, 1, 1], vec![0, 0, 1], vec![2, 3, 4]],
            vec![2, 3],
            vec![0, 0],
            3,
            vec![vec![2, 1], vec![2, 0]],
        ),
    ];
    for (grid, pricing, start, k, expect) in cases {
        assert_eq!(Solution::highest_ranked_k_items(grid, pricing, start, k), expect);
    }
}
