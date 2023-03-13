#![allow(dead_code)]

/*

// 1494. Parallel Courses II
// https://leetcode.com/problems/parallel-courses-ii/
// https://leetcode.cn/problems/parallel-courses-ii/
//
// Hard
//
// You are given an integer n, which indicates that there are n courses labeled from 1 to n. You are also given an array relations where relations[i] = [prevCoursei, nextCoursei], representing a prerequisite relationship between course prevCoursei and course nextCoursei: course prevCoursei has to be taken before course nextCoursei. Also, you are given the integer k.

In one semester, you can take at most k courses as long as you have taken all the prerequisites in the previous semesters for the courses you are taking.

Return the minimum number of semesters needed to take all courses. The testcases will be generated such that it is possible to take every course.

Example 1:

Input: n = 4, relations = [[2,1],[3,1],[1,4]], k = 2
Output: 3
Explanation: The figure above represents the given graph.
In the first semester, you can take courses 2 and 3.
In the second semester, you can take course 1.
In the third semester, you can take course 4.

Example 2:

Input: n = 5, relations = [[2,1],[3,1],[4,1],[1,5]], k = 2
Output: 4
Explanation: The figure above represents the given graph.
In the first semester, you can only take courses 2 and 3 since you cannot take more than two per semester.
In the second semester, you can take course 4.
In the third semester, you can take course 1.
In the fourth semester, you can take course 5.

Constraints:

    1 <= n <= 15
    1 <= k <= n
    0 <= relations.length <= n * (n-1) / 2
    relations[i].length == 2
    1 <= prevCoursei, nextCoursei <= n
    prevCoursei != nextCoursei
    All the pairs [prevCoursei, nextCoursei] are unique.
    The given graph is a directed acyclic graph.
*/

struct Solution;

impl Solution {
    pub fn min_number_of_semesters(n: i32, relations: Vec<Vec<i32>>, k: i32) -> i32 {
        fn count_setbit(mask: i32) -> i32 {
            if mask == 0 {
                return 0;
            }
            1 + count_setbit(mask & (mask - 1))
        }

        let n = n as usize;
        let k = k as usize;
        let mut dependency = vec![0; n];
        for relation in relations {
            let course = (relation[1] - 1) as usize;
            let prerequisite = (relation[0] - 1) as usize;
            dependency[course] |= 1 << prerequisite;
        }

        let mut prerequisites = vec![0; 1 << n];
        for (i, prerequisite) in prerequisites.iter_mut().enumerate() {
            for (j, &item) in dependency.iter().enumerate() {
                if i & (1 << j) != 0 {
                    *prerequisite |= item;
                }
            }
        }

        let mut dp = vec![n + 1; 1 << n];
        dp[0] = 0;
        for i in 1..(1 << n) {
            let mut j = i;
            loop {
                if count_setbit(j as i32) as usize > k {
                    j = (j - 1) & i;
                    continue;
                }

                let already_taken = i ^ ((1 << n) - 1);
                if (already_taken & prerequisites[j]) == prerequisites[j] {
                    dp[i] = dp[i].min(dp[i ^ j] + 1);
                }

                if j == 0 {
                    break;
                }
                j = (j - 1) & i;
            }
        }

        dp[(1 << n) - 1] as _
    }
}

#[test]
fn test() {
    let cases = vec![
        (4, vec![vec![2, 1], vec![3, 1], vec![1, 4]], 2, 3),
        (5, vec![vec![2, 1], vec![3, 1], vec![4, 1], vec![1, 5]], 2, 4),
    ];
    for (n, relations, k, expected) in cases {
        assert_eq!(Solution::min_number_of_semesters(n, relations, k), expected);
    }
}
