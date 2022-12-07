#![allow(dead_code)]

// 690. Employee Importance
// https://leetcode.com/problems/employee-importance/
//
// You have a data structure of employee information, including the employee's unique ID, importance value, and direct subordinates' IDs.
//
// You are given an array of employees employees where:
//
// employees[i].id is the ID of the ith employee.
// employees[i].importance is the importance value of the ith employee.
// employees[i].subordinates is a list of the IDs of the direct subordinates of the ith employee.
// Given an integer id that represents an employee's ID, return the total importance value of this employee and all their direct and indirect subordinates.
//
// Example 1:
//
// Input: employees = [[1,5,[2,3]],[2,3,[]],[3,3,[]]], id = 1
// Output: 11
// Explanation: Employee 1 has an importance value of 5 and has two direct subordinates: employee 2 and employee 3.
// They both have an importance value of 3.
// Thus, the total importance value of employee 1 is 5 + 3 + 3 = 11.
//
// Example 2:
//
// Input: employees = [[1,2,[5]],[5,-3,[]]], id = 5
// Output: -3
// Explanation: Employee 5 has an importance value of -3 and has no direct subordinates.
// Thus, the total importance value of employee 5 is -3.
//
// Constraints:
//
// - 1 <= employees.length <= 2000
// - 1 <= employees[i].id <= 2000
// - All employees[i].id are unique.
// - -100 <= employees[i].importance <= 100
// - One employee has at most one direct leader and may have several subordinates.
// - The IDs in employees[i].subordinates are valid IDs.
//

// use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Employee {
    id: i32,
    importance: i32,
    subordinates: Vec<i32>,
}

impl Employee {
    fn new(id: i32, importance: i32, subordinates: Vec<i32>) -> Self {
        Employee {
            id,
            importance,
            subordinates,
        }
    }
}

struct Solution;

impl Solution {
    pub fn get_importance(employees: Vec<Employee>, id: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        for employee in employees {
            map.insert(employee.id, employee);
        }

        let mut importance = 0;
        let mut stack = vec![id];
        while let Some(id) = stack.pop() {
            if let Some(employee) = map.get(&id) {
                importance += employee.importance;
                stack.extend(&employee.subordinates);
            }
        }

        importance
    }
}

#[test]
fn test() {
    let employees = vec![
        Employee::new(1, 5, vec![2, 3]),
        Employee::new(2, 3, vec![]),
        Employee::new(3, 3, vec![]),
    ];
    let id = 1;
    let expected = 11;
    assert_eq!(Solution::get_importance(employees, id), expected);

    let employees = vec![Employee::new(1, 2, vec![5]), Employee::new(5, -3, vec![])];
    let id = 5;
    let expected = -3;
    assert_eq!(Solution::get_importance(employees, id), expected);
}
