#![allow(dead_code)]

/*

// 2097. Valid Arrangement of Pairs
// https://leetcode.com/problems/valid-arrangement-of-pairs/
// https://leetcode.cn/problems/valid-arrangement-of-pairs/
//
// Hard
//
// You are given a 0-indexed 2D integer array pairs where pairs[i] = [starti, endi]. An arrangement of pairs is valid if for every index i where 1 <= i < pairs.length, we have endi-1 == starti.

Return any valid arrangement of pairs.

Note: The inputs will be generated such that there exists a valid arrangement of pairs.

Example 1:

Input: pairs = [[5,1],[4,5],[11,9],[9,4]]
Output: [[11,9],[9,4],[4,5],[5,1]]
Explanation:
This is a valid arrangement since endi-1 always equals starti.
end0 = 9 == 9 = start1
end1 = 4 == 4 = start2
end2 = 5 == 5 = start3

Example 2:

Input: pairs = [[1,3],[3,2],[2,1]]
Output: [[1,3],[3,2],[2,1]]
Explanation:
This is a valid arrangement since endi-1 always equals starti.
end0 = 3 == 3 = start1
end1 = 2 == 2 = start2
The arrangements [[2,1],[1,3],[3,2]] and [[3,2],[2,1],[1,3]] are also valid.

Example 3:

Input: pairs = [[1,2],[1,3],[2,1]]
Output: [[1,2],[2,1],[1,3]]
Explanation:
This is a valid arrangement since endi-1 always equals starti.
end0 = 2 == 2 = start1
end1 = 1 == 1 = start2

Constraints:

    1 <= pairs.length <= 10^5
    pairs[i].length == 2
    0 <= starti, endi <= 10^9
    starti != endi
    No two pairs are exactly the same.
    There exists a valid arrangement of pairs.
*/

struct Solution;

impl Solution {
    pub fn valid_arrangement(pairs: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::{HashMap, VecDeque};
        let mut graph = HashMap::<i32, Vec<i32>>::new();
        let (mut in_degree, mut out_degree) = (HashMap::<i32, i32>::new(), HashMap::<i32, i32>::new());

        for p in pairs {
            let (u, v) = (p[0], p[1]);
            graph.entry(u).or_default().push(v);
            *in_degree.entry(v).or_insert(0) += 1;
            *out_degree.entry(u).or_insert(0) += 1;
        }

        let mut start = *graph.keys().next().unwrap();
        for (key, val) in &out_degree {
            if let Some(val1) = in_degree.get(key) {
                if *val <= *val1 {
                    continue;
                }
            }
            start = *key;
            break;
        }

        let mut q = VecDeque::new();
        let mut sk = vec![];

        q.push_back(start);
        while let Some(u) = q.back() {
            let u = *u;
            if !graph.contains_key(&u) {
                sk.push(u);
                q.pop_back();
                continue;
            }
            if let Some(v) = graph.get_mut(&u).unwrap().pop() {
                q.push_back(v);
            }
            if graph.get(&u).unwrap().is_empty() {
                graph.remove(&u);
            }
        }

        let mut ret = vec![];
        for i in (1..sk.len()).rev() {
            ret.push(vec![sk[i], sk[i - 1]]);
        }

        ret
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![5, 1], vec![4, 5], vec![11, 9], vec![9, 4]],
            vec![vec![4, 5], vec![5, 1], vec![9, 4], vec![11, 9]],
        ),
        (vec![vec![1, 3], vec![3, 2], vec![2, 1]], vec![vec![1, 3], vec![2, 1], vec![3, 2]]),
        (vec![vec![1, 2], vec![1, 3], vec![2, 1]], vec![vec![1, 2], vec![1, 3], vec![2, 1]]),
    ];
    for (pairs, expected) in cases {
        let mut v = Solution::valid_arrangement(pairs);
        v.sort();
        assert_eq!(v, expected);
    }
}
