#![allow(dead_code)]

/*

// 2001. Number of Pairs of Interchangeable Rectangles
// https://leetcode.com/problems/number-of-pairs-of-interchangeable-rectangles/
// https://leetcode.cn/problems/number-of-pairs-of-interchangeable-rectangles/
//
// Medium
//
// You are given n rectangles represented by a 0-indexed 2D integer array rectangles, where rectangles[i] = [widthi, heighti] denotes the width and height of the ith rectangle.

Two rectangles i and j (i < j) are considered interchangeable if they have the same width-to-height ratio. More formally, two rectangles are interchangeable if widthi/heighti == widthj/heightj (using decimal division, not integer division).

Return the number of pairs of interchangeable rectangles in rectangles.

Example 1:

Input: rectangles = [[4,8],[3,6],[10,20],[15,30]]
Output: 6
Explanation: The following are the interchangeable pairs of rectangles by index (0-indexed):
- Rectangle 0 with rectangle 1: 4/8 == 3/6.
- Rectangle 0 with rectangle 2: 4/8 == 10/20.
- Rectangle 0 with rectangle 3: 4/8 == 15/30.
- Rectangle 1 with rectangle 2: 3/6 == 10/20.
- Rectangle 1 with rectangle 3: 3/6 == 15/30.
- Rectangle 2 with rectangle 3: 10/20 == 15/30.

Example 2:

Input: rectangles = [[4,5],[7,8]]
Output: 0
Explanation: There are no interchangeable pairs of rectangles.

Constraints:

    n == rectangles.length
    1 <= n <= 10^5
    rectangles[i].length == 2
    1 <= widthi, heighti <= 10^5
*/

struct Solution;

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut ratios = HashMap::new();
        let mut interchangeable_count = 0;
        for rectangle in rectangles {
            let (w, h) = (rectangle[0], rectangle[1]);
            let ratio = w as f64 / h as f64;
            *ratios.entry(F64(ratio)).or_insert(0) += 1;
        }
        for ratio_count in ratios.values() {
            interchangeable_count += (ratio_count * (ratio_count - 1)) / 2;
        }
        interchangeable_count
    }
}

use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
    ops::Div,
};

struct F64(f64);

impl Hash for F64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let bits = self.0.to_bits();
        bits.hash(state);
    }
}

impl PartialEq for F64 {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Div for F64 {
    type Output = F64;

    fn div(self, other: F64) -> F64 {
        F64(self.0 / other.0)
    }
}

impl Eq for F64 {}

#[test]
fn test() {
    let cases = vec![
        (vec![vec![4, 8], vec![3, 6], vec![10, 20], vec![15, 30]], 6),
        (vec![vec![4, 5], vec![7, 8]], 0),
    ];
    for (rectangles, expected) in cases {
        assert_eq!(Solution::interchangeable_rectangles(rectangles), expected);
    }
}
