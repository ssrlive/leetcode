#![allow(dead_code)]

// 3200. Maximum Height of a Triangle
// https://leetcode.com/problems/maximum-height-of-a-triangle/
// https://leetcode.cn/problems/maximum-height-of-a-triangle/
//
// Easy
//
// You are given two integers red and blue representing the count of red and blue colored balls.
// You have to arrange these balls to form a triangle such that the 1st row will have 1 ball,
// the 2nd row will have 2 balls, the 3rd row will have 3 balls, and so on.
//
// All the balls in a particular row should be the same color, and adjacent rows should have different colors.
//
// Return the maximum height of the triangle that can be achieved.
//
// Example 1:
//
// Input: red = 2, blue = 4
//
// Output: 3
//
// Explanation:
//
// The only possible arrangement is shown above.
//
// Example 2:
//
// Input: red = 2, blue = 1
//
// Output: 2
//
// Explanation:
//
// The only possible arrangement is shown above.
//
// Example 3:
//
// Input: red = 1, blue = 1
//
// Output: 1
//
// Example 4:
//
// Input: red = 10, blue = 1
//
// Output: 2
//
// Explanation:
//
// The only possible arrangement is shown above.
//
// Constraints:
//
//     1 <= red, blue <= 100
//

struct Solution;

impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        fn f(n: i32, m: i32) -> i32 {
            let k = (n as f64).sqrt() as i32;
            let l = ((((4 * m + 1) as f64).sqrt() - 1.0) / 2.0) as i32;
            k.min(l + 1) + l.min(k)
        }

        f(red, blue).max(f(blue, red))
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_height_of_triangle(2, 4), 3);
    assert_eq!(Solution::max_height_of_triangle(2, 1), 2);
    assert_eq!(Solution::max_height_of_triangle(1, 1), 1);
    assert_eq!(Solution::max_height_of_triangle(10, 1), 2);
}
