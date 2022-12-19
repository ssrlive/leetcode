#![allow(dead_code)]

// 149. Max Points on a Line
// https://leetcode.com/problems/max-points-on-a-line/
// https://leetcode.cn/problems/max-points-on-a-line/
//
// Given an array of points where points[i] = [xi, yi] represents a point on the X-Y plane,
// return the maximum number of points that lie on the same straight line.
//

struct Solution;

impl Solution {
    pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
        fn slope(p1: &[i32], p2: &[i32]) -> f64 {
            if p1[0] - p2[0] == 0 {
                return f64::INFINITY;
            } else if p1[1] - p2[1] == 0 {
                return -f64::INFINITY;
            }
            (p1[0] - p2[0]) as f64 / (p1[1] - p2[1]) as f64
        }

        if points.len() <= 2 {
            return points.len() as i32;
        }
        let mut streak = 2;
        let mut cp = points.clone();
        let mut ans = 2;
        for p in points {
            cp.sort_by(|a, b| slope(a, &p).partial_cmp(&slope(b, &p)).unwrap());
            let mut last_slope = f64::NAN;
            streak = 2;
            for each in cp.iter() {
                let this_slope = slope(each, &p);
                if each[0] == p[0] && each[1] == p[1] {
                    continue;
                }
                if last_slope == this_slope {
                    streak += 1;
                    ans = i32::max(ans, streak);
                } else {
                    streak = 2;
                }
                last_slope = this_slope;
            }
        }
        i32::max(ans, streak)
    }
}

#[test]
fn test_max_points() {
    assert_eq!(Solution::max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]), 3);
    assert_eq!(
        Solution::max_points(vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4]
        ]),
        4
    );
}
