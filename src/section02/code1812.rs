#![allow(dead_code)]

/*

// 1812. Determine Color of a Chessboard Square
// https://leetcode.com/problems/determine-color-of-a-chessboard-square/
// https://leetcode.cn/problems/determine-color-of-a-chessboard-square/
//
// Easy
//
// You are given coordinates, a string that represents the coordinates of a square of the chessboard. Below is a chessboard for your reference.

Return true if the square is white, and false if the square is black.

The coordinate will always represent a valid chessboard square. The coordinate will always have the letter first, and the number second.

Example 1:

Input: coordinates = "a1"
Output: false
Explanation: From the chessboard above, the square with coordinates "a1" is black, so return false.

Example 2:

Input: coordinates = "h3"
Output: true
Explanation: From the chessboard above, the square with coordinates "h3" is white, so return true.

Example 3:

Input: coordinates = "c7"
Output: false

Constraints:

    coordinates.length == 2
    'a' <= coordinates[0] <= 'h'
    '1' <= coordinates[1] <= '8'
*/

struct Solution;

impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        coordinates.as_bytes().iter().fold(0, |acc, &b| acc + (b - b'1') % 2) == 1
    }
}

#[test]
fn test() {
    assert_eq!(Solution::square_is_white("a1".to_string()), false);
    assert_eq!(Solution::square_is_white("h3".to_string()), true);
    assert_eq!(Solution::square_is_white("c7".to_string()), false);
}
