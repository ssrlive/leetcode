#![allow(dead_code)]

// 3382. Maximum Area Rectangle With Point Constraints II
// https://leetcode.com/problems/maximum-area-rectangle-with-point-constraints-ii/
// https://leetcode.cn/problems/maximum-area-rectangle-with-point-constraints-ii/
//
// Hard
//
// There are n points on an infinite plane. You are given two integer arrays xCoord and yCoord
// where (xCoord[i], yCoord[i]) represents the coordinates of the ith point.
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
// Input: xCoord = [1,1,3,3], yCoord = [1,3,1,3]
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
// Input: xCoord = [1,1,3,3,2], yCoord = [1,3,1,3,2]
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
// Input: xCoord = [1,1,3,3,1,3], yCoord = [1,3,1,3,2,2]
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
// 1 <= xCoord.length == yCoord.length <= 2 * 10^5
// 0 <= xCoord[i], yCoord[i] <= 8 * 10^7
// All the given points are unique.
//

struct Solution;

impl Solution {
    pub fn max_rectangle_area(x_coord: Vec<i32>, y_coord: Vec<i32>) -> i64 {
        fn xx(v: &[i32], val: i32) -> i32 {
            let mut s = 0;
            let mut e = v.len() as i32 - 1;
            let mut ans = i32::MAX;
            while s <= e {
                let mid = (s + e) / 2;
                if v[mid as usize] > val {
                    ans = v[mid as usize];
                    e = mid - 1;
                } else {
                    s = mid + 1;
                }
            }
            ans
        }

        let mut v = Vec::new();
        let mut mpp = std::collections::BTreeMap::new();
        for i in 0..x_coord.len() {
            mpp.entry(y_coord[i]).or_insert(Vec::new()).push(x_coord[i]);
            v.push((x_coord[i], y_coord[i]));
        }

        for (_, v) in mpp.iter_mut() {
            v.sort_unstable();
        }

        v.sort_unstable();

        let mut ans = -1;

        for i in 1..v.len() {
            if v[i].0 != v[i - 1].0 {
                continue;
            }

            let x1 = xx(&mpp[&v[i].1], v[i].0);
            let x2 = xx(&mpp[&v[i - 1].1], v[i - 1].0);

            if x1 == x2 && x1 != i32::MAX {
                let height = (v[i].1 - v[i - 1].1).abs();
                let width = (v[i].0 - x1).abs();
                let area = height as i64 * width as i64;

                let mut valid = true;
                for (cnt, j) in (i + 1..v.len()).enumerate() {
                    if cnt > 100 {
                        break;
                    }
                    if v[j].0 > x1 {
                        break;
                    }
                    if v[j].1 < v[i].1 && v[j].1 > v[i - 1].1 {
                        valid = false;
                        break;
                    }
                }

                if valid {
                    ans = ans.max(area);
                }
            }
        }
        ans
    }
}

#[test]
fn test() {
    let x_coord = vec![1, 1, 3, 3];
    let y_coord = vec![1, 3, 1, 3];
    let res = 4;
    assert_eq!(Solution::max_rectangle_area(x_coord, y_coord), res);

    let x_coord = vec![1, 1, 3, 3, 2];
    let y_coord = vec![1, 3, 1, 3, 2];
    let res = -1;
    assert_eq!(Solution::max_rectangle_area(x_coord, y_coord), res);

    let x_coord = vec![1, 1, 3, 3, 1, 3];
    let y_coord = vec![1, 3, 1, 3, 2, 2];
    let res = 2;
    assert_eq!(Solution::max_rectangle_area(x_coord, y_coord), res);
}
