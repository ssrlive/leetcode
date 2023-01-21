#![allow(dead_code)]

// 210. Course Schedule II
// https://leetcode.com/problems/course-schedule-ii/
// https://leetcode.cn/problems/course-schedule-ii/
//
// Medium
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

/*
 * 1. 通过邻接表构建图
 * 2. 统计每个节点的入度
 * 3. 将入度为 0 的节点入队
 * 4. 从队列中取出节点，将其入度减 1，如果减 1 后入度为 0，则将其邻接节点入队
 * 5. 重复 4，直到队列为空
 * 6. 如果结果长度等于课程数，则返回结果，否则返回空数组
*/

impl Solution {
    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let num_courses = num_courses as usize;
        let mut graph = vec![vec![]; num_courses];
        let mut in_degree = vec![0; num_courses];
        for edge in prerequisites {
            let pre = edge[1] as usize;
            let next = edge[0] as usize;
            graph[pre].push(next);
            in_degree[next] += 1;
        }
        let mut queue = std::collections::VecDeque::new();
        for (i, &item) in in_degree.iter().enumerate().take(num_courses) {
            if item == 0 {
                queue.push_back(i);
            }
        }
        let mut result = Vec::new();
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            result.push(node);
            for &next in &graph[node] {
                let item = in_degree.get_mut(next).unwrap();
                *item -= 1;
                if *item == 0 {
                    queue.push_back(next);
                }
            }
        }
        if result.len() == num_courses {
            result.iter().map(|&x| x as i32).collect()
        } else {
            vec![]
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (2, vec![vec![1, 0]], vec![0, 1]),
        (
            4,
            vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]],
            vec![0, 1, 2, 3],
        ),
        (1, vec![], vec![0]),
    ];
    for (num_courses, prerequisites, expected) in cases {
        assert_eq!(Solution::find_order(num_courses, prerequisites), expected);
    }
}
