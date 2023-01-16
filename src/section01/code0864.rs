#![allow(dead_code)]

// 864. Shortest Path to Get All Keys
// https://leetcode.com/problems/shortest-path-to-get-all-keys/
// https://leetcode.cn/problems/shortest-path-to-get-all-keys/
//
// You are given an m x n grid grid where:
//
// '.' is an empty cell.
// '#' is a wall.
// '@' is the starting point.
// Lowercase letters represent keys.
// Uppercase letters represent locks.
// You start at the starting point and one move consists of walking one space in one of the four cardinal directions. You cannot walk outside the grid, or walk into a wall.
//
// If you walk over a key, you can pick it up and you cannot walk over a lock unless you have its corresponding key.
//
// For some 1 <= k <= 6, there is exactly one lowercase and one uppercase letter of the first k letters of the English alphabet in the grid.
// This means that there is exactly one key for each lock, and one lock for each key;
// and also that the letters used to represent the keys and locks were chosen in the same order as the English alphabet.
//
// Return the lowest number of moves to acquire all keys. If it is impossible, return -1.
//
// Example 1:
//
// Input: grid = ["@.a..","###.#","b.A.B"]
// Output: 8
// Explanation: Note that the goal is to obtain all the keys not to open all the locks.
//
// Example 2:
//
// Input: grid = ["@..aA","..B#.","....b"]
// Output: 6
//
// Example 3:
//
// Input: grid = ["@Aa"]
// Output: -1
//
// Constraints:
//
// - m == grid.length
// - n == grid[i].length
// - 1 <= m, n <= 30
// - grid[i][j] is either an English letter, '.', '#', or '@'.
// - The number of keys in the grid is in the range [1, 6].
// - Each key in the grid is unique.
// - Each key in the grid has a matching lock.
//

struct Solution;

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        use std::collections::HashSet;
        let (mut q, _n, _m, mut ans, mut k) = (HashSet::new(), grid.len(), grid[0].len(), 0, 0);
        let (mut g, mut f) = ([[128; 32]; 32], [[[1000; 32]; 32]; 64]);
        for (i, x) in grid.iter().enumerate() {
            for (j, y) in x.chars().enumerate() {
                match y as u8 {
                    46 => {
                        g[i + 1][j + 1] = 0;
                    }
                    64 => {
                        g[i + 1][j + 1] = 0;
                        q.insert((i as u8 + 1, j as u8 + 1, 0u8));
                        f[0][i + 1][j + 1] = 0;
                    }
                    65..=71 => {
                        g[i + 1][j + 1] = 1 << (y as u32 - 65);
                    }
                    97..=103 => {
                        g[i + 1][j + 1] = 256 << (y as u32 - 97);
                        k += 1;
                    }
                    _ => {}
                }
            }
        }
        while !q.is_empty() {
            let mut tmp = HashSet::new();
            ans += 1;
            for (i, j, s) in q.into_iter().map(|x| (x.0 as usize, x.1 as usize, x.2 as usize)) {
                let ss: usize = s | (g[i + 1][j] >> 8);
                if ss.count_ones() as i32 == k {
                    return ans;
                }
                if g[i + 1][j] > 128 && f[ss][i + 1][j] > ans {
                    f[ss][i + 1][j] = ans;
                    tmp.insert((i as u8 + 1, j as u8, ss as u8));
                }
                if g[i + 1][j] < 256 && g[i + 1][j] & s == g[i + 1][j] && f[s][i + 1][j] > ans {
                    f[s][i + 1][j] = ans;
                    tmp.insert((i as u8 + 1, j as u8, s as u8));
                }
                let ss: usize = s | (g[i - 1][j] >> 8);
                if ss.count_ones() as i32 == k {
                    return ans;
                }
                if g[i - 1][j] > 128 && f[ss][i - 1][j] > ans {
                    f[ss][i - 1][j] = ans;
                    tmp.insert((i as u8 - 1, j as u8, ss as u8));
                }
                if g[i - 1][j] < 256 && g[i - 1][j] & s == g[i - 1][j] && f[s][i - 1][j] > ans {
                    f[s][i - 1][j] = ans;
                    tmp.insert((i as u8 - 1, j as u8, s as u8));
                }
                let ss: usize = s | (g[i][j + 1] >> 8);
                if ss.count_ones() as i32 == k {
                    return ans;
                }
                if g[i][j + 1] > 128 && f[ss][i][j + 1] > ans {
                    f[ss][i][j + 1] = ans;
                    tmp.insert((i as u8, j as u8 + 1, ss as u8));
                }
                if g[i][j + 1] < 256 && g[i][j + 1] & s == g[i][j + 1] && f[s][i][j + 1] > ans {
                    f[s][i][j + 1] = ans;
                    tmp.insert((i as u8, j as u8 + 1, s as u8));
                }
                let ss: usize = s | (g[i][j - 1] >> 8);
                if ss.count_ones() as i32 == k {
                    return ans;
                }
                if g[i][j - 1] > 128 && f[ss][i][j - 1] > ans {
                    f[ss][i][j - 1] = ans;
                    tmp.insert((i as u8, j as u8 - 1, ss as u8));
                }
                if g[i][j - 1] < 256 && g[i][j - 1] & s == g[i][j - 1] && f[s][i][j - 1] > ans {
                    f[s][i][j - 1] = ans;
                    tmp.insert((i as u8, j as u8 - 1, s as u8));
                }
            }
            q = tmp;
        }
        -1
    }
}

#[test]
fn test() {
    let grid = vec!["@.a..".to_string(), "###.#".to_string(), "b.A.B".to_string()];
    let res = 8;
    assert_eq!(Solution::shortest_path_all_keys(grid), res);

    let grid = vec!["@..aA".to_string(), "..B#.".to_string(), "....b".to_string()];
    let res = 6;
    assert_eq!(Solution::shortest_path_all_keys(grid), res);

    let grid = vec!["@Aa".to_string()];
    let res = -1;
    assert_eq!(Solution::shortest_path_all_keys(grid), res);
}
