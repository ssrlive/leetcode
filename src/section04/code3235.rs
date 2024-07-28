#![allow(dead_code)]

// 3235. Check if the Rectangle Corner Is Reachable
// https://leetcode.com/problems/check-if-the-rectangle-corner-is-reachable/
// https://leetcode.cn/problems/check-if-the-rectangle-corner-is-reachable/
//
// Hard
//
// You are given two positive integers X and Y, and a 2D array circles, where circles[i] = [xi, yi, ri] denotes a circle with center at (xi, yi) and radius ri.
//
// There is a rectangle in the coordinate plane with its bottom left corner at the origin and top right corner at the coordinate (X, Y).
// You need to check whether there is a path from the bottom left corner to the top right corner such that the entire path lies inside the rectangle,
// does not touch or lie inside any circle, and touches the rectangle only at the two corners.
//
// Return true if such a path exists, and false otherwise.
//
// Example 1:
//
// Input: X = 3, Y = 4, circles = [[2,1,1]]
//
// Output: true
//
// Explanation:
//
// The black curve shows a possible path between (0, 0) and (3, 4).
//
// Example 2:
//
// Input: X = 3, Y = 3, circles = [[1,1,2]]
//
// Output: false
//
// Explanation:
//
// No path exists from (0, 0) to (3, 3).
//
// Example 3:
//
// Input: X = 3, Y = 3, circles = [[2,1,1],[1,2,1]]
//
// Output: false
//
// Explanation:
//
// No path exists from (0, 0) to (3, 3).
//
// Constraints:
//
//     3 <= X, Y <= 10^9
//     1 <= circles.length <= 1000
//     circles[i].length == 3
//     1 <= xi, yi, ri <= 10^9
//

struct Solution;

impl Solution {
    pub fn can_reach_corner(x: i32, y: i32, circles: Vec<Vec<i32>>) -> bool {
        let circles: Vec<Vec<i64>> = circles.iter().map(|v| v.iter().map(|&x| x as i64).collect()).collect();
        let n = circles.len();
        let mut graph = vec![vec![]; n];
        let mut bottom = std::collections::HashSet::new();
        let mut right = std::collections::HashSet::new();
        let mut left = std::collections::HashSet::new();
        let mut up = std::collections::HashSet::new();
        for i in 0..n {
            for j in i + 1..n {
                if (circles[i][2] + circles[j][2]) * (circles[i][2] + circles[j][2])
                    >= (circles[i][0] - circles[j][0]) * (circles[i][0] - circles[j][0])
                        + (circles[i][1] - circles[j][1]) * (circles[i][1] - circles[j][1])
                {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
            if circles[i][0] <= circles[i][2] {
                bottom.insert(i);
            }
            if circles[i][1] <= circles[i][2] {
                left.insert(i);
            }
            if x as i64 - circles[i][0] <= circles[i][2] {
                up.insert(i);
            }
            if y as i64 - circles[i][1] <= circles[i][2] {
                right.insert(i);
            }
        }

        let mut pos = true;
        let mut q = std::collections::VecDeque::new();
        let mut vis = vec![0; n];
        for &b in &bottom {
            q.push_back(b);
            vis[b] = 1;
        }
        while !q.is_empty() {
            let node = q.pop_front().unwrap();
            if up.contains(&node) || left.contains(&node) {
                pos = false;
                break;
            }
            for &g in &graph[node] {
                if vis[g] == 0 {
                    vis[g] = 1;
                    q.push_back(g);
                }
            }
        }
        if !pos {
            return false;
        }
        for vis_i in vis.iter_mut().take(n) {
            *vis_i = 0;
        }
        for &r in &right {
            q.push_back(r);
            vis[r] = 1;
        }
        while !q.is_empty() {
            let node = q.pop_front().unwrap();
            if up.contains(&node) || left.contains(&node) {
                pos = false;
                break;
            }
            for &g in &graph[node] {
                if vis[g] == 0 {
                    vis[g] = 1;
                    q.push_back(g);
                }
            }
        }
        pos
    }
}

#[test]
fn test() {
    let x = 3;
    let y = 4;
    let circles = vec![vec![2, 1, 1]];
    let res = true;
    assert_eq!(Solution::can_reach_corner(x, y, circles), res);

    let x = 3;
    let y = 3;
    let circles = vec![vec![1, 1, 2]];
    let res = false;
    assert_eq!(Solution::can_reach_corner(x, y, circles), res);

    let x = 3;
    let y = 3;
    let circles = vec![vec![2, 1, 1], vec![1, 2, 1]];
    let res = false;
    assert_eq!(Solution::can_reach_corner(x, y, circles), res);
}
