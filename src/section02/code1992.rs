#![allow(dead_code)]

/*

// 1992. Find All Groups of Farmland
// https://leetcode.com/problems/find-all-groups-of-farmland/
// https://leetcode.cn/problems/find-all-groups-of-farmland/
//
// Medium
//
// You are given a 0-indexed m x n binary matrix land where a 0 represents a hectare of forested land and a 1 represents a hectare of farmland.

To keep the land organized, there are designated rectangular areas of hectares that consist entirely of farmland. These rectangular areas are called groups. No two groups are adjacent, meaning farmland in one group is not four-directionally adjacent to another farmland in a different group.

land can be represented by a coordinate system where the top left corner of land is (0, 0) and the bottom right corner of land is (m-1, n-1). Find the coordinates of the top left and bottom right corner of each group of farmland. A group of farmland with a top left corner at (r1, c1) and a bottom right corner at (r2, c2) is represented by the 4-length array [r1, c1, r2, c2].

Return a 2D array containing the 4-length arrays described above for each group of farmland in land. If there are no groups of farmland, return an empty array. You may return the answer in any order.

Example 1:

Input: land = [[1,0,0],[0,1,1],[0,1,1]]
Output: [[0,0,0,0],[1,1,2,2]]
Explanation:
The first group has a top left corner at land[0][0] and a bottom right corner at land[0][0].
The second group has a top left corner at land[1][1] and a bottom right corner at land[2][2].

Example 2:

Input: land = [[1,1],[1,1]]
Output: [[0,0,1,1]]
Explanation:
The first group has a top left corner at land[0][0] and a bottom right corner at land[1][1].

Example 3:

Input: land = [[0]]
Output: []
Explanation:
There are no groups of farmland.

Constraints:

    m == land.length
    n == land[i].length
    1 <= m, n <= 300
    land consists of only 0's and 1's.
    Groups of farmland are rectangular in shape.
*/

struct Solution;

impl Solution {
    pub fn find_farmland(land: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(land: &[Vec<i32>], visited: &mut Vec<Vec<bool>>, i: usize, j: usize, x: &mut usize, y: &mut usize) {
            if visited[i][j] {
                return;
            }
            visited[i][j] = true;
            *x = i.max(*x);
            *y = j.max(*y);
            if i + 1 < land.len() && land[i + 1][j] == 1 {
                dfs(land, visited, i + 1, j, x, y);
            }
            if j + 1 < land[0].len() && land[i][j + 1] == 1 {
                dfs(land, visited, i, j + 1, x, y);
            }
        }

        let mut ans = Vec::new();
        let mut visited = vec![vec![false; land[0].len()]; land.len()];
        for i in 0..land.len() {
            for j in 0..land[i].len() {
                if visited[i][j] || land[i][j] == 0 {
                    continue;
                }
                let mut x = 0;
                let mut y = 0;
                dfs(&land, &mut visited, i, j, &mut x, &mut y);
                ans.push(vec![i as i32, j as i32, x as i32, y as i32]);
            }
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![1, 0, 0], vec![0, 1, 1], vec![0, 1, 1]],
            vec![vec![0, 0, 0, 0], vec![1, 1, 2, 2]],
        ),
        (vec![vec![1, 1], vec![1, 1]], vec![vec![0, 0, 1, 1]]),
        (vec![vec![0]], vec![]),
    ];
    for (land, expected) in cases {
        assert_eq!(Solution::find_farmland(land), expected);
    }
}
