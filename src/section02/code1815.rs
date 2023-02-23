#![allow(dead_code)]

/*

// 1815. Maximum Number of Groups Getting Fresh Donuts
// https://leetcode.com/problems/maximum-number-of-groups-getting-fresh-donuts/
// https://leetcode.cn/problems/maximum-number-of-groups-getting-fresh-donuts/
//
// Hard
//
// There is a donuts shop that bakes donuts in batches of batchSize. They have a rule where they must serve all of the donuts of a batch before serving any donuts of the next batch. You are given an integer batchSize and an integer array groups, where groups[i] denotes that there is a group of groups[i] customers that will visit the shop. Each customer will get exactly one donut.

When a group visits the shop, all customers of the group must be served before serving any of the following groups. A group will be happy if they all get fresh donuts. That is, the first customer of the group does not receive a donut that was left over from the previous group.

You can freely rearrange the ordering of the groups. Return the maximum possible number of happy groups after rearranging the groups.

Example 1:

Input: batchSize = 3, groups = [1,2,3,4,5,6]
Output: 4
Explanation: You can arrange the groups as [6,2,4,5,1,3]. Then the 1st, 2nd, 4th, and 6th groups will be happy.

Example 2:

Input: batchSize = 4, groups = [1,3,2,5,2,2,1,6]
Output: 4

Constraints:

    1 <= batchSize <= 9
    1 <= groups.length <= 30
    1 <= groups[i] <= 10^9
*/

struct Solution;

impl Solution {
    pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        fn dfs(cnt: &mut Vec<i32>, left: i32, dp: &mut HashMap<Vec<i32>, i32>) -> i32 {
            if let Some(&res) = dp.get(cnt) {
                return res;
            }
            let mut res = 0;
            let bz = cnt.len() as i32;
            for j in 1..bz {
                cnt[j as usize] -= 1;
                if cnt[j as usize] >= 0 {
                    res = res.max((left == 0) as i32 + dfs(cnt, (bz + left - j) % bz, dp));
                }
                cnt[j as usize] += 1;
            }
            dp.insert(cnt.clone(), res);
            res
        }

        let mut dp = HashMap::new();
        let mut cnt = vec![0; batch_size as usize];
        let mut res = 0;
        for group in groups {
            if group % batch_size == 0 {
                res += 1;
            } else if cnt[(batch_size - group % batch_size) as usize] > 0 {
                cnt[(batch_size - group % batch_size) as usize] -= 1;
                res += 1;
            } else {
                cnt[(group % batch_size) as usize] += 1;
            }
        }
        res + dfs(&mut cnt, 0, &mut dp)
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (3, vec![1, 2, 3, 4, 5, 6], 4), 
        (4, vec![1, 3, 2, 5, 2, 2, 1, 6], 4)
    ];
    for (batch_size, groups, expected) in cases {
        assert_eq!(Solution::max_happy_groups(batch_size, groups), expected);
    }
}
