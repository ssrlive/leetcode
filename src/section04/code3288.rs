#![allow(dead_code)]

// 3288. Length of the Longest Increasing Path
// https://leetcode.com/problems/length-of-the-longest-increasing-path/
// https://leetcode.cn/problems/length-of-the-longest-increasing-path/
//
// Hard
//
// You are given a 2D array of integers coordinates of length n and an integer k, where 0 <= k < n.
//
// coordinates[i] = [xi, yi] indicates the point (xi, yi) in a 2D plane.
//
// An increasing path of length m is defined as a list of points (x1, y1), (x2, y2), (x3, y3), ..., (xm, ym) such that:
//
//     xi < xi + 1 and yi < yi + 1 for all i where 1 <= i < m.
//     (xi, yi) is in the given coordinates for all i where 1 <= i <= m.
//
// Return the maximum length of an increasing path that contains coordinates[k].
//
// Example 1:
//
// Input: coordinates = [[3,1],[2,2],[4,1],[0,0],[5,3]], k = 1
//
// Output: 3
//
// Explanation:
//
// (0, 0), (2, 2), (5, 3) is the longest increasing path that contains (2, 2).
//
// Example 2:
//
// Input: coordinates = [[2,1],[7,0],[5,6]], k = 2
//
// Output: 2
//
// Explanation:
//
// (2, 1), (5, 6) is the longest increasing path that contains (5, 6).
//
// Constraints:
//
//     1 <= n == coordinates.length <= 10^5
//     coordinates[i].length == 2
//     0 <= coordinates[i][0], coordinates[i][1] <= 10^9
//     All elements in coordinates are distinct.
//     0 <= k <= n - 1
//

struct Solution;

impl Solution {
    pub fn max_path_length(coordinates: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut coordinates = coordinates;
        let k = k as usize;
        let x = coordinates[k][0];
        let y = coordinates[k][1];
        coordinates.sort_by(|a, b| if a[0] == b[0] { b[1].cmp(&a[1]) } else { a[0].cmp(&b[0]) });
        let mut before = Vec::new();
        let mut after = Vec::new();
        for c in &coordinates {
            if c[0] < x && c[1] < y {
                Solution::insert_lis(&mut before, c[1]);
            } else if c[0] > x && c[1] > y {
                Solution::insert_lis(&mut after, c[1]);
            }
        }
        1 + before.len() as i32 + after.len() as i32
    }

    fn insert_lis(ms: &mut Vec<i32>, y: i32) {
        fn lower_bound<T: Ord>(slice: &[T], value: &T) -> usize {
            slice.partition_point(|x| x < value)
        }

        let it = lower_bound(ms, &y);
        if it == ms.len() {
            ms.push(y);
        } else {
            ms[it] = y;
        }
    }
}

#[test]
fn test() {
    let coordinates = vec![vec![3, 1], vec![2, 2], vec![4, 1], vec![0, 0], vec![5, 3]];
    let k = 1;
    let res = 3;
    assert_eq!(Solution::max_path_length(coordinates, k), res);

    let coordinates = vec![vec![2, 1], vec![7, 0], vec![5, 6]];
    let k = 2;
    let res = 2;
    assert_eq!(Solution::max_path_length(coordinates, k), res);
}
