#![allow(dead_code)]

/*

// 2225. Find Players With Zero or One Losses
// https://leetcode.com/problems/find-players-with-zero-or-one-losses/
// https://leetcode.cn/problems/find-players-with-zero-or-one-losses/
//
// Medium
//
// You are given an integer array matches where matches[i] = [winneri, loseri] indicates that the player winneri defeated player loseri in a match.

Return a list answer of size 2 where:

    answer[0] is a list of all players that have not lost any matches.
    answer[1] is a list of all players that have lost exactly one match.

The values in the two lists should be returned in increasing order.

Note:

    You should only consider the players that have played at least one match.
    The testcases will be generated such that no two matches will have the same outcome.

Example 1:

Input: matches = [[1,3],[2,3],[3,6],[5,6],[5,7],[4,5],[4,8],[4,9],[10,4],[10,9]]
Output: [[1,2,10],[4,5,7,8]]
Explanation:
Players 1, 2, and 10 have not lost any matches.
Players 4, 5, 7, and 8 each have lost one match.
Players 3, 6, and 9 each have lost two matches.
Thus, answer[0] = [1,2,10] and answer[1] = [4,5,7,8].

Example 2:

Input: matches = [[2,3],[1,3],[5,4],[6,4]]
Output: [[1,2,5,6],[]]
Explanation:
Players 1, 2, 5, and 6 have not lost any matches.
Players 3 and 4 each have lost two matches.
Thus, answer[0] = [1,2,5,6] and answer[1] = [].

Constraints:

    1 <= matches.length <= 10^5
    matches[i].length == 2
    1 <= winneri, loseri <= 10^5
    winneri != loseri
    All matches[i] are unique.
*/

struct Solution;

impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;
        let mut losses = HashMap::new();
        for i in matches {
            *losses.entry(i[1]).or_insert(0) += 1;
            losses.entry(i[0]).or_insert(0);
        }
        let mut zero_losses = Vec::new();
        let mut one_loss = Vec::new();
        for i in losses {
            if i.1 == 0 {
                zero_losses.push(i.0);
            } else if i.1 == 1 {
                one_loss.push(i.0);
            }
        }
        zero_losses.sort_unstable();
        one_loss.sort_unstable();
        vec![zero_losses, one_loss]
    }
}

#[test]
fn test() {
    let matches = vec![
        vec![1, 3],
        vec![2, 3],
        vec![3, 6],
        vec![5, 6],
        vec![5, 7],
        vec![4, 5],
        vec![4, 8],
        vec![4, 9],
        vec![10, 4],
        vec![10, 9],
    ];
    let result = vec![vec![1, 2, 10], vec![4, 5, 7, 8]];
    assert_eq!(Solution::find_winners(matches), result);
    let matches = vec![vec![2, 3], vec![1, 3], vec![5, 4], vec![6, 4]];
    let result = vec![vec![1, 2, 5, 6], Vec::new()];
    assert_eq!(Solution::find_winners(matches), result);
}
