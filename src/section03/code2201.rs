#![allow(dead_code)]

/*

// 2201. Count Artifacts That Can Be Extracted
// https://leetcode.com/problems/count-artifacts-that-can-be-extracted/
// https://leetcode.cn/problems/count-artifacts-that-can-be-extracted/
//
// Medium
//
// There is an n x n 0-indexed grid with some artifacts buried in it. You are given the integer n and a 0-indexed 2D integer array artifacts describing the positions of the rectangular artifacts where artifacts[i] = [r1i, c1i, r2i, c2i] denotes that the ith artifact is buried in the subgrid where:

    (r1i, c1i) is the coordinate of the top-left cell of the ith artifact and
    (r2i, c2i) is the coordinate of the bottom-right cell of the ith artifact.

You will excavate some cells of the grid and remove all the mud from them. If the cell has a part of an artifact buried underneath, it will be uncovered. If all the parts of an artifact are uncovered, you can extract it.

Given a 0-indexed 2D integer array dig where dig[i] = [ri, ci] indicates that you will excavate the cell (ri, ci), return the number of artifacts that you can extract.

The test cases are generated such that:

    No two artifacts overlap.
    Each artifact only covers at most 4 cells.
    The entries of dig are unique.

Example 1:

Input: n = 2, artifacts = [[0,0,0,0],[0,1,1,1]], dig = [[0,0],[0,1]]
Output: 1
Explanation:
The different colors represent different artifacts. Excavated cells are labeled with a 'D' in the grid.
There is 1 artifact that can be extracted, namely the red artifact.
The blue artifact has one part in cell (1,1) which remains uncovered, so we cannot extract it.
Thus, we return 1.

Example 2:

Input: n = 2, artifacts = [[0,0,0,0],[0,1,1,1]], dig = [[0,0],[0,1],[1,1]]
Output: 2
Explanation: Both the red and blue artifacts have all parts uncovered (labeled with a 'D') and can be extracted, so we return 2.

Constraints:

    1 <= n <= 1000
    1 <= artifacts.length, dig.length <= min(n2, 10^5)
    artifacts[i].length == 4
    dig[i].length == 2
    0 <= r1i, c1i, r2i, c2i, ri, ci <= n - 1
    r1i <= r2i
    c1i <= c2i
    No two artifacts will overlap.
    The number of cells covered by an artifact is at most 4.
    The entries of dig are unique.
*/

struct Solution;

impl Solution {
    pub fn dig_artifacts(_n: i32, artifacts: Vec<Vec<i32>>, dig: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        let mut ans = 0;
        let mut xs: HashSet<(i32, i32)> = HashSet::new();
        for v1 in dig {
            let x = v1[0];
            let y = v1[1];
            xs.insert((x, y));
        }
        for v in artifacts {
            let (r1, c1, r2, c2) = (v[0], v[1], v[2], v[3]);
            let mut covered = true;
            'area_loop: for r in r1..=r2 {
                for c in c1..=c2 {
                    if !xs.contains(&(r, c)) {
                        covered = false;
                        break 'area_loop;
                    }
                }
            }
            if covered {
                ans += 1;
            }
        }
        ans
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (2, vec![vec![0, 0, 0, 0], vec![0, 1, 1, 1]], vec![vec![0, 0], vec![0, 1]], 1),
        (2, vec![vec![0, 0, 0, 0], vec![0, 1, 1, 1]], vec![vec![0, 0], vec![0, 1], vec![1, 1]], 2),
    ];
    for (n, artifacts, dig, expect) in cases {
        assert_eq!(Solution::dig_artifacts(n, artifacts, dig), expect);
    }
}
