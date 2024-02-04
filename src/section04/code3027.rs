#![allow(dead_code)]

// 3027. Find the Number of Ways to Place People II
// https://leetcode.com/problems/find-the-number-of-ways-to-place-people-ii/
// https://leetcode.cn/problems/find-the-number-of-ways-to-place-people-ii/
//
// Hard
//
// You are given a 2D array points of size n x 2 representing integer coordinates of some points on a 2D-plane, where points[i] = [xi, yi].
//
// We define the right direction as positive x-axis (increasing x-coordinate) and the left direction as negative x-axis (decreasing x-coordinate).
// Similarly, we define the up direction as positive y-axis (increasing y-coordinate) and the down direction as negative y-axis (decreasing y-coordinate)
//
// You have to place n people, including Chisato and Takina, at these points such that there is exactly one person at every point.
// Chisato wants to be alone with Takina, so Chisato will build a rectangular fence with Chisato's position as the upper left corner
// and Takina's position as the lower right corner of the fence (Note that the fence might not enclose any area, i.e. it can be a line).
// If any person other than Chisato and Takina is either inside the fence or on the fence, Chisato will be sad.
//
// Return the number of pairs of points where you can place Chisato and Takina, such that Chisato does not become sad on building the fence.
//
// Note that Chisato can only build a fence with Chisato's position as the upper left corner, and Takina's position as the lower right corner.
// For example, Chisato cannot build either of the fences in the picture below with four corners (1, 1), (1, 3), (3, 1), and (3, 3), because:
//
// With Chisato at (3, 3) and Takina at (1, 1), Chisato's position is not the upper left corner and Takina's position is not the lower right corner of the fence.
// With Chisato at (1, 3) and Takina at (1, 1), Takina's position is not the lower right corner of the fence.
//
// Example 1:
//
// Input: points = [[1,1],[2,2],[3,3]]
// Output: 0
// Explanation: There is no way to place Chisato and Takina such that Chisato can build a fence with Chisato's position as the upper
// left corner and Takina's position as the lower right corner. Hence we return 0.
//
// Example 2:
//
// Input: points = [[6,2],[4,4],[2,6]]
// Output: 2
// Explanation: There are two ways to place Chisato and Takina such that Chisato will not be sad:
// - Place Chisato at (4, 4) and Takina at (6, 2).
// - Place Chisato at (2, 6) and Takina at (4, 4).
// You cannot place Chisato at (2, 6) and Takina at (6, 2) because the person at (4, 4) will be inside the fence.
//
// Example 3:
//
// Input: points = [[3,1],[1,3],[1,1]]
// Output: 2
// Explanation: There are two ways to place Chisato and Takina such that Chisato will not be sad:
// - Place Chisato at (1, 1) and Takina at (3, 1).
// - Place Chisato at (1, 3) and Takina at (1, 1).
// You cannot place Chisato at (1, 3) and Takina at (3, 1) because the person at (1, 1) will be on the fence.
// Note that it does not matter if the fence encloses any area, the first and second fences in the image are valid.
//
// Constraints:
//
// 2 <= n <= 1000
// points[i].length == 2
// -10^9 <= points[i][0], points[i][1] <= 10^9
// All points[i] are distinct.
//

struct Solution;

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut res = 0;
        let mut points = points;
        points.sort_unstable_by(|a, b| if a[0] == b[0] { b[1].cmp(&a[1]) } else { a[0].cmp(&b[0]) });
        let n = points.len();
        for i in 0..n {
            let mut y = std::i32::MIN;
            for j in i + 1..n {
                if points[i][1] >= points[j][1] && y < points[j][1] {
                    res += 1;
                    y = points[j][1];
                }
            }
        }
        res
    }
}

#[test]
fn test() {
    let points = vec![vec![1, 1], vec![2, 2], vec![3, 3]];
    let res = 0;
    assert_eq!(Solution::number_of_pairs(points), res);
    let points = vec![vec![6, 2], vec![4, 4], vec![2, 6]];
    let res = 2;
    assert_eq!(Solution::number_of_pairs(points), res);
    let points = vec![vec![3, 1], vec![1, 3], vec![1, 1]];
    let res = 2;
    assert_eq!(Solution::number_of_pairs(points), res);
}
