#![allow(dead_code)]

// 947. Most Stones Removed with Same Row or Column
// https://leetcode.com/problems/most-stones-removed-with-same-row-or-column/
// https://leetcode.cn/problems/most-stones-removed-with-same-row-or-column/
//
// On a 2D plane, we place n stones at some integer coordinate points. Each coordinate point may have at most one stone.
//
// A stone can be removed if it shares either the same row or the same column as another stone that has not been removed.
//
// Given an array stones of length n where stones[i] = [xi, yi] represents the location of the ith stone,
// return the largest possible number of stones that can be removed.
//
// Example 1:
//
// Input: stones = [[0,0],[0,1],[1,0],[1,2],[2,1],[2,2]]
// Output: 5
// Explanation: One way to remove 5 stones is as follows:
// 1. Remove stone [2,2] because it shares the same row as [2,1].
// 2. Remove stone [2,1] because it shares the same column as [0,1].
// 3. Remove stone [1,2] because it shares the same row as [1,0].
// 4. Remove stone [1,0] because it shares the same column as [0,0].
// 5. Remove stone [0,1] because it shares the same row as [0,0].
// Stone [0,0] cannot be removed since it does not share a row/column with another stone still on the plane.
//
// Example 2:
//
// Input: stones = [[0,0],[0,2],[1,1],[2,0],[2,2]]
// Output: 3
// Explanation: One way to make 3 moves is as follows:
// 1. Remove stone [2,2] because it shares the same row as [2,0].
// 2. Remove stone [2,0] because it shares the same column as [0,0].
// 3. Remove stone [0,2] because it shares the same row as [0,0].
// Stones [0,0] and [1,1] cannot be removed since they do not share a row/column with another stone still on the plane.
//
// Example 3:
//
// Input: stones = [[0,0]]
// Output: 0
// Explanation: [0,0] is the only stone on the plane, so you cannot remove it.
//
// Constraints:
//
// - 1 <= stones.length <= 1000
// - 0 <= xi, yi <= 10^4
// - No two stones are at the same coordinate point.
//

struct Solution;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashMap, HashSet};

        fn stone_dfs(
            i: usize,
            rows: &mut HashMap<i32, HashSet<usize>>,
            cols: &mut HashMap<i32, HashSet<usize>>,
            stones: &Vec<Vec<i32>>,
        ) -> i32 {
            rows.get_mut(&stones[i][0]).unwrap().remove(&i);
            cols.get_mut(&stones[i][1]).unwrap().remove(&i);

            let mut total = 1;
            while let Some(&j) = rows.get(&stones[i][0]).unwrap().iter().next() {
                total += stone_dfs(j, rows, cols, stones);
            }
            while let Some(&j) = cols.get(&stones[i][1]).unwrap().iter().next() {
                total += stone_dfs(j, rows, cols, stones);
            }
            total
        }

        let mut rows: HashMap<i32, HashSet<usize>> = HashMap::new();
        let mut cols: HashMap<i32, HashSet<usize>> = HashMap::new();
        for (i, v) in stones.iter().enumerate() {
            rows.entry(v[0]).or_default().insert(i);
            cols.entry(v[1]).or_default().insert(i);
        }
        let mut ans = 0;
        for i in 0..stones.len() {
            if rows.get(&stones[i][0]).unwrap().contains(&i) {
                ans += stone_dfs(i, &mut rows, &mut cols, &stones) - 1;
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1], vec![2, 2]], 5),
        (vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]], 3),
        (vec![vec![0, 0]], 0),
    ];
    for (stones, expected) in cases {
        assert_eq!(Solution::remove_stones(stones), expected);
    }
}
