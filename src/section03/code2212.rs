#![allow(dead_code)]

/*

// 2212. Maximum Points in an Archery Competition
// https://leetcode.com/problems/maximum-points-in-an-archery-competition/
// https://leetcode.cn/
//
// Medium
//
// Alice and Bob are opponents in an archery competition. The competition has set the following rules:

    Alice first shoots numArrows arrows and then Bob shoots numArrows arrows.
    The points are then calculated as follows:
        The target has integer scoring sections ranging from 0 to 11 inclusive.
        For each section of the target with score k (in between 0 to 11), say Alice and Bob have shot ak and bk arrows on that section respectively. If ak >= bk, then Alice takes k points. If ak < bk, then Bob takes k points.
        However, if ak == bk == 0, then nobody takes k points.

    For example, if Alice and Bob both shot 2 arrows on the section with score 11, then Alice takes 11 points. On the other hand, if Alice shot 0 arrows on the section with score 11 and Bob shot 2 arrows on that same section, then Bob takes 11 points.

You are given the integer numArrows and an integer array aliceArrows of size 12, which represents the number of arrows Alice shot on each scoring section from 0 to 11. Now, Bob wants to maximize the total number of points he can obtain.

Return the array bobArrows which represents the number of arrows Bob shot on each scoring section from 0 to 11. The sum of the values in bobArrows should equal numArrows.

If there are multiple ways for Bob to earn the maximum total points, return any one of them.

Example 1:

Input: numArrows = 9, aliceArrows = [1,1,0,1,0,0,2,1,0,1,2,0]
Output: [0,0,0,0,1,1,0,0,1,2,3,1]
Explanation: The table above shows how the competition is scored.
Bob earns a total point of 4 + 5 + 8 + 9 + 10 + 11 = 47.
It can be shown that Bob cannot obtain a score higher than 47 points.

Example 2:

Input: numArrows = 3, aliceArrows = [0,0,1,0,0,0,0,0,0,0,0,2]
Output: [0,0,0,0,0,0,0,0,1,1,1,0]
Explanation: The table above shows how the competition is scored.
Bob earns a total point of 8 + 9 + 10 = 27.
It can be shown that Bob cannot obtain a score higher than 27 points.

Constraints:

    1 <= numArrows <= 10^5
    aliceArrows.length == bobArrows.length == 12
    0 <= aliceArrows[i], bobArrows[i] <= numArrows
    sum(aliceArrows[i]) == numArrows
*/

struct Solution;

impl Solution {
    pub fn maximum_bob_points(num_arrows: i32, alice_arrows: Vec<i32>) -> Vec<i32> {
        let limit = 1 << 12;
        let mut max = 0;
        let mut result = vec![0; 12];
        for i in 0..limit {
            let mut temp = 0;
            let mut score = 0;
            let mut memo = vec![0; 12];
            for j in 0..12 {
                if (i >> j) & 1 == 1 {
                    temp += alice_arrows[j] + 1;
                    memo[j] = alice_arrows[j] + 1;
                    score += j as i32;
                }
            }
            if max < score && temp <= num_arrows {
                max = score;
                result = memo;
            }
        }
        let mut tot = 0;
        for &result_i in result.iter() {
            tot += result_i;
        }
        result[11] += num_arrows - tot;
        result
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            9,
            vec![1, 1, 0, 1, 0, 0, 2, 1, 0, 1, 2, 0],
            vec![0, 0, 0, 0, 1, 1, 0, 0, 1, 2, 3, 1],
        ),
        (
            3,
            vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 2],
            vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0],
        ),
    ];
    for (num_arrows, alice_arrows, expected) in cases {
        assert_eq!(Solution::maximum_bob_points(num_arrows, alice_arrows), expected);
    }
}
