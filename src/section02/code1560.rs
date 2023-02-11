#![allow(dead_code)]

/*

// 1560. Most Visited Sector in a Circular Track
// https://leetcode.com/problems/most-visited-sector-in-a-circular-track/
// https://leetcode.cn/problems/most-visited-sector-in-a-circular-track/
//
// Easy
//
// Given an integer n and an integer array rounds. We have a circular track which consists of n sectors labeled from 1 to n. A marathon will be held on this track, the marathon consists of m rounds. The ith round starts at sector rounds[i - 1] and ends at sector rounds[i]. For example, round 1 starts at sector rounds[0] and ends at sector rounds[1]

Return an array of the most visited sectors sorted in ascending order.

Notice that you circulate the track in ascending order of sector numbers in the counter-clockwise direction (See the first example).

Example 1:

Input: n = 4, rounds = [1,3,1,2]
Output: [1,2]
Explanation: The marathon starts at sector 1. The order of the visited sectors is as follows:
1 --> 2 --> 3 (end of round 1) --> 4 --> 1 (end of round 2) --> 2 (end of round 3 and the marathon)
We can see that both sectors 1 and 2 are visited twice and they are the most visited sectors. Sectors 3 and 4 are visited only once.

Example 2:

Input: n = 2, rounds = [2,1,2,1,2,1,2,1,2]
Output: [2]

Example 3:

Input: n = 7, rounds = [1,3,5,7]
Output: [1,2,3,4,5,6,7]

Constraints:

    2 <= n <= 100
    1 <= m <= 100
    rounds.length == m + 1
    1 <= rounds[i] <= n
    rounds[i] != rounds[i + 1] for 0 <= i < m
*/

struct Solution;

impl Solution {
    pub fn most_visited(n: i32, rounds: Vec<i32>) -> Vec<i32> {
        let &start = rounds.first().unwrap();
        let &end = rounds.last().unwrap();
        if end >= start {
            (start..=end).collect()
        } else {
            (1..=end).chain(start..=n).collect()
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (4, vec![1, 3, 1, 2], vec![1, 2]),
        (2, vec![2, 1, 2, 1, 2, 1, 2, 1, 2], vec![2]),
        (7, vec![1, 3, 5, 7], vec![1, 2, 3, 4, 5, 6, 7]),
    ];
    for (n, rounds, expect) in cases {
        assert_eq!(Solution::most_visited(n, rounds), expect);
    }
}
