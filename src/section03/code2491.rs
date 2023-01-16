#![allow(dead_code)]

// 2491. Divide Players Into Teams of Equal Skill
// https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/
// https://leetcode.cn/problems/divide-players-into-teams-of-equal-skill/
//
// You are given a positive integer array skill of even length n where skill[i] denotes the skill of the ith player.
// Divide the players into n / 2 teams of size 2 such that the total skill of each team is equal.
//
// The chemistry of a team is equal to the product of the skills of the players on that team.
//
// Return the sum of the chemistry of all the teams, or return -1 if there is no way to divide the players into teams such that the total skill of each team is equal.
//
// Example 1:
//
// Input: skill = [3,2,5,1,3,4]
// Output: 22
// Explanation:
// Divide the players into the following teams: (1, 5), (2, 4), (3, 3), where each team has a total skill of 6.
// The sum of the chemistry of all the teams is: 1 * 5 + 2 * 4 + 3 * 3 = 5 + 8 + 9 = 22.
//
// Example 2:
//
// Input: skill = [3,4]
// Output: 12
// Explanation:
// The two players form a team with a total skill of 7.
// The chemistry of the team is 3 * 4 = 12.
//
// Example 3:
//
// Input: skill = [1,1,2,3]
// Output: -1
// Explanation:
// There is no way to divide the players into teams such that the total skill of each team is equal.
//
// Constraints:
//
// - 2 <= skill.length <= 105
// - skill.length is even.
// - 1 <= skill[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut skill = skill;
        skill.sort_unstable();
        let len = skill.len();
        let sum = skill[0] + skill[len - 1];
        let mut ans = 0;
        let mut left = 0;
        let mut right = len - 1;
        while left < right {
            if skill[left] + skill[right] != sum {
                return -1;
            }
            ans += i64::from(skill[left] * skill[right]);
            left += 1;
            right -= 1;
        }
        ans
    }
}

#[test]
fn test() {
    assert_eq!(Solution::divide_players(vec![3, 2, 5, 1, 3, 4]), 22);
    assert_eq!(Solution::divide_players(vec![3, 4]), 12);
    assert_eq!(Solution::divide_players(vec![1, 1, 2, 3]), -1);
}
