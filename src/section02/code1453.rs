#![allow(dead_code)]

/*
// 1453. Maximum Number of Darts Inside of a Circular Dartboard
Hard

Alice is throwing n darts on a very large wall. You are given an array darts where darts[i] = [xi, yi] is the position of the ith dart that Alice threw on the wall.

Bob knows the positions of the n darts on the wall. He wants to place a dartboard of radius r on the wall so that the maximum number of darts that Alice throws lies on the dartboard.

Given the integer r, return the maximum number of darts that can lie on the dartboard.

Example 1:

Input: darts = [[-2,0],[2,0],[0,2],[0,-2]], r = 2
Output: 4
Explanation: Circle dartboard with center in (0,0) and radius = 2 contain all points.

Example 2:

Input: darts = [[-3,0],[3,0],[2,6],[5,4],[0,9],[7,8]], r = 5
Output: 5
Explanation: Circle dartboard with center in (0,4) and radius = 5 contain all points except the point (7,8).

Constraints:

    1 <= darts.length <= 100
    darts[i].length == 2
    -10^4 <= xi, yi <= 10^4
    All the darts are unique
    1 <= r <= 5000
*/

struct Solution;

impl Solution {
    pub fn num_points(darts: Vec<Vec<i32>>, r: i32) -> i32 {
        let mut res = 1;
        let r = r as f64;
        for i in 0..darts.len() - 1 {
            for j in i + 1..darts.len() {
                let x1 = darts[i][0] as f64;
                let y1 = darts[i][1] as f64;
                let x2 = darts[j][0] as f64;
                let y2 = darts[j][1] as f64;
                let d = ((x1 - x2).powi(2) + (y1 - y2).powi(2)) / 4.0;
                if d > r.powi(2) {
                    continue;
                }
                let x0 = (x1 + x2) / 2.0 + (y2 - y1) * (r.powi(2) - d).sqrt() / (d * 4.0).sqrt();
                let y0 = (y1 + y2) / 2.0 - (x2 - x1) * (r.powi(2) - d).sqrt() / (d * 4.0).sqrt();
                let mut cnt = 0;
                for point in &darts {
                    let x = point[0] as f64;
                    let y = point[1] as f64;
                    if (x - x0).powi(2) + (y - y0).powi(2) <= r.powi(2) + 0.00001 {
                        cnt += 1;
                    }
                }
                res = res.max(cnt);
                let x0 = (x1 + x2) / 2.0 - (y2 - y1) * (r.powi(2) - d).sqrt();
                let y0 = (y1 + y2) / 2.0 + (x2 - x1) * (r.powi(2) - d).sqrt();
                let mut cnt = 0;
                for point in &darts {
                    let x = point[0] as f64;
                    let y = point[1] as f64;
                    if (x - x0).powi(2) + (y - y0).powi(2) <= r.powi(2) + 0.00001 {
                        cnt += 1;
                    }
                }
                res = res.max(cnt);
            }
        }
        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![-2, 0], vec![2, 0], vec![0, 2], vec![0, -2]], 2, 4),
        (
            vec![vec![-3, 0], vec![3, 0], vec![2, 6], vec![5, 4], vec![0, 9], vec![7, 8]],
            5,
            5,
        ),
    ];
    for (darts, r, expected) in cases {
        assert_eq!(Solution::num_points(darts, r), expected);
    }
}
