#![allow(dead_code)]

// 3464. Maximize the Distance Between Points on a Square
// https://leetcode.com/problems/maximize-the-distance-between-points-on-a-square/
// https://leetcode.cn/problems/maximize-the-distance-between-points-on-a-square/
//
// Hard
//
// You are given an integer side, representing the edge length of a square with corners at (0, 0), (0, side), (side, 0), and (side, side) on a Cartesian plane.
//
// You are also given a positive integer k and a 2D integer array points, where points[i] = [xi, yi] represents the coordinate of a point lying on the boundary of the square.
//
// You need to select k elements among points such that the minimum Manhattan distance between any two points is maximized.
//
// Return the maximum possible minimum Manhattan distance between the selected k points.
//
// The Manhattan Distance between two cells (xi, yi) and (xj, yj) is |xi - xj| + |yi - yj|.
//
// Example 1:
//
// Input: side = 2, points = [[0,2],[2,0],[2,2],[0,0]], k = 4
//
// Output: 2
//
// Explanation:
//
// Select all four points.
//
// Example 2:
//
// Input: side = 2, points = [[0,0],[1,2],[2,0],[2,2],[2,1]], k = 4
//
// Output: 1
//
// Explanation:
//
// Select the points (0, 0), (2, 0), (2, 2), and (2, 1).
//
// Example 3:
//
// Input: side = 2, points = [[0,0],[0,1],[0,2],[1,2],[2,0],[2,2],[2,1]], k = 5
//
// Output: 1
//
// Explanation:
//
// Select the points (0, 0), (0, 1), (0, 2), (1, 2), and (2, 2).
//
// Constraints:
//
// 1 <= side <= 10^9
// 4 <= points.length <= min(4 * side, 15 * 10^3)
// points[i] == [xi, yi]
// The input is generated such that:
// points[i] lies on the boundary of the square.
// All points[i] are unique.
// 4 <= k <= min(25, points.length)
//

struct Solution;

impl Solution {
    pub fn max_distance(side: i32, points: Vec<Vec<i32>>, k: i32) -> i32 {
        fn md(points: &[Vec<i64>], i: usize, j: usize) -> i64 {
            (points[i][0] - points[j][0]).abs() + (points[i][1] - points[j][1]).abs()
        }

        fn get_next(points: &[Vec<i64>], i: i64, mid: i64) -> i64 {
            let n = points.len();
            let base = i as usize % n;
            let mut lo = i + 1;
            let mut hi = i + n as i64 - 1;
            while lo < hi {
                let m = ((lo + hi + 1) / 2) as usize;
                let d1 = (points[base][0] - points[(m - 1) % n][0]).abs() + (points[base][1] - points[(m - 1) % n][1]).abs();
                let d2 = (points[base][0] - points[m % n][0]).abs() + (points[base][1] - points[m % n][1]).abs();
                if d1 < d2 {
                    lo = m as i64;
                } else {
                    hi = m as i64 - 1;
                }
            }
            let peak = lo;
            let mut candidate1 = -1;
            let mut lo1 = i + 1;
            let mut hi1 = peak;
            while lo1 <= hi1 {
                let m = ((lo1 + hi1) / 2) as usize;
                let d = (points[base][0] - points[m % n][0]).abs() + (points[base][1] - points[m % n][1]).abs();
                if d >= mid {
                    candidate1 = m as i64;
                    hi1 = m as i64 - 1;
                } else {
                    lo1 = m as i64 + 1;
                }
            }
            let mut candidate2 = -1;
            let mut lo2 = peak;
            let mut hi2 = i + n as i64 - 1;
            while lo2 <= hi2 {
                let m = ((lo2 + hi2) / 2) as usize;
                let d = (points[base][0] - points[m % n][0]).abs() + (points[base][1] - points[m % n][1]).abs();
                if d >= mid {
                    candidate2 = m as i64;
                    hi2 = m as i64 - 1;
                } else {
                    lo2 = m as i64 + 1;
                }
            }
            if candidate1 == -1 && candidate2 == -1 {
                return -1;
            }
            if candidate1 == -1 {
                return candidate2;
            }
            if candidate2 == -1 {
                return candidate1;
            }
            if candidate1 < candidate2 {
                candidate1
            } else {
                candidate2
            }
        }

        fn can(points: &[Vec<i64>], n: i64, k: i64, mid: i64) -> bool {
            for i in 0..n {
                let mut cnt = 1;
                let mut cur = i;
                let mut valid = true;
                while cnt < k {
                    let nxt = get_next(points, cur, mid);
                    if nxt == -1 || nxt >= i + n {
                        valid = false;
                        break;
                    }
                    cur = nxt;
                    cnt += 1;
                }
                if valid && cnt == k && md(points, (cur % n) as usize, (i % n) as usize) >= mid {
                    return true;
                }
            }
            false
        }

        let n = points.len() as i64;
        let cx = side as f64 / 2.0;
        let cy = side as f64 / 2.0;
        let mut points: Vec<Vec<i64>> = points.iter().map(|v| v.iter().map(|&x| x as i64).collect()).collect();
        points.sort_by(|a, b| {
            let (a0, a1, b0, b1) = (a[0] as f64, a[1] as f64, b[0] as f64, b[1] as f64);
            (b1 - cy).atan2(b0 - cx).partial_cmp(&(a1 - cy).atan2(a0 - cx)).unwrap()
        });

        let mut lo_val = 0;
        let mut hi_val = 2 * side as i64;
        let mut ans = 0;
        while lo_val <= hi_val {
            let mid = (lo_val + hi_val) / 2;
            if can(&points, n, k as i64, mid) {
                ans = mid;
                lo_val = mid + 1;
            } else {
                hi_val = mid - 1;
            }
        }

        ans as i32
    }
}

#[test]
fn test() {
    let side = 2;
    let points = vec![vec![0, 2], vec![2, 0], vec![2, 2], vec![0, 0]];
    let k = 4;
    let res = 2;
    assert_eq!(Solution::max_distance(side, points, k), res);

    let side = 2;
    let points = vec![vec![0, 0], vec![1, 2], vec![2, 0], vec![2, 2], vec![2, 1]];
    let k = 4;
    let res = 1;
    assert_eq!(Solution::max_distance(side, points, k), res);

    let side = 2;
    let points = vec![vec![0, 0], vec![0, 1], vec![0, 2], vec![1, 2], vec![2, 0], vec![2, 2], vec![2, 1]];
    let k = 5;
    let res = 1;
    assert_eq!(Solution::max_distance(side, points, k), res);
}
