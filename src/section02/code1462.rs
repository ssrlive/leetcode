#![allow(dead_code)]

/*
// 1462. Course Schedule IV
Medium

There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course ai first if you want to take course bi.

    For example, the pair [0, 1] indicates that you have to take course 0 before you can take course 1.

Prerequisites can also be indirect. If course a is a prerequisite of course b, and course b is a prerequisite of course c, then course a is a prerequisite of course c.

You are also given an array queries where queries[j] = [uj, vj]. For the jth query, you should answer whether course uj is a prerequisite of course vj or not.

Return a boolean array answer, where answer[j] is the answer to the jth query.

Example 1:

Input: numCourses = 2, prerequisites = [[1,0]], queries = [[0,1],[1,0]]
Output: [false,true]
Explanation: The pair [1, 0] indicates that you have to take course 1 before you can take course 0.
Course 0 is not a prerequisite of course 1, but the opposite is true.

Example 2:

Input: numCourses = 2, prerequisites = [], queries = [[1,0],[0,1]]
Output: [false,false]
Explanation: There are no prerequisites, and each course is independent.

Example 3:

Input: numCourses = 3, prerequisites = [[1,2],[1,0],[2,0]], queries = [[1,0],[1,2]]
Output: [true,true]

Constraints:

    2 <= numCourses <= 100
    0 <= prerequisites.length <= (numCourses * (numCourses - 1) / 2)
    prerequisites[i].length == 2
    0 <= ai, bi <= n - 1
    ai != bi
    All the pairs [ai, bi] are unique.
    The prerequisites graph has no cycles.
    1 <= queries.length <= 10^4
    0 <= ui, vi <= n - 1
    ui != vi
*/

struct Solution;

impl Solution {
    pub fn check_if_prerequisite(num_courses: i32, prerequisites: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let n = num_courses as usize;
        let mut g = vec![vec![]; n];
        for p in &prerequisites {
            g[p[0] as usize].push(p[1] as usize);
        }
        let mut c = vec![false; n];
        let mut pre = vec![vec![false; n]; n];
        for i in 0..n {
            if !c[i] {
                Solution::search(i, &g, &mut c, &mut pre);
            }
        }
        let mut res = vec![];
        for q in &queries {
            res.push(pre[q[0] as usize][q[1] as usize]);
        }
        res
    }

    fn search(u: usize, g: &[Vec<usize>], c: &mut [bool], pre: &mut [Vec<bool>]) {
        for v in &g[u] {
            if !c[*v] {
                Solution::search(*v, g, c, pre);
            }
            pre[u][*v] = true;
            let (pre_u, pre_v) = if u < *v {
                let (left, right) = pre.split_at_mut(*v);
                (&mut left[u], &right[0])
            } else {
                let (left, right) = pre.split_at_mut(u);
                (&mut right[0], &left[*v])
            };
            for (pu, &pv) in pre_u.iter_mut().zip(pre_v.iter()) {
                *pu |= pv;
            }
        }
        c[u] = true;
    }
}

#[test]
fn test() {
    let cases = vec![
        (2, vec![vec![1, 0]], vec![vec![0, 1], vec![1, 0]], vec![false, true]),
        (2, vec![], vec![vec![1, 0], vec![0, 1]], vec![false, false]),
        (
            3,
            vec![vec![1, 2], vec![1, 0], vec![2, 0]],
            vec![vec![1, 0], vec![1, 2]],
            vec![true, true],
        ),
    ];
    for (n, prerequisites, queries, expected) in cases {
        assert_eq!(Solution::check_if_prerequisite(n, prerequisites, queries), expected);
    }
}
