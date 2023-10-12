#![allow(dead_code)]

/*

// 2392. Build a Matrix With Conditions
// https://leetcode.com/problems/build-a-matrix-with-conditions/
// https://leetcode.cn/problems/build-a-matrix-with-conditions/
//
// Hard
//
// You are given a positive integer k. You are also given:

    a 2D integer array rowConditions of size n where rowConditions[i] = [abovei, belowi], and
    a 2D integer array colConditions of size m where colConditions[i] = [lefti, righti].

The two arrays contain integers from 1 to k.

You have to build a k x k matrix that contains each of the numbers from 1 to k exactly once. The remaining cells should have the value 0.

The matrix should also satisfy the following conditions:

    The number abovei should appear in a row that is strictly above the row at which the number belowi appears for all i from 0 to n - 1.
    The number lefti should appear in a column that is strictly left of the column at which the number righti appears for all i from 0 to m - 1.

Return any matrix that satisfies the conditions. If no answer exists, return an empty matrix.

Example 1:

Input: k = 3, rowConditions = [[1,2],[3,2]], colConditions = [[2,1],[3,2]]
Output: [[3,0,0],[0,0,1],[0,2,0]]
Explanation: The diagram above shows a valid example of a matrix that satisfies all the conditions.
The row conditions are the following:
- Number 1 is in row 1, and number 2 is in row 2, so 1 is above 2 in the matrix.
- Number 3 is in row 0, and number 2 is in row 2, so 3 is above 2 in the matrix.
The column conditions are the following:
- Number 2 is in column 1, and number 1 is in column 2, so 2 is left of 1 in the matrix.
- Number 3 is in column 0, and number 2 is in column 1, so 3 is left of 2 in the matrix.
Note that there may be multiple correct answers.

Example 2:

Input: k = 3, rowConditions = [[1,2],[2,3],[3,1],[2,3]], colConditions = [[2,1]]
Output: []
Explanation: From the first two conditions, 3 has to be below 1 but the third conditions needs 3 to be above 1 to be satisfied.
No matrix can satisfy all the conditions, so we return the empty matrix.

Constraints:

    2 <= k <= 400
    1 <= rowConditions.length, colConditions.length <= 10^4
    rowConditions[i].length == colConditions[i].length == 2
    1 <= abovei, belowi, lefti, righti <= k
    abovei != belowi
    lefti != righti
*/

struct Solution;

impl Solution {
    pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn find_order(k: i32, conditions: &Vec<Vec<i32>>) -> Vec<i32> {
            let n = k as usize;
            let mut graph = vec![vec![]; n];
            let mut in_degree = vec![0i32; n];
            for p in conditions {
                let (u, v) = (p[0] as usize - 1, p[1] as usize - 1);
                in_degree[u] += 1;
                graph[v].push(u);
            }
            let (mut sk, mut ret) = (vec![], vec![]);
            for (u, &in_degree_u) in in_degree.iter().enumerate() {
                if in_degree_u > 0 {
                    continue;
                }
                ret.push(u as i32 + 1);
                sk.push(u);
            }
            while let Some(u) = sk.pop() {
                for v in &graph[u] {
                    in_degree[*v] -= 1;
                    if in_degree[*v] > 0 {
                        continue;
                    }
                    sk.push(*v);
                    ret.push(*v as i32 + 1);
                }
            }
            if ret.len() < n {
                return vec![];
            }
            ret.reverse();
            ret
        }

        let row_order = find_order(k, &row_conditions);
        let col_order = find_order(k, &col_conditions);
        if row_order.is_empty() || col_order.is_empty() {
            return Vec::new();
        }
        let k = k as usize;
        let mut ret = vec![vec![0; k]; k];
        for i in 0..k {
            for (j, &col_order_j) in col_order.iter().enumerate().take(k) {
                if row_order[i] == col_order_j {
                    ret[i][j] = row_order[i];
                }
            }
        }
        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            3,
            vec![vec![1, 2], vec![3, 2]],
            vec![vec![2, 1], vec![3, 2]],
            vec![vec![3, 0, 0], vec![0, 0, 1], vec![0, 2, 0]],
        ),
        (3, vec![vec![1, 2], vec![2, 3], vec![3, 1], vec![2, 3]], vec![vec![2, 1]], vec![]),
    ];
    for (k, row_conditions, col_conditions, expect) in cases {
        assert_eq!(Solution::build_matrix(k, row_conditions, col_conditions), expect);
    }
}
