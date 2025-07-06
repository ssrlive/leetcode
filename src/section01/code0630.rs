#![allow(dead_code)]

// 630. Course Schedule III
// https://leetcode.com/problems/course-schedule-iii/
// https://leetcode.cn/problems/course-schedule-iii/
//
// Hard
//
// There are n different online courses numbered from 1 to n. You are given an array courses where courses[i] = [durationi, lastDayi]
// indicate that the ith course should be taken continuously for durationi days and must be finished before or on lastDayi.
//
// You will start on the 1st day and you cannot take two or more courses simultaneously.
//
// Return the maximum number of courses that you can take.
//
// Example 1:
//
// Input: courses = [[100,200],[200,1300],[1000,1250],[2000,3200]]
// Output: 3
// Explanation:
// There are totally 4 courses, but you can take 3 courses at most:
// First, take the 1st course, it costs 100 days so you will finish it on the 100th day, and ready to take the next course on the 101st day.
// Second, take the 3rd course, it costs 1000 days so you will finish it on the 1100th day, and ready to take the next course on the 1101st day.
// Third, take the 2nd course, it costs 200 days so you will finish it on the 1300th day.
// The 4th course cannot be taken now, since you will finish it on the 3300th day, which exceeds the closed date.
//
// Example 2:
//
// Input: courses = [[1,2]]
// Output: 1
//
// Example 3:
//
// Input: courses = [[3,2],[4,3]]
// Output: 0
//
// Constraints:
//
// - 1 <= courses.length <= 10^4
// - 1 <= durationi, lastDayi <= 10^4
//

struct Solution;

impl Solution {
    pub fn schedule_course(courses: Vec<Vec<i32>>) -> i32 {
        let mut courses = courses;
        courses.sort_by_key(|c| c[1]);
        let mut heap = std::collections::BinaryHeap::new();
        let mut time = 0;
        for c in courses {
            if time + c[0] <= c[1] {
                time += c[0];
                heap.push(c[0]);
            } else if let Some(&t) = heap.peek()
                && t > c[0]
            {
                time += c[0] - t;
                heap.pop();
                heap.push(c[0]);
            }
        }
        heap.len() as i32
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::schedule_course(vec![vec![100, 200], vec![200, 1300], vec![1000, 1250], vec![2000, 3200]]),
        3
    );
    assert_eq!(Solution::schedule_course(vec![vec![1, 2]]), 1);
    assert_eq!(Solution::schedule_course(vec![vec![3, 2], vec![4, 3]]), 0);
}
