#![allow(dead_code)]

// 1042. Flower Planting With No Adjacent
// https://leetcode.com/problems/flower-planting-with-no-adjacent/
// https://leetcode.cn/problems/flower-planting-with-no-adjacent/
//
// You have n gardens, labeled from 1 to n, and an array paths where paths[i] = [xi, yi] describes a bidirectional path between garden xi to garden yi. In each garden, you want to plant one of 4 types of flowers.
//
// All gardens have at most 3 paths coming into or leaving it.
//
// Your task is to choose a flower type for each garden such that, for any two gardens connected by a path, they have different types of flowers.
//
// Return any such a choice as an array answer, where answer[i] is the type of flower planted in the (i+1)th garden. The flower types are denoted 1, 2, 3, or 4. It is guaranteed an answer exists.
//
// Example 1:
//
// Input: n = 3, paths = [[1,2],[2,3],[3,1]]
// Output: [1,2,3]
// Explanation:
// Gardens 1 and 2 have different types.
// Gardens 2 and 3 have different types.
// Gardens 3 and 1 have different types.
// Hence, [1,2,3] is a valid answer. Other valid answers include [1,2,4], [1,4,2], and [3,2,1].
//
// Example 2:
//
// Input: n = 4, paths = [[1,2],[3,4]]
// Output: [1,2,1,2]
//
// Example 3:
//
// Input: n = 4, paths = [[1,2],[2,3],[3,4],[4,1],[1,3],[2,4]]
// Output: [1,2,3,4]
//
// Constraints:
//
// - 1 <= n <= 10^4
// - 0 <= paths.length <= 2 * 10^4
// - paths[i].length == 2
// - 1 <= xi, yi <= n
// - xi != yi
// - Every garden has at most 3 paths coming into or leaving it.
//

struct Solution;

impl Solution {
    pub fn garden_no_adj(n: i32, paths: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n];
        for path in paths {
            let (x, y) = (path[0] as usize - 1, path[1] as usize - 1);
            graph[x].push(y);
            graph[y].push(x);
        }
        let mut colors = vec![0; n];
        for i in 0..n {
            let mut used = [false; 5];
            for &j in &graph[i] {
                used[colors[j] as usize] = true;
            }
            for (c, &item) in used.iter().enumerate().skip(1) {
                if !item {
                    colors[i] = c as i32;
                    break;
                }
            }
        }
        colors
    }
}

#[test]
fn test() {
    let cases = vec![
        (3, vec![vec![1, 2], vec![2, 3], vec![3, 1]], vec![1, 2, 3]),
        (4, vec![vec![1, 2], vec![3, 4]], vec![1, 2, 1, 2]),
        (
            4,
            vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![1, 3], vec![2, 4]],
            vec![1, 2, 3, 4],
        ),
    ];
    for (n, paths, expected) in cases {
        assert_eq!(Solution::garden_no_adj(n, paths), expected);
    }
}
