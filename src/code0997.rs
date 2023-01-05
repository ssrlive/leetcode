#![allow(dead_code)]

// 997. Find the Town Judge
// https://leetcode.com/problems/find-the-town-judge/
// https://leetcode.cn/problems/find-the-town-judge/
//
// In a town, there are n people labeled from 1 to n. There is a rumor that one of these people is secretly the town judge.
//
// If the town judge exists, then:
//
// The town judge trusts nobody.
// Everybody (except for the town judge) trusts the town judge.
// There is exactly one person that satisfies properties 1 and 2.
// You are given an array trust where trust[i] = [ai, bi] representing that the person labeled ai trusts the person labeled bi.
//
// Return the label of the town judge if the town judge exists and can be identified, or return -1 otherwise.
//
// Example 1:
//
// Input: n = 2, trust = [[1,2]]
// Output: 2
//
// Example 2:
//
// Input: n = 3, trust = [[1,3],[2,3]]
// Output: 3
//
// Example 3:
//
// Input: n = 3, trust = [[1,3],[2,3],[3,1]]
// Output: -1
//
// Constraints:
//
// - 1 <= n <= 1000
// - 0 <= trust.length <= 10^4
// - trust[i].length == 2
// - All the pairs of trust are unique.
// - ai != bi
// - 1 <= ai, bi <= n
//

struct Solution;

impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashSet;
        const TRUSTER: usize = 0;
        const TRUSTED: usize = 1;
        if n == 1 {
            1
        } else {
            let n = n as usize;
            let mut no_trust: HashSet<usize> = HashSet::from_iter(1..=n);
            let mut trusted_by = vec![0; n + 1];
            for relation in trust {
                no_trust.remove(&(relation[TRUSTER] as usize));
                trusted_by[relation[TRUSTED] as usize] += 1;
            }
            for label in no_trust {
                if trusted_by[label] == n - 1 {
                    return label as i32;
                }
            }
            -1
        }
    }
}

#[test]
fn test() {
    let n = 2;
    let trust = vec![vec![1, 2]];
    assert_eq!(Solution::find_judge(n, trust), 2);

    let n = 3;
    let trust = vec![vec![1, 3], vec![2, 3]];
    assert_eq!(Solution::find_judge(n, trust), 3);

    let n = 3;
    let trust = vec![vec![1, 3], vec![2, 3], vec![3, 1]];
    assert_eq!(Solution::find_judge(n, trust), -1);

    let n = 4;
    let trust = vec![vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![4, 3]];
    assert_eq!(Solution::find_judge(n, trust), 3);
}
