#![allow(dead_code)]

// 621. Task Scheduler
// https://leetcode.com/problems/task-scheduler/
// https://leetcode.cn/problems/task-scheduler/
//
// Given a characters array tasks, representing the tasks a CPU needs to do, where each letter represents a different task.
// Tasks could be done in any order. Each task is done in one unit of time.
// For each unit of time, the CPU could complete either one task or just be idle.
//
// However, there is a non-negative integer n that represents the cooldown period between two same tasks (the same letter in the array),
// that is that there must be at least n units of time between any two same tasks.
//
// Return the least number of units of times that the CPU will take to finish all the given tasks.
//
// Example 1:
//
// Input: tasks = ["A","A","A","B","B","B"], n = 2
// Output: 8
// Explanation:
// A -> B -> idle -> A -> B -> idle -> A -> B
// There is at least 2 units of time between any two same tasks.
//
// Example 2:
//
// Input: tasks = ["A","A","A","B","B","B"], n = 0
// Output: 6
// Explanation: On this case any permutation of size 6 would work since n = 0.
// ["A","A","A","B","B","B"]
// ["A","B","A","B","A","B"]
// ["B","B","B","A","A","A"]
// ...
// And so on.
//
// Example 3:
//
// Input: tasks = ["A","A","A","A","A","A","B","C","D","E","F","G"], n = 2
// Output: 16
// Explanation:
// One possible solution is
// A -> B -> C -> A -> D -> E -> A -> F -> G -> A -> idle -> idle -> A -> idle -> idle -> A
//
// Constraints:
//
// - 1 <= task.length <= 10^4
// - tasks[i] is upper-case English letter.
// - The integer n is in the range [0, 100].
//

struct Solution;

impl Solution {
    pub fn least_interval(tasks: Vec<char>, n: i32) -> i32 {
        let mut task_count = [0; 26];
        for &task in tasks.iter() {
            task_count[task as usize - 'A' as usize] += 1;
        }

        let mut max_count = 0;
        let mut max_count_count = 0;
        for &count in task_count.iter() {
            match count.cmp(&max_count) {
                std::cmp::Ordering::Greater => {
                    max_count = count;
                    max_count_count = 1;
                }
                std::cmp::Ordering::Equal => {
                    max_count_count += 1;
                }
                _ => {}
            }
        }

        let mut result = (max_count - 1) * (n + 1) + max_count_count;
        if result < tasks.len() as i32 {
            result = tasks.len() as i32;
        }

        result
    }
}

#[test]
fn test() {
    assert_eq!(
        Solution::least_interval(vec!['A', 'A', 'A', 'A', 'A', 'A', 'B', 'C', 'D', 'E', 'F', 'G'], 2),
        16
    );
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 2), 8);
    assert_eq!(Solution::least_interval(vec!['A', 'A', 'A', 'B', 'B', 'B'], 0), 6);
}
