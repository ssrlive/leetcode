#![allow(dead_code)]

// 497. Random Point in Non-overlapping Rectangles
// https://leetcode.com/problems/random-point-in-non-overlapping-rectangles/
// https://leetcode.cn/problems/random-point-in-non-overlapping-rectangles/
//
// You are given an array of non-overlapping axis-aligned rectangles rects where rects[i] = [ai, bi, xi, yi] indicates that (ai, bi) is the bottom-left corner point of the ith rectangle and (xi, yi) is the top-right corner point of the ith rectangle. Design an algorithm to pick a random integer point inside the space covered by one of the given rectangles. A point on the perimeter of a rectangle is included in the space covered by the rectangle.
//
// Any integer point inside the space covered by one of the given rectangles should be equally likely to be returned.
//
// Note that an integer point is a point that has integer coordinates.
//
// Implement the Solution class:
//
// Solution(int[][] rects) Initializes the object with the given rectangles rects.
// int[] pick() Returns a random integer point [u, v] inside the space covered by one of the given rectangles.
//
// Example 1:
//
// Input
// ["Solution", "pick", "pick", "pick", "pick", "pick"]
// [[[[-2, -2, 1, 1], [2, 2, 4, 6]]], [], [], [], [], []]
// Output
// [null, [1, -2], [1, -1], [-1, -2], [-2, -2], [0, 0]]
//
// Explanation
// Solution solution = new Solution([[-2, -2, 1, 1], [2, 2, 4, 6]]);
// solution.pick(); // return [1, -2]
// solution.pick(); // return [1, -1]
// solution.pick(); // return [-1, -2]
// solution.pick(); // return [-2, -2]
// solution.pick(); // return [0, 0]
//
// Constraints:
//
// - 1 <= rects.length <= 100
// - rects[i].length == 4
// - -10^9 <= ai < xi <= 10^9
// - -10^9 <= bi < yi <= 10^9
// - xi - ai <= 2000
// - yi - bi <= 2000
// - All the rectangles do not overlap.
// - At most 104 calls will be made to pick.
//

struct Solution {
    rects: Vec<Vec<i32>>,
    c_areas: Vec<i32>,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        let mut c_areas = vec![0];
        for rect in &rects {
            let area = (rect[2] - rect[0] + 1) * (rect[3] - rect[1] + 1);
            c_areas.push(area + c_areas[c_areas.len() - 1]);
        }
        Solution { rects, c_areas }
    }

    fn pick(&self) -> Vec<i32> {
        use rand::Rng;
        let r = rand::rng().random_range(0..self.c_areas[self.c_areas.len() - 1]) + 1;
        let i = self.c_areas.binary_search(&r).unwrap_or_else(|x| x) - 1;
        let rect = &self.rects[i];
        let x = rand::rng().random_range(rect[0]..rect[2] + 1);
        let y = rand::rng().random_range(rect[1]..rect[3] + 1);
        vec![x, y]
    }
}

#[test]
fn test() {
    let solution = Solution::new(vec![vec![-2, -2, 1, 1], vec![2, 2, 4, 6]]);
    for _ in 0..10 {
        println!("{:?}", solution.pick());
    }
}
