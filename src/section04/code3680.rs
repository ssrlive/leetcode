#![allow(dead_code)]

// 3680. Generate Schedule
// https://leetcode.com/problems/generate-schedule/
// https://leetcode.cn/problems/generate-schedule/
//
// Medium
//
// You are given an integer n representing n teams. You are asked to generate a schedule such that:
//
// Each team plays every other team exactly twice: once at home and once away.
// There is exactly one match per day; the schedule is a list of consecutive days and schedule[i] is the match on day i.
// No team plays on consecutive days.
// Return a 2D integer array schedule, where schedule[i][0] represents the home team and schedule[i][1] represents the away team. If multiple schedules meet the conditions, return any one of them.
//
// If no schedule exists that meets the conditions, return an empty array.
//
// Example 1:
//
// Input: n = 3
//
// Output: []
//
// Explanation:
//
// ​​​​​​​Since each team plays every other team exactly twice, a total of 6 matches need to be played: [0,1],[0,2],[1,2],[1,0],[2,0],[2,1].
//
// It's not possible to create a schedule without at least one team playing consecutive days.
//
// Example 2:
//
// Input: n = 5
//
// Output: [[0,1],[2,3],[0,4],[1,2],[3,4],[0,2],[1,3],[2,4],[0,3],[1,4],[2,0],[3,1],[4,0],[2,1],[4,3],[1,0],[3,2],[4,1],[3,0],[4,2]]
//
// Explanation:
//
// Since each team plays every other team exactly twice, a total of 20 matches need to be played.
//
// The output shows one of the schedules that meet the conditions. No team plays on consecutive days.
//
// Constraints:
//
// 2 <= n <= 50​​​​​​​
//

struct Solution;

impl Solution {
    pub fn generate_schedule(n: i32) -> Vec<Vec<i32>> {
        // For n <= 4 no valid schedule exists (problem statement example shows n=3 impossible).
        if n <= 4 {
            return vec![];
        }
        let total_matches = (n * (n - 1)) as usize; // each ordered pair once (home/away)
        let nu = n as usize;
        let mut matches = vec![vec![false; nu]; nu]; // matches[a][b] = played a home vs b away
        let mut games_played = vec![0; nu];
        let mut last1 = -1; // last day's home team
        let mut last2 = -1; // last day's away team
        let mut schedule: Vec<Vec<i32>> = Vec::with_capacity(total_matches);

        for _day in 0..total_matches {
            let mut best: Option<((i32, i32), i32)> = None;
            // deterministic iteration order to avoid HashSet nondeterminism & dead-ends
            for a in 0..n {
                if a == last1 || a == last2 {
                    continue;
                }
                for b in 0..n {
                    if b == a || b == last1 || b == last2 {
                        continue;
                    }
                    if matches[a as usize][b as usize] {
                        continue;
                    }
                    let score = games_played[a as usize] + games_played[b as usize];
                    match best {
                        Some((_, best_score)) if score >= best_score => { /* keep current best */ }
                        _ => {
                            best = Some(((a, b), score));
                        }
                    }
                }
            }
            if let Some(((a, b), _)) = best {
                matches[a as usize][b as usize] = true;
                schedule.push(vec![a, b]);
                games_played[a as usize] += 1;
                games_played[b as usize] += 1;
                last1 = a;
                last2 = b;
            } else {
                // No feasible next match without violating no-consecutive rule.
                return vec![]; // per problem: return empty if impossible
            }
        }
        schedule
    }
}

#[test]
fn test() {
    let n1 = 3;
    let result1 = Solution::generate_schedule(n1);
    assert!(result1.is_empty());

    let n2 = 5;
    let result2 = Solution::generate_schedule(n2);
    assert_eq!(result2.len(), 20);
}
