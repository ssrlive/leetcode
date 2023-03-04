#![allow(dead_code)]

/*

// 2127. Maximum Employees to Be Invited to a Meeting
// https://leetcode.com/problems/maximum-employees-to-be-invited-to-a-meeting/
// https://leetcode.cn/problems/maximum-employees-to-be-invited-to-a-meeting/
//
// Hard
//
// A company is organizing a meeting and has a list of n employees, waiting to be invited. They have arranged for a large circular table, capable of seating any number of employees.

The employees are numbered from 0 to n - 1. Each employee has a favorite person and they will attend the meeting only if they can sit next to their favorite person at the table. The favorite person of an employee is not themself.

Given a 0-indexed integer array favorite, where favorite[i] denotes the favorite person of the ith employee, return the maximum number of employees that can be invited to the meeting.

Example 1:

Input: favorite = [2,2,1,2]
Output: 3
Explanation:
The above figure shows how the company can invite employees 0, 1, and 2, and seat them at the round table.
All employees cannot be invited because employee 2 cannot sit beside employees 0, 1, and 3, simultaneously.
Note that the company can also invite employees 1, 2, and 3, and give them their desired seats.
The maximum number of employees that can be invited to the meeting is 3.

Example 2:

Input: favorite = [1,2,0]
Output: 3
Explanation:
Each employee is the favorite person of at least one other employee, and the only way the company can invite them is if they invite every employee.
The seating arrangement will be the same as that in the figure given in example 1:
- Employee 0 will sit between employees 2 and 1.
- Employee 1 will sit between employees 0 and 2.
- Employee 2 will sit between employees 1 and 0.
The maximum number of employees that can be invited to the meeting is 3.

Example 3:

Input: favorite = [3,0,1,4,1]
Output: 4
Explanation:
The above figure shows how the company will invite employees 0, 1, 3, and 4, and seat them at the round table.
Employee 2 cannot be invited because the two spots next to their favorite employee 1 are taken.
So the company leaves them out of the meeting.
The maximum number of employees that can be invited to the meeting is 4.

Constraints:

    n == favorite.length
    2 <= n <= 10^5
    0 <= favorite[i] <= n - 1
    favorite[i] != i
*/

struct Solution;

impl Solution {
    pub fn maximum_invitations(favorite: Vec<i32>) -> i32 {
        let n = favorite.len();
        let mut visited = vec![false; n];
        let mut to_leaf = vec![0; n];
        let mut in_degree = vec![0; n];

        for &item in favorite.iter() {
            in_degree[item as usize] += 1;
        }
        let mut q = std::collections::VecDeque::new();
        for (i, &item) in in_degree.iter().enumerate() {
            if item == 0 {
                q.push_back(i);
                visited[i] = true;
            }
        }

        while !q.is_empty() {
            let el = q.pop_front().unwrap();
            let j = favorite[el];
            to_leaf[j as usize] = to_leaf[j as usize].max(1 + to_leaf[el]);
            in_degree[j as usize] -= 1;
            if in_degree[j as usize] == 0 {
                visited[j as usize] = true;
                q.push_back(j as usize);
            }
        }

        let (mut res, mut res2) = (0, 0);
        for i in 0..n {
            if !visited[i] {
                let mut len = 0;
                let mut j = i;
                while !visited[j] {
                    visited[j] = true;
                    len += 1;
                    j = favorite[j] as usize;
                }
                if len == 2 {
                    res2 += 2 + to_leaf[i] + to_leaf[favorite[i] as usize];
                } else {
                    res = res.max(len);
                }
            }
        }

        res.max(res2)
    }
}

#[test]
fn test() {
    #[rustfmt::skip]
    let cases = vec![
        (vec![2, 2, 1, 2], 3),
        (vec![1, 2, 0], 3),
        (vec![3, 0, 1, 4, 1], 4),
    ];
    for (favorite, expected) in cases {
        assert_eq!(Solution::maximum_invitations(favorite), expected);
    }
}
