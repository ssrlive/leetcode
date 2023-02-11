#![allow(dead_code)]

/*

// 1552. Magnetic Force Between Two Balls
// https://leetcode.com/problems/magnetic-force-between-two-balls/
// https://leetcode.cn/problems/magnetic-force-between-two-balls/
//
// Medium
//
// In the universe Earth C-137, Rick discovered a special form of magnetic force between two balls if they are put in his new invented basket. Rick has n empty baskets, the ith basket is at position[i], Morty has m balls and needs to distribute the balls into the baskets such that the minimum magnetic force between any two balls is maximum.

Rick stated that magnetic force between two different balls at positions x and y is |x - y|.

Given the integer array position and the integer m. Return the required force.

Example 1:

Input: position = [1,2,3,4,7], m = 3
Output: 3
Explanation: Distributing the 3 balls into baskets 1, 4 and 7 will make the magnetic force between ball pairs [3, 3, 6]. The minimum magnetic force is 3. We cannot achieve a larger minimum magnetic force than 3.

Example 2:

Input: position = [5,4,3,2,1,1000000000], m = 2
Output: 999999999
Explanation: We can use baskets 1 and 1000000000.

Constraints:

    n == position.length
    2 <= n <= 10^5
    1 <= position[i] <= 10^9
    All integers in position are distinct.
    2 <= m <= position.length
*/

struct Solution;

impl Solution {
    pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        let mut position = position;
        position.sort();
        let mut left = 1;
        let mut right = position[position.len() - 1] - position[0];
        while left < right {
            let mid = (left + right + 1) / 2;
            let mut count = 1;
            let mut last = position[0];
            for &pos in position.iter().skip(1) {
                if pos - last >= mid {
                    count += 1;
                    last = pos;
                }
            }
            if count >= m {
                left = mid;
            } else {
                right = mid - 1;
            }
        }
        left
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4, 7], 3, 3),
        (vec![5, 4, 3, 2, 1, 1000000000], 2, 999999999),
    ];
    for (position, m, expected) in cases {
        assert_eq!(Solution::max_distance(position, m), expected);
    }
}
