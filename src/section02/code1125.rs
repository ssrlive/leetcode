#![allow(dead_code)]

// 1125. Smallest Sufficient Team
// https://leetcode.com/problems/smallest-sufficient-team/
// https://leetcode.cn/problems/smallest-sufficient-team/
//
// In a project, you have a list of required skills req_skills, and a list of people.
// The ith person people[i] contains a list of skills that the person has.
//
// Consider a sufficient team: a set of people such that for every required skill in req_skills,
// there is at least one person in the team who has that skill. We can represent these teams by the index of each person.
//
// For example, team = [0, 1, 3] represents the people with skills people[0], people[1], and people[3].
// Return any sufficient team of the smallest possible size, represented by the index of each person. You may return the answer in any order.
//
// It is guaranteed an answer exists.
//
// Example 1:
//
// Input: req_skills = ["java","nodejs","reactjs"], people = [["java"],["nodejs"],["nodejs","reactjs"]]
// Output: [0,2]
//
// Example 2:
//
// Input: req_skills = ["algorithms","math","java","reactjs","csharp","aws"], people = [["algorithms","math","java"],["algorithms","math","reactjs"],["java","csharp","aws"],["reactjs","csharp"],["csharp","math"],["aws","java"]]
// Output: [1,2]
//
// Constraints:
//
// - 1 <= req_skills.length <= 16
// - 1 <= req_skills[i].length <= 16
// - req_skills[i] consists of lowercase English letters.
// - All the strings of req_skills are unique.
// - 1 <= people.length <= 60
// - 0 <= people[i].length <= 16
// - 1 <= people[i][j].length <= 16
// - people[i][j] consists of lowercase English letters.
// - All the strings of people[i] are unique.
// - Every skill in people[i] is a skill in req_skills.
// - It is guaranteed a sufficient team exists.
//

struct Solution;

impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        let mut skills = std::collections::HashMap::new();
        for (i, skill) in req_skills.iter().enumerate() {
            skills.insert(skill, i);
        }
        let mut people_skill = vec![0; people.len()];
        for (i, person) in people.iter().enumerate() {
            for skill in person {
                let temp = *skills.get(skill).unwrap();
                people_skill[i] |= 1 << temp;
            }
        }
        let s = 1 << req_skills.len();
        let mut dp = vec![std::i32::MAX; s];
        let mut parent = vec![-1; s];
        let mut parent_state = vec![0; s];
        dp[0] = 0;
        for i in 0..(1 << req_skills.len()) {
            for (j, &item) in people_skill.iter().enumerate() {
                if dp[i] == std::i32::MAX {
                    continue;
                }
                let temp = i | item;
                if dp[temp] > dp[i] + 1 {
                    parent[temp] = j as i32;
                    parent_state[temp] = i;
                    dp[temp] = dp[i] + 1;
                }
            }
        }
        let mut temp = (1 << req_skills.len()) - 1;
        let mut ret = vec![];
        while parent[temp] != -1 {
            ret.push(parent[temp]);
            temp = parent_state[temp];
        }
        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec!["java", "nodejs", "reactjs"],
            vec![vec!["java"], vec!["nodejs"], vec!["nodejs", "reactjs"]],
            vec![2, 0],
        ),
        (
            vec!["algorithms", "math", "java", "reactjs", "csharp", "aws"],
            vec![
                vec!["algorithms", "math", "java"],
                vec!["algorithms", "math", "reactjs"],
                vec!["java", "csharp", "aws"],
                vec!["reactjs", "csharp"],
                vec!["csharp", "math"],
                vec!["aws", "java"],
            ],
            vec![2, 1],
        ),
    ];
    for (req_skills, people, expected) in cases {
        let req_skills = req_skills.iter().map(|s| s.to_string()).collect();
        let people = people.iter().map(|person| person.iter().map(|s| s.to_string()).collect()).collect();
        assert_eq!(Solution::smallest_sufficient_team(req_skills, people), expected);
    }
}
