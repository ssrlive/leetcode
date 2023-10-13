#![allow(dead_code)]

// 1145. Binary Tree Coloring Game
// https://leetcode.com/problems/binary-tree-coloring-game/
// https://leetcode.cn/problems/binary-tree-coloring-game/
//
// Two players play a turn based game on a binary tree. We are given the root of this binary tree,
// and the number of nodes n in the tree. n is odd, and each node has a distinct value from 1 to n.
//
// Initially, the first player names a value x with 1 <= x <= n, and the second player names a value y with 1 <= y <= n and y != x.
// The first player colors the node with value x red, and the second player colors the node with value y blue.
//
// Then, the players take turns starting with the first player. In each turn, that player chooses a node of their color
// (red if player 1, blue if player 2) and colors an uncolored neighbor of the chosen node (either the left child, right child, or parent of the chosen node.)
//
// If (and only if) a player cannot choose such a node in this way, they must pass their turn.
// If both players pass their turn, the game ends, and the winner is the player that colored more nodes.
//
// You are the second player. If it is possible to choose such a y to ensure you win the game, return true. If it is not possible, return false.
//
// Example 1:
//
// Input: root = [1,2,3,4,5,6,7,8,9,10,11], n = 11, x = 3
// Output: true
// Explanation: The second player can choose the node with value 2.
//
// Example 2:
//
// Input: root = [1,2,3], n = 3, x = 1
// Output: false
//
// Constraints:
//
// - The number of nodes in the tree is n.
// - 1 <= x <= n <= 100
// - n is odd.
// - 1 <= Node.val <= n
// - All the values of the tree are unique.
//

use crate::treenode::TreeNode;

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, count: &mut i32, n: i32, x: i32) -> bool {
            if root.is_none() {
                return false;
            }
            let node = root.as_ref().unwrap().borrow();
            let (mut count1, mut count2) = (0, 0);
            let ret1 = dfs(&node.left, &mut count1, n, x);
            let ret2 = dfs(&node.right, &mut count2, n, x);
            if ret1 || ret2 {
                return true;
            }
            *count = 1 + count1 + count2;
            if node.val == x {
                return 2 * count1 > n || 2 * count2 > n || 2 * *count < n;
            }
            false
        }

        let mut count = 0;
        dfs(&root, &mut count, n, x)
    }
}

#[test]
fn test() {
    let root = TreeNode::from_vec(&[
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
        Some(7),
        Some(8),
        Some(9),
        Some(10),
        Some(11),
    ]);
    assert!(Solution::btree_game_winning_move(root, 11, 3));

    let root = TreeNode::from_vec(&[Some(1), Some(2), Some(3)]);
    assert!(!Solution::btree_game_winning_move(root, 3, 1));
}
