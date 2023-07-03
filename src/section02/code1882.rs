#![allow(dead_code)]

/*

// 1882. Process Tasks Using Servers
// https://leetcode.com/problems/process-tasks-using-servers/
// https://leetcode.cn/problems/process-tasks-using-servers/
//
// Medium
//
// You are given two 0-indexed integer arrays servers and tasks of lengths n​​​​​​ and m​​​​​​ respectively. servers[i] is the weight of the i​​​​​​th​​​​ server, and tasks[j] is the time needed to process the j​​​​​​th​​​​ task in seconds.

Tasks are assigned to the servers using a task queue. Initially, all servers are free, and the queue is empty.

At second j, the jth task is inserted into the queue (starting with the 0th task being inserted at second 0). As long as there are free servers and the queue is not empty, the task in the front of the queue will be assigned to a free server with the smallest weight, and in case of a tie, it is assigned to a free server with the smallest index.

If there are no free servers and the queue is not empty, we wait until a server becomes free and immediately assign the next task. If multiple servers become free at the same time, then multiple tasks from the queue will be assigned in order of insertion following the weight and index priorities above.

A server that is assigned task j at second t will be free again at second t + tasks[j].

Build an array ans​​​​ of length m, where ans[j] is the index of the server the j​​​​​​th task will be assigned to.

Return the array ans​​​​.

Example 1:

Input: servers = [3,3,2], tasks = [1,2,3,2,1,2]
Output: [2,2,0,2,1,2]
Explanation: Events in chronological order go as follows:
- At second 0, task 0 is added and processed using server 2 until second 1.
- At second 1, server 2 becomes free. Task 1 is added and processed using server 2 until second 3.
- At second 2, task 2 is added and processed using server 0 until second 5.
- At second 3, server 2 becomes free. Task 3 is added and processed using server 2 until second 5.
- At second 4, task 4 is added and processed using server 1 until second 5.
- At second 5, all servers become free. Task 5 is added and processed using server 2 until second 7.

Example 2:

Input: servers = [5,1,4,3,2], tasks = [2,1,2,4,5,2,1]
Output: [1,4,1,4,1,3,2]
Explanation: Events in chronological order go as follows:
- At second 0, task 0 is added and processed using server 1 until second 2.
- At second 1, task 1 is added and processed using server 4 until second 2.
- At second 2, servers 1 and 4 become free. Task 2 is added and processed using server 1 until second 4.
- At second 3, task 3 is added and processed using server 4 until second 7.
- At second 4, server 1 becomes free. Task 4 is added and processed using server 1 until second 9.
- At second 5, task 5 is added and processed using server 3 until second 7.
- At second 6, task 6 is added and processed using server 2 until second 7.

Constraints:

    servers.length == n
    tasks.length == m
    1 <= n, m <= 2 * 10^5
    1 <= servers[i], tasks[j] <= 2 * 10^5
*/

struct Solution;

impl Solution {
    pub fn assign_tasks(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let mut q = std::collections::BTreeSet::<(i32, i32, i32)>::new();
        let mut avail = std::collections::BTreeSet::new();
        let mut res = vec![-1; tasks.len()];
        for (i, &server) in servers.iter().enumerate() {
            avail.insert((server, i as i32));
        }
        let mut i = 0;
        let mut t = 0;
        while i < tasks.len() as i32 {
            t = t.max(i);
            while !q.is_empty() && q.iter().next().unwrap().0 <= t {
                let v = *q.iter().next().unwrap();
                q.remove(&v);
                let &(_, ind, index) = &v;
                avail.insert((servers[ind as usize], ind));
                res[index as usize] = ind;
            }
            if !avail.is_empty() {
                let &(server, ind) = avail.iter().next().unwrap();
                q.insert((t + tasks[i as usize], ind, i));
                avail.remove(&(server, ind));
            } else {
                t = q.iter().next().unwrap().0;
                i -= 1;
            }
            i += 1;
        }
        while !q.is_empty() {
            let v = *q.iter().next().unwrap();
            q.remove(&v);
            let &(_, ind, index) = &v;
            res[index as usize] = ind;
        }
        res
    }

    pub fn assign_tasks2(servers: Vec<i32>, tasks: Vec<i32>) -> Vec<i32> {
        let mut q = std::collections::BTreeSet::<(i32, i32, i32)>::new();
        let mut avail = std::collections::BTreeSet::new();
        let mut res = vec![-1; tasks.len()];
        for (i, &server) in servers.iter().enumerate() {
            avail.insert((server, i as i32));
        }
        let mut i = 0;
        let mut t = 0;
        while i < tasks.len() as i32 {
            t = t.max(i);
            while !q.is_empty() && q.iter().next().unwrap().0 <= t {
                let (_, ind, index) = q.pop_first().unwrap();
                avail.insert((servers[ind as usize], ind));
                res[index as usize] = ind;
            }
            if !avail.is_empty() {
                let (_, ind) = avail.pop_first().unwrap();
                q.insert((t + tasks[i as usize], ind, i));
            } else {
                t = q.iter().next().unwrap().0;
                i -= 1;
            }
            i += 1;
        }
        while !q.is_empty() {
            let (_, ind, index) = q.pop_first().unwrap();
            res[index as usize] = ind;
        }

        res
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![3, 3, 2], vec![1, 2, 3, 2, 1, 2], vec![2, 2, 0, 2, 1, 2]),
        (vec![5, 1, 4, 3, 2], vec![2, 1, 2, 4, 5, 2, 1], vec![1, 4, 1, 4, 1, 3, 2]),
        (
            vec![10, 63, 95, 16, 85, 57, 83, 95, 6, 29, 71],
            vec![70, 31, 83, 15, 32, 67, 98, 65, 56, 48, 38, 90, 5],
            vec![8, 0, 3, 9, 5, 1, 10, 6, 4, 2, 7, 9, 0],
        ),
    ];
    for (servers, tasks, expect) in cases {
        assert_eq!(Solution::assign_tasks2(servers, tasks), expect);
    }
}
