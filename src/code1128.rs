#![allow(dead_code)]

// 1128. Number of Equivalent Domino Pairs
// https://leetcode.com/problems/number-of-equivalent-domino-pairs/
// https://leetcode.cn/problems/number-of-equivalent-domino-pairs/
//
// Given a list of dominoes, dominoes[i] = [a, b] is equivalent to dominoes[j] = [c, d] if and only if either (a == c and b == d), or (a == d and b == c) - that is, one domino can be rotated to be equal to another domino.
//
// Return the number of pairs (i, j) for which 0 <= i < j < dominoes.length, and dominoes[i] is equivalent to dominoes[j].
//
// Example 1:
//
// Input: dominoes = [[1,2],[2,1],[3,4],[5,6]]
// Output: 1
//
// Example 2:
//
// Input: dominoes = [[1,2],[1,2],[1,1],[1,2],[2,2]]
// Output: 3
//
// Constraints:
//
// - 1 <= dominoes.length <= 4 * 10^4
// - dominoes[i].length == 2
// - 1 <= dominoes[i][j] <= 9
//

struct Solution;

impl Solution {
    pub fn num_equiv_domino_pairs(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut table = [[0; 9]; 9];
        let combinations = |n: i32| -> i32 { (n - 1) * n / 2 };

        dominoes.iter().for_each(|d| {
            let (row, col) = match (d[0] - 1, d[1] - 1) {
                (a, b) if a < b => (a, b),
                (a, b) => (b, a),
            };
            table[row as usize][col as usize] += 1;
        });

        for (start_col, row) in table.iter().enumerate() {
            for &num in row[start_col..].iter().filter(|&&num| num > 1) {
                res += combinations(num);
            }
        }
        res
    }
}

#[test]
fn test() {
    let dominoes = vec![vec![1, 2], vec![2, 1], vec![3, 4], vec![5, 6]];
    assert_eq!(Solution::num_equiv_domino_pairs(dominoes), 1);

    let dominoes = vec![vec![1, 2], vec![1, 2], vec![1, 1], vec![1, 2], vec![2, 2]];
    assert_eq!(Solution::num_equiv_domino_pairs(dominoes), 3);
}
