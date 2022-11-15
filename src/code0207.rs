#![allow(dead_code)]

// 207. Course Schedule
// https://leetcode.com/problems/course-schedule/
//
// There are a total of numCourses courses you have to take, labeled from 0 to numCourses-1.
// You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you
// must take course bi first if you want to take course ai.
//
// For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
// Return true if you can finish all courses. Otherwise, return false.
//

struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        fn has_cycle(
            i: i32,
            graph: &std::collections::HashMap<i32, Vec<i32>>,
            visited: &mut Vec<bool>,
            visiting: &mut Vec<bool>,
        ) -> bool {
            if visiting[i as usize] {
                return true;
            }
            if visited[i as usize] {
                return false;
            }
            visiting[i as usize] = true;
            if let Some(neighbors) = graph.get(&i) {
                for &n in neighbors {
                    if has_cycle(n, graph, visited, visiting) {
                        return true;
                    }
                }
            }
            visiting[i as usize] = false;
            visited[i as usize] = true;
            false
        }

        let mut graph = std::collections::HashMap::<i32, Vec<i32>>::new();
        for p in prerequisites {
            graph.entry(p[0]).or_default().push(p[1]);
        }
        let mut visited = vec![false; num_courses as usize];
        let mut visiting = vec![false; num_courses as usize];
        for i in 0..num_courses {
            if !visited[i as usize] && has_cycle(i, &graph, &mut visited, &mut visiting) {
                return false;
            }
        }
        true
    }
}

#[test]
fn test_can_finish() {
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
    assert_eq!(
        Solution::can_finish(3, vec![vec![1, 0], vec![1, 2], vec![0, 1]]),
        false
    );
    assert_eq!(
        Solution::can_finish(3, vec![vec![1, 0], vec![2, 0], vec![0, 1]]),
        false
    );
    assert_eq!(
        Solution::can_finish(3, vec![vec![1, 0], vec![2, 0], vec![0, 2]]),
        false
    );
    assert_eq!(
        Solution::can_finish(3, vec![vec![1, 0], vec![2, 0], vec![0, 1], vec![2, 1]]),
        false
    );
    assert_eq!(
        Solution::can_finish(
            3,
            vec![vec![1, 0], vec![2, 0], vec![0, 1], vec![2, 1], vec![1, 2]]
        ),
        false
    );
    assert_eq!(
        Solution::can_finish(
            3,
            vec![
                vec![1, 0],
                vec![2, 0],
                vec![0, 1],
                vec![2, 1],
                vec![1, 2],
                vec![2, 1]
            ]
        ),
        false
    );
    assert_eq!(
        Solution::can_finish(
            3,
            vec![
                vec![1, 0],
                vec![2, 0],
                vec![0, 1],
                vec![2, 1],
                vec![1, 2],
                vec![2, 1],
                vec![1, 0]
            ]
        ),
        false
    );
    assert_eq!(
        Solution::can_finish(
            3,
            vec![
                vec![1, 0],
                vec![2, 0],
                vec![0, 1],
                vec![2, 1],
                vec![1, 2],
                vec![2, 1],
                vec![1, 0],
                vec![0, 2]
            ]
        ),
        false
    );
    assert_eq!(
        Solution::can_finish(
            3,
            vec![
                vec![1, 0],
                vec![2, 0],
                vec![0, 1],
                vec![2, 1],
                vec![1, 2],
                vec![2, 1],
                vec![1, 0],
                vec![0, 2],
                vec![2, 0]
            ]
        ),
        false
    );
}
