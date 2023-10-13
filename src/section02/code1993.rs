#![allow(dead_code)]

/*

// 1993. Operations on Tree
// https://leetcode.com/problems/operations-on-tree/
// https://leetcode.cn/problems/operations-on-tree/
//
// Medium
//
// You are given a tree with n nodes numbered from 0 to n - 1 in the form of a parent array parent where parent[i] is the parent of the ith node. The root of the tree is node 0, so parent[0] = -1 since it has no parent. You want to design a data structure that allows users to lock, unlock, and upgrade nodes in the tree.

The data structure should support the following functions:

    Lock: Locks the given node for the given user and prevents other users from locking the same node. You may only lock a node using this function if the node is unlocked.
    Unlock: Unlocks the given node for the given user. You may only unlock a node using this function if it is currently locked by the same user.
    Upgrade: Locks the given node for the given user and unlocks all of its descendants regardless of who locked it. You may only upgrade a node if all 3 conditions are true:
        The node is unlocked,
        It has at least one locked descendant (by any user), and
        It does not have any locked ancestors.

Implement the LockingTree class:

    LockingTree(int[] parent) initializes the data structure with the parent array.
    lock(int num, int user) returns true if it is possible for the user with id user to lock the node num, or false otherwise. If it is possible, the node num will become locked by the user with id user.
    unlock(int num, int user) returns true if it is possible for the user with id user to unlock the node num, or false otherwise. If it is possible, the node num will become unlocked.
    upgrade(int num, int user) returns true if it is possible for the user with id user to upgrade the node num, or false otherwise. If it is possible, the node num will be upgraded.

Example 1:

Input
["LockingTree", "lock", "unlock", "unlock", "lock", "upgrade", "lock"]
[[[-1, 0, 0, 1, 1, 2, 2]], [2, 2], [2, 3], [2, 2], [4, 5], [0, 1], [0, 1]]
Output
[null, true, false, true, true, true, false]

Explanation
LockingTree lockingTree = new LockingTree([-1, 0, 0, 1, 1, 2, 2]);
lockingTree.lock(2, 2);    // return true because node 2 is unlocked.
                           // Node 2 will now be locked by user 2.
lockingTree.unlock(2, 3);  // return false because user 3 cannot unlock a node locked by user 2.
lockingTree.unlock(2, 2);  // return true because node 2 was previously locked by user 2.
                           // Node 2 will now be unlocked.
lockingTree.lock(4, 5);    // return true because node 4 is unlocked.
                           // Node 4 will now be locked by user 5.
lockingTree.upgrade(0, 1); // return true because node 0 is unlocked and has at least one locked descendant (node 4).
                           // Node 0 will now be locked by user 1 and node 4 will now be unlocked.
lockingTree.lock(0, 1);    // return false because node 0 is already locked.

Constraints:

    n == parent.length
    2 <= n <= 2000
    0 <= parent[i] <= n - 1 for i != 0
    parent[0] == -1
    0 <= num <= n - 1
    1 <= user <= 10^4
    parent represents a valid tree.
    At most 2000 calls in total will be made to lock, unlock, and upgrade.
*/

struct LockingTree {
    locks: Vec<Option<usize>>,
    g: Vec<Vec<usize>>,
    parent: Vec<i32>,
}

impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let n = parent.len();
        let mut g = vec![vec![]; n];
        for i in 0..n {
            if parent[i] == -1 {
                continue;
            }
            g[parent[i] as usize].push(i);
        }

        LockingTree {
            locks: vec![None; n],
            g,
            parent,
        }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        if self.locks[num as usize].is_some() {
            false
        } else {
            self.locks[num as usize] = Some(user as usize);
            true
        }
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        if let Some(lock_user) = self.locks[num as usize] {
            if lock_user as i32 == user {
                self.locks[num as usize] = None;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        let mut ci = num;
        while ci != -1 {
            let i = ci as usize;
            if self.locks[i].is_some() {
                return false;
            }
            ci = self.parent[i];
        }

        let mut dirty = false;
        let mut stack = vec![num as usize];
        while !stack.is_empty() {
            let mut new_stack = vec![];
            while let Some(ci) = stack.pop() {
                for i in 0..self.g[ci].len() {
                    let ni = self.g[ci][i];

                    if self.locks[ni].is_some() {
                        self.locks[ni] = None;
                        dirty = true;
                    }
                    new_stack.push(ni);
                }
            }
            stack = new_stack;
        }

        if dirty {
            self.locks[num as usize] = Some(user as usize);
        }

        dirty
    }
}

#[test]
fn test() {
    let mut tree = LockingTree::new(vec![-1, 0, 0, 1, 1, 2, 2]);
    assert!(tree.lock(2, 2));
    assert!(!tree.unlock(2, 3));
    assert!(tree.unlock(2, 2));
    assert!(tree.lock(4, 5));
    assert!(tree.upgrade(0, 1));
    assert!(!tree.lock(0, 1));
}
