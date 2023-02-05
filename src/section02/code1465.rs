#![allow(dead_code)]

/*
// 1465. Maximum Area of a Piece of Cake After Horizontal and Vertical Cuts
Medium

You are given a rectangular cake of size h x w and two arrays of integers horizontalCuts and verticalCuts where:

    horizontalCuts[i] is the distance from the top of the rectangular cake to the ith horizontal cut and similarly, and
    verticalCuts[j] is the distance from the left of the rectangular cake to the jth vertical cut.

Return the maximum area of a piece of cake after you cut at each horizontal and vertical position provided in the arrays horizontalCuts and verticalCuts. Since the answer can be a large number, return this modulo 109 + 7.

Example 1:

Input: h = 5, w = 4, horizontalCuts = [1,2,4], verticalCuts = [1,3]
Output: 4
Explanation: The figure above represents the given rectangular cake. Red lines are the horizontal and vertical cuts. After you cut the cake, the green piece of cake has the maximum area.

Example 2:

Input: h = 5, w = 4, horizontalCuts = [3,1], verticalCuts = [1]
Output: 6
Explanation: The figure above represents the given rectangular cake. Red lines are the horizontal and vertical cuts. After you cut the cake, the green and yellow pieces of cake have the maximum area.

Example 3:

Input: h = 5, w = 4, horizontalCuts = [3], verticalCuts = [3]
Output: 9

Constraints:

    2 <= h, w <= 10^9
    1 <= horizontalCuts.length <= min(h - 1, 10^5)
    1 <= verticalCuts.length <= min(w - 1, 10^5)
    1 <= horizontalCuts[i] < h
    1 <= verticalCuts[i] < w
    All the elements in horizontalCuts are distinct.
    All the elements in verticalCuts are distinct.
*/

struct Solution;

impl Solution {
    pub fn max_area(h: i32, w: i32, horizontal_cuts: Vec<i32>, vertical_cuts: Vec<i32>) -> i32 {
        let mut horizontal_cuts = horizontal_cuts.iter().map(|&x| x as i64).collect::<Vec<_>>();
        let mut vertical_cuts = vertical_cuts.iter().map(|&x| x as i64).collect::<Vec<_>>();
        horizontal_cuts.push(0);
        horizontal_cuts.push(h as i64);
        vertical_cuts.push(0);
        vertical_cuts.push(w as i64);
        horizontal_cuts.sort();
        vertical_cuts.sort();
        let mut max_h = 0;
        let mut max_v = 0;
        for i in 1..horizontal_cuts.len() {
            max_h = max_h.max(horizontal_cuts[i] - horizontal_cuts[i - 1]);
        }
        for i in 1..vertical_cuts.len() {
            max_v = max_v.max(vertical_cuts[i] - vertical_cuts[i - 1]);
        }
        ((max_h * max_v) % 1_000_000_007) as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (5, 4, vec![1, 2, 4], vec![1, 3], 4),
        (5, 4, vec![3, 1], vec![1], 6),
        (5, 4, vec![3], vec![3], 9),
    ];
    for (h, w, hor, v, e) in cases {
        assert_eq!(Solution::max_area(h, w, hor, v), e);
    }
}
