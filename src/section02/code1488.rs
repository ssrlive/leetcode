#![allow(dead_code)]

/*

// 1488. Avoid Flood in The City
// https://leetcode.com/problems/avoid-flood-in-the-city/
// https://leetcode.cn/problems/avoid-flood-in-the-city/
//
// Medium
//
Your country has an infinite number of lakes. Initially, all the lakes are empty, but when it rains over the nth lake, the nth lake becomes full of water. If it rains over a lake that is full of water, there will be a flood. Your goal is to avoid floods in any lake.

Given an integer array rains where:

    rains[i] > 0 means there will be rains over the rains[i] lake.
    rains[i] == 0 means there are no rains this day and you can choose one lake this day and dry it.

Return an array ans where:

    ans.length == rains.length
    ans[i] == -1 if rains[i] > 0.
    ans[i] is the lake you choose to dry in the ith day if rains[i] == 0.

If there are multiple valid answers return any of them. If it is impossible to avoid flood return an empty array.

Notice that if you chose to dry a full lake, it becomes empty, but if you chose to dry an empty lake, nothing changes.

Example 1:

Input: rains = [1,2,3,4]
Output: [-1,-1,-1,-1]
Explanation: After the first day full lakes are [1]
After the second day full lakes are [1,2]
After the third day full lakes are [1,2,3]
After the fourth day full lakes are [1,2,3,4]
There's no day to dry any lake and there is no flood in any lake.

Example 2:

Input: rains = [1,2,0,0,2,1]
Output: [-1,-1,2,1,-1,-1]
Explanation: After the first day full lakes are [1]
After the second day full lakes are [1,2]
After the third day, we dry lake 2. Full lakes are [1]
After the fourth day, we dry lake 1. There is no full lakes.
After the fifth day, full lakes are [2].
After the sixth day, full lakes are [1,2].
It is easy that this scenario is flood-free. [-1,-1,1,2,-1,-1] is another acceptable scenario.

Example 3:

Input: rains = [1,2,0,1,2]
Output: []
Explanation: After the second day, full lakes are  [1,2]. We have to dry one lake in the third day.
After that, it will rain over lakes [1,2]. It's easy to prove that no matter which lake you choose to dry in the 3rd day, the other one will flood.

Constraints:

    1 <= rains.length <= 10^5
    0 <= rains[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        use std::collections::{BTreeSet, HashMap};
        use std::ops::Bound::*;
        let mut dry_lakes = BTreeSet::new();
        let mut lakes = HashMap::new();
        let mut ans = vec![1; rains.len()];
        for (i, r) in rains.into_iter().enumerate() {
            if r == 0 {
                dry_lakes.insert(i);
                continue;
            }
            if lakes.contains_key(&r) {
                if let Some(j) = dry_lakes.range((Excluded(lakes[&r]), Excluded(i))).next().copied() {
                    ans[j] = r;
                    dry_lakes.remove(&j);
                } else {
                    return vec![];
                }
            }
            lakes.insert(r, i);
            ans[i] = -1;
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 2, 3, 4], vec![-1, -1, -1, -1]),
        (vec![1, 2, 0, 0, 2, 1], vec![-1, -1, 2, 1, -1, -1]),
        (vec![1, 2, 0, 1, 2], vec![]),
        (vec![69, 0, 0, 0, 69], vec![-1, 69, 1, 1, -1]),
        (vec![10, 20, 20], vec![]),
    ];
    for (rains, want) in cases {
        assert_eq!(Solution::avoid_flood(rains), want);
    }
}
