#![allow(dead_code)]

// 3380. Maximum Area Rectangle With Point Constraints I
// https://leetcode.com/problems/maximum-area-rectangle-with-point-constraints-i/
// https://leetcode.cn/problems/maximum-area-rectangle-with-point-constraints-i/
//
// Medium
//
// You are given an array points where points[i] = [xi, yi] represents the coordinates of a point on an infinite plane.
//
// Your task is to find the maximum area of a rectangle that:
//
// Can be formed using four of these points as its corners.
// Does not contain any other point inside or on its border.
// Has its edges parallel to the axes.
// Return the maximum area that you can obtain or -1 if no such rectangle is possible.
//
// Example 1:
//
// Input: points = [[1,1],[1,3],[3,1],[3,3]]
//
// Output: 4
//
// Explanation:
//
// Example 1 diagram
//
// We can make a rectangle with these 4 points as corners and there is no other point
// that lies inside or on the border. Hence, the maximum possible area would be 4.
//
// Example 2:
//
// Input: points = [[1,1],[1,3],[3,1],[3,3],[2,2]]
//
// Output: -1
//
// Explanation:
//
// Example 2 diagram
//
// There is only one rectangle possible is with points [1,1], [1,3], [3,1] and [3,3]
// but [2,2] will always lie inside it. Hence, returning -1.
//
// Example 3:
//
// Input: points = [[1,1],[1,3],[3,1],[3,3],[1,2],[3,2]]
//
// Output: 2
//
// Explanation:
//
// Example 3 diagram
//
// The maximum area rectangle is formed by the points [1,3], [1,2], [3,2], [3,3], which has an area of 2.
// Additionally, the points [1,1], [1,2], [3,1], [3,2] also form a valid rectangle with the same area.
//
// Constraints:
//
// 1 <= points.length <= 10
// points[i].length == 2
// 0 <= xi, yi <= 100
// All the given points are unique.
//

struct Solution;

impl Solution {
    pub fn max_rectangle_area(points: Vec<Vec<i32>>) -> i32 {
        fn find_area(points: &[Vec<i32>]) -> i32 {
            let bl = &points[0];
            let tl = &points[1];
            let br = &points[2];
            let tr = &points[3];

            if bl[0] != tl[0] || br[0] != tr[0] {
                return -1;
            }
            if bl[1] != br[1] || tl[1] != tr[1] {
                return -1;
            }
            (bl[0] - br[0]).abs() * (tl[1] - bl[1]).abs()
        }

        fn has_between_points(points: &[Vec<i32>], vals: &[Vec<i32>]) -> bool {
            let bl = &vals[0];
            let tl = &vals[1];
            let br = &vals[2];
            let tr = &vals[3];

            for it in points {
                if it == bl || it == tl || it == br || it == tr {
                    continue;
                }
                let x = it[0];
                let y = it[1];
                if x >= bl[0] && x <= br[0] && y >= bl[1] && y <= tl[1] {
                    return true;
                }
            }
            false
        }

        let mut ans = -1;
        let n = points.len();
        if n < 4 {
            return ans;
        }
        for i in 0..n {
            for j in i + 1..n {
                for k in j + 1..n {
                    for l in k + 1..n {
                        let mut vals = vec![points[i].clone(), points[j].clone(), points[k].clone(), points[l].clone()];
                        vals.sort();
                        let area = find_area(&vals);
                        if area == -1 {
                            continue;
                        }
                        if has_between_points(&points, &vals) {
                            continue;
                        }
                        ans = ans.max(area);
                    }
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let points = vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3]];
    let res = 4;
    assert_eq!(Solution::max_rectangle_area(points), res);

    let points = vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3], vec![2, 2]];
    let res = -1;
    assert_eq!(Solution::max_rectangle_area(points), res);

    let points = vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3], vec![1, 2], vec![3, 2]];
    let res = 2;
    assert_eq!(Solution::max_rectangle_area(points), res);
}
