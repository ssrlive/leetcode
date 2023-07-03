#![allow(dead_code)]

/*

// 2352. Equal Row and Column Pairs
// https://leetcode.com/problems/equal-row-and-column-pairs/
// https://leetcode.cn/problems/equal-row-and-column-pairs/
//
// Medium
//
// Given a 0-indexed n x n integer matrix grid, return the number of pairs (ri, cj) such that row ri and column cj are equal.

A row and column pair is considered equal if they contain the same elements in the same order (i.e., an equal array).

Example 1:

Input: grid = [[3,2,1],[1,7,6],[2,7,7]]
Output: 1
Explanation: There is 1 equal row and column pair:
- (Row 2, Column 1): [2,7,7]

Example 2:

Input: grid = [[3,1,2,2],[1,4,4,5],[2,4,2,2],[2,4,2,2]]
Output: 3
Explanation: There are 3 equal row and column pairs:
- (Row 0, Column 0): [3,1,2,2]
- (Row 2, Column 2): [2,4,2,2]
- (Row 3, Column 2): [2,4,2,2]

Constraints:

    n == grid.length == grid[i].length
    1 <= n <= 200
    1 <= grid[i][j] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut ret = 0;
        for i in 0..n {
            for j in 0..n {
                let mut valid = true;
                for k in 0..n {
                    if grid[i][k] != grid[k][j] {
                        valid = false;
                    }
                }
                if valid {
                    ret += 1;
                }
            }
        }
        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]], 1),
        (vec![vec![3, 1, 2, 2], vec![1, 4, 4, 5], vec![2, 4, 2, 2], vec![2, 4, 2, 2]], 3),
    ];
    for (grid, expect) in cases {
        assert_eq!(Solution::equal_pairs(grid), expect);
    }
}
