#![allow(dead_code)]

// 3143. Maximum Points Inside the Square
// https://leetcode.com/problems/maximum-points-inside-the-square/
// https://leetcode.cn/problems/maximum-points-inside-the-square/
//
// Medium
//
// You are given a 2D array points and a string s where, points[i] represents the coordinates of point i, and s[i] represents the tag of point i.
//
// A valid square is a square centered at the origin (0, 0), has edges parallel to the axes, and does not contain two points with the same tag.
//
// Return the maximum number of points contained in a valid square.
//
// Note:
//
// A point is considered to be inside the square if it lies on or within the square's boundaries.
// The side length of the square can be zero.
//
// Example 1:
//
// Input: points = [[2,2],[-1,-2],[-4,4],[-3,1],[3,-3]], s = "abdca"
//
// Output: 2
//
// Explanation:
//
// The square of side length 4 covers two points points[0] and points[1].
//
// Example 2:
//
// Input: points = [[1,1],[-2,-2],[-2,2]], s = "abb"
//
// Output: 1
//
// Explanation:
//
// The square of side length 2 covers one point, which is points[0].
//
// Example 3:
//
// Input: points = [[1,1],[-1,-1],[2,-2]], s = "ccd"
//
// Output: 0
//
// Explanation:
//
// It's impossible to make any valid squares centered at the origin such that it covers only one point among points[0] and points[1].
//
// Constraints:
//
// 1 <= s.length, points.length <= 10^5
// points[i].length == 2
// -10^9 <= points[i][0], points[i][1] <= 10^9
// s.length == points.length
// points consists of distinct coordinates.
// s consists only of lowercase English letters.
//

struct Solution;

impl Solution {
    pub fn max_points_inside_square(points: Vec<Vec<i32>>, s: String) -> i32 {
        let s = s.as_bytes();
        let (mut lo, mut hi, mut res) = (0, 0, 0);
        for points_i in &points {
            hi = hi.max(points_i[0].abs()).max(points_i[1].abs());
        }
        hi += 3;

        while lo < hi {
            let mi = (lo + hi) / 2;
            let (is_ok, num) = Self::is_ok(&points, s, mi);

            if is_ok {
                lo = mi + 1;
                res = res.max(num);
            } else {
                hi = mi;
            }
        }

        res
    }

    fn is_ok(points: &[Vec<i32>], s: &[u8], h: i32) -> (bool, i32) {
        let mut seen = [false; 26];
        let mut num_points = 0;

        for i in 0..points.len() {
            if -h <= points[i][0] && points[i][0] <= h && -h <= points[i][1] && points[i][1] <= h {
                if seen[(s[i] - 97) as usize] {
                    return (false, -1);
                }
                seen[(s[i] - 97) as usize] = true;
                num_points += 1;
            }
        }

        (true, num_points)
    }
}

#[test]
fn test() {
    let points = vec![vec![2, 2], vec![-1, -2], vec![-4, 4], vec![-3, 1], vec![3, -3]];
    let s = "abdca".to_string();
    let res = 2;
    assert_eq!(Solution::max_points_inside_square(points, s), res);

    let points = vec![vec![1, 1], vec![-2, -2], vec![-2, 2]];
    let s = "abb".to_string();
    let res = 1;
    assert_eq!(Solution::max_points_inside_square(points, s), res);

    let points = vec![vec![1, 1], vec![-1, -1], vec![2, -2]];
    let s = "ccd".to_string();
    let res = 0;
    assert_eq!(Solution::max_points_inside_square(points, s), res);
}
