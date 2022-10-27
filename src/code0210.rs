#![allow(dead_code)]

// 210. Course Schedule II
// https://leetcode.com/problems/course-schedule-ii/
//
// There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1.
// You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must
//  take course bi first if you want to take course ai.
//
// For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.
// Return the ordering of courses you should take to finish all courses. If there are many valid answers,
// return any of them. If it is impossible to finish all courses, return an empty array.
//
// Example 1:
//
// Input: numCourses = 2, prerequisites = [[1,0]]
// Output: [0,1]
// Explanation: There are a total of 2 courses to take. To take course 1 you should have finished course 0.
// So the correct course order is [0,1].
//
// Example 2:
//
// Input: numCourses = 4, prerequisites = [[1,0],[2,0],[3,1],[3,2]]
// Output: [0,2,1,3]
// Explanation: There are a total of 4 courses to take. To take course 3 you should have finished both courses 1 and 2.
// Both courses 1 and 2 should be taken after you finished course 0.
// So one correct course order is [0,1,2,3]. Another correct ordering is [0,2,1,3].
//
// Example 3:
//
// Input: numCourses = 1, prerequisites = []
// Output: [0]
//

struct Solution;

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![vec![]; num_courses as usize];
        let mut in_degree = vec![0; num_courses as usize];
        for edge in prerequisites {
            graph[edge[1] as usize].push(edge[0]);
            in_degree[edge[0] as usize] += 1;
        }
        let mut queue = Vec::new();
        for i in 0..num_courses {
            if in_degree[i as usize] == 0 {
                queue.push(i);
            }
        }
        let mut result = Vec::new();
        while !queue.is_empty() {
            let node = queue.remove(0);
            result.push(node);
            for &next in &graph[node as usize] {
                in_degree[next as usize] -= 1;
                if in_degree[next as usize] == 0 {
                    queue.push(next);
                }
            }
        }
        if result.len() == num_courses as usize {
            result
        } else {
            vec![]
        }
    }
}

#[test]
fn test() {
    assert_eq!(Solution::find_order(2, vec![vec![1, 0]]), vec![0, 1]);
    assert_eq!(
        Solution::find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]),
        vec![0, 1, 2, 3]
    );
    assert_eq!(Solution::find_order(1, vec![]), vec![0]);
}
