#![allow(dead_code)]

// 1007. Minimum Domino Rotations For Equal Row
// https://leetcode.com/problems/minimum-domino-rotations-for-equal-row/
// https://leetcode.cn/problems/minimum-domino-rotations-for-equal-row/
//
// In a row of dominoes, tops[i] and bottoms[i] represent the top and bottom halves of the ith domino. (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)
//
// We may rotate the ith domino, so that tops[i] and bottoms[i] swap values.
//
// Return the minimum number of rotations so that all the values in tops are the same, or all the values in bottoms are the same.
//
// If it cannot be done, return -1.
//
// Example 1:
//
// Input: tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
// Output: 2
// Explanation:
// The first figure represents the dominoes as given by tops and bottoms: before we do any rotations.
// If we rotate the second and fourth dominoes, we can make every value in the top row equal to 2, as indicated by the second figure.
//
// Example 2:
//
// Input: tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
// Output: -1
// Explanation:
// In this case, it is not possible to rotate the dominoes to make one row of values equal.
//
// Constraints:
//
// - 2 <= tops.length <= 2 * 10^4
// - bottoms.length == tops.length
// - 1 <= tops[i], bottoms[i] <= 6
//

struct Solution;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        'nums: for num in 1..=6 {
            let mut rotations = (0, 0);
            for domino in tops.iter().zip(bottoms.iter()) {
                match (*domino.0 == num, *domino.1 == num) {
                    (true, true) => {}
                    (true, false) => rotations.0 += 1,
                    (false, true) => rotations.1 += 1,
                    (false, false) => continue 'nums,
                }
            }
            return std::cmp::min(rotations.0, rotations.1);
        }
        -1
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2], 2),
        (vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4], -1),
    ];
    for (tops, bottoms, expected) in cases {
        assert_eq!(Solution::min_domino_rotations(tops, bottoms), expected);
    }
}
