#![allow(dead_code)]

// 3531. Count Covered Buildings
// https://leetcode.com/problems/count-covered-buildings/
// https://leetcode.cn/problems/count-covered-buildings/
//
// Medium
//
// You are given a positive integer n, representing an n x n city. You are also given a 2D grid buildings, where buildings[i] = [x, y] denotes a unique building located at coordinates [x, y].
//
// A building is covered if there is at least one building in all four directions: left, right, above, and below.
//
// Return the number of covered buildings.
//
// Example 1:
//
// Input: n = 3, buildings = [[1,2],[2,2],[3,2],[2,1],[2,3]]
//
// Output: 1
//
// Explanation:
//
//     Only building [2,2] is covered as it has at least one building:
//         above ([1,2])
//         below ([3,2])
//         left ([2,1])
//         right ([2,3])
//     Thus, the count of covered buildings is 1.
//
// Example 2:
//
// Input: n = 3, buildings = [[1,1],[1,2],[2,1],[2,2]]
//
// Output: 0
//
// Explanation:
//
//     No building has at least one building in all four directions.
//
// Example 3:
//
// Input: n = 5, buildings = [[1,3],[3,2],[3,3],[3,5],[5,3]]
//
// Output: 1
//
// Explanation:
//
//     Only building [3,3] is covered as it has at least one building:
//         above ([1,3])
//         below ([5,3])
//         left ([3,2])
//         right ([3,5])
//     Thus, the count of covered buildings is 1.
//
// Constraints:
//
//     2 <= n <= 10^5
//     1 <= buildings.length <= 10^5
//     buildings[i] = [x, y]
//     1 <= x, y <= n
//     All coordinates of buildings are unique.
//

struct Solution;

impl Solution {
    pub fn count_covered_buildings(_n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        use std::cmp::{max, min};
        use std::collections::HashMap;

        let mut r = 0;
        let mut x_coord: HashMap<i32, [i32; 2]> = HashMap::new();
        let mut y_coord: HashMap<i32, [i32; 2]> = HashMap::new();
        for v in &buildings {
            let (x, y) = (v[0], v[1]);
            if x_coord.contains_key(&x) {
                x_coord.insert(x, [min(y, x_coord[&x][0]), max(y, x_coord[&x][1])]);
            } else {
                x_coord.insert(x, [y, y]);
            }
            if y_coord.contains_key(&y) {
                y_coord.insert(y, [min(x, y_coord[&y][0]), max(x, y_coord[&y][1])]);
            } else {
                y_coord.insert(y, [x, x]);
            }
        }
        for v in &buildings {
            let (x, y) = (v[0], v[1]);
            if (x_coord[&x][0] < y && y < x_coord[&x][1]) && (y_coord[&y][0] < x && x < y_coord[&y][1]) {
                r += 1;
            }
        }
        r
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::count_covered_buildings(3, vec![vec![1, 2], vec![2, 2], vec![3, 2], vec![2, 1], vec![2, 3]]),
        1
    );
    assert_eq!(
        Solution::count_covered_buildings(3, vec![vec![1, 1], vec![1, 2], vec![2, 1], vec![2, 2]]),
        0
    );
    assert_eq!(
        Solution::count_covered_buildings(5, vec![vec![1, 3], vec![3, 2], vec![3, 3], vec![3, 5], vec![5, 3]]),
        1
    );
}
