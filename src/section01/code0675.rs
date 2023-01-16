#![allow(dead_code)]

// 675. Cut Off Trees for Golf Event
// https://leetcode.com/problems/cut-off-trees-for-golf-event/
// https://leetcode.cn/problems/cut-off-trees-for-golf-event/
//
// You are asked to cut off all the trees in a forest for a golf event. The forest is represented as an m x n matrix. In this matrix:
//
// - 0 means the cell cannot be walked through.
// - 1 represents an empty cell that can be walked through.
// - A number greater than 1 represents a tree in a cell that can be walked through, and this number is the tree's height.
//
// In one step, you can walk in any of the four directions: north, east, south, and west. If you are standing in a cell with a tree,
// you can choose whether to cut it off.
//
// You must cut off the trees in order from shortest to tallest. When you cut off a tree, the value at its cell becomes 1 (an empty cell).
//
// Starting from the point (0, 0), return the minimum steps you need to walk to cut off all the trees. If you cannot cut off all the trees, return -1.
//
// Note: The input is generated such that no two trees have the same height, and there is at least one tree needs to be cut off.
//
// Example 1:
//
// Input: forest = [[1,2,3],[0,0,4],[7,6,5]]
// Output: 6
// Explanation: Following the path above allows you to cut off the trees from shortest to tallest in 6 steps.
//
// Example 2:
//
// Input: forest = [[1,2,3],[0,0,0],[7,6,5]]
// Output: -1
// Explanation: The trees in the bottom row cannot be accessed as the middle row is blocked.
//
// Example 3:
//
// Input: forest = [[2,3,4],[0,0,5],[8,7,6]]
// Output: 6
// Explanation: You can follow the same path as Example 1 to cut off all the trees.
// Note that you can cut off the first tree at (0, 0) before making any steps.
//
// Constraints:
//
// - m == forest.length
// - n == forest[i].length
// - 1 <= m, n <= 50
// - 0 <= forest[i][j] <= 10^9
// - Heights of all trees are distinct.
//

struct Solution;

impl Solution {
    pub fn cut_off_tree(forest: Vec<Vec<i32>>) -> i32 {
        let mut trees = forest
            .iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, &height)| if height > 1 { Some((height, i, j)) } else { None })
            })
            .collect::<Vec<_>>();
        trees.sort_unstable();

        let mut steps = 0;
        let mut start = (0, 0);
        for (_, i, j) in trees {
            let step = Self::bfs(&forest, start, (i, j));
            if step < 0 {
                return -1;
            }
            steps += step;
            start = (i, j);
        }
        steps
    }

    fn bfs(forest: &[Vec<i32>], start: (usize, usize), end: (usize, usize)) -> i32 {
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((start, 0));
        let mut visited = std::collections::HashSet::new();
        visited.insert(start);

        while let Some(((i, j), step)) = queue.pop_front() {
            if (i, j) == end {
                return step;
            }

            for (di, dj) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (ni, nj) = (i as i32 + di, j as i32 + dj);
                if ni >= 0
                    && ni < forest.len() as i32
                    && nj >= 0
                    && nj < forest[0].len() as i32
                    && forest[ni as usize][nj as usize] > 0
                    && visited.insert((ni as usize, nj as usize))
                {
                    queue.push_back(((ni as usize, nj as usize), step + 1));
                }
            }
        }
        -1
    }
}

#[test]
fn test() {
    let forest = vec![vec![1, 2, 3], vec![0, 0, 4], vec![7, 6, 5]];
    assert_eq!(Solution::cut_off_tree(forest), 6);

    let forest = vec![vec![1, 2, 3], vec![0, 0, 0], vec![7, 6, 5]];
    assert_eq!(Solution::cut_off_tree(forest), -1);

    let forest = vec![vec![2, 3, 4], vec![0, 0, 5], vec![8, 7, 6]];
    assert_eq!(Solution::cut_off_tree(forest), 6);
}
