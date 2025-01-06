#![allow(dead_code)]

// 3408. Design Task Manager
// https://leetcode.com/problems/design-task-manager/
// https://leetcode.cn/problems/design-task-manager/
//
// Medium
//
// There is a task management system that allows users to manage their tasks, each associated with a priority.
// The system should efficiently handle adding, modifying, executing, and removing tasks.
//
// Implement the TaskManager class:
//
// - TaskManager(vector<vector<int>>& tasks) initializes the task manager with a list of user-task-priority triples.
//   Each element in the input list is of the form [userId, taskId, priority], which adds a task to the specified user with the given priority.
// - void add(int userId, int taskId, int priority) adds a task with the specified taskId and priority
//   to the user with userId. It is guaranteed that taskId does not exist in the system.
// - void edit(int taskId, int newPriority) updates the priority of the existing taskId to newPriority.
//   It is guaranteed that taskId exists in the system.
// - void rmv(int taskId) removes the task identified by taskId from the system.
//   It is guaranteed that taskId exists in the system.
// - int execTop() executes the task with the highest priority across all users. If there are multiple tasks with the
//   same highest priority, execute the one with the highest taskId. After executing, the taskId is removed from the system.
//   Return the userId associated with the executed task. If no tasks are available, return -1.
//
// Note that a user may be assigned multiple tasks.
//
// Example 1:
//
// Input:
// ["TaskManager", "add", "edit", "execTop", "rmv", "add", "execTop"]
// [[[[1, 101, 10], [2, 102, 20], [3, 103, 15]]], [4, 104, 5], [102, 8], [], [101], [5, 105, 15], []]
//
// Output:
// [null, null, null, 3, null, null, 5]
//
// Explanation
// TaskManager taskManager = new TaskManager([[1, 101, 10], [2, 102, 20], [3, 103, 15]]); // Initializes with three tasks for Users 1, 2, and 3.
// taskManager.add(4, 104, 5); // Adds task 104 with priority 5 for User 4.
// taskManager.edit(102, 8); // Updates priority of task 102 to 8.
// taskManager.execTop(); // return 3. Executes task 103 for User 3.
// taskManager.rmv(101); // Removes task 101 from the system.
// taskManager.add(5, 105, 15); // Adds task 105 with priority 15 for User 5.
// taskManager.execTop(); // return 5. Executes task 105 for User 5.
//
// Constraints:
//
//     1 <= tasks.length <= 10^5
//     0 <= userId <= 10^5
//     0 <= taskId <= 10^5
//     0 <= priority <= 10^9
//     0 <= newPriority <= 10^9
//     At most 2 * 10^5 calls will be made in total to add, edit, rmv, and execTop methods.
//

use std::collections::{BinaryHeap, HashMap};

struct TaskManager {
    tasks: BinaryHeap<(i32, i32, i32)>,
    prs: HashMap<i32, i32>,
    usrs: HashMap<i32, i32>,
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut tasks_heap = BinaryHeap::new();
        let mut prs = HashMap::new();
        let mut usrs = HashMap::new();
        for task in tasks {
            let (usr, task_id, priority) = (task[0], task[1], task[2]);
            tasks_heap.push((priority, task_id, usr));
            prs.insert(task_id, priority);
            usrs.insert(task_id, usr);
        }
        TaskManager {
            tasks: tasks_heap,
            prs,
            usrs,
        }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.tasks.push((priority, task_id, user_id));
        self.prs.insert(task_id, priority);
        self.usrs.insert(task_id, user_id);
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        let user_id = self.usrs.get(&task_id).unwrap();
        self.tasks.push((new_priority, task_id, *user_id));
        self.prs.insert(task_id, new_priority);
    }

    fn rmv(&mut self, task_id: i32) {
        self.prs.insert(task_id, -1);
        self.usrs.insert(task_id, -1);
    }

    fn exec_top(&mut self) -> i32 {
        let (mut priority, mut task_id, mut user_id) = match self.tasks.pop() {
            Some((p, t, u)) => (p, t, u),
            None => return -1,
        };
        while self.prs.get(&task_id).unwrap() != &priority {
            (priority, task_id, user_id) = match self.tasks.pop() {
                Some((p, t, u)) => (p, t, u),
                None => return -1,
            };
        }
        self.rmv(task_id);
        user_id
    }
}

#[test]
fn test() {
    let mut task_manager = TaskManager::new(vec![vec![1, 101, 10], vec![2, 102, 20], vec![3, 103, 15]]);
    task_manager.add(4, 104, 5);
    task_manager.edit(102, 8);
    assert_eq!(task_manager.exec_top(), 3);
    task_manager.rmv(101);
    task_manager.add(5, 105, 15);
    assert_eq!(task_manager.exec_top(), 5);
}
