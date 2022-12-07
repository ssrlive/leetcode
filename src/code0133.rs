#![allow(dead_code)]

// 133. Clone Graph
// https://leetcode.com/problems/clone-graph/
//
// Given a reference of a node in a connected undirected graph.
//
// Return a deep copy (clone) of the graph.
//
// Each node in the graph contains a val (int) and a list (List[Node]) of its neighbors.
//

struct Node {
    val: i32,
    neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: Vec::new(),
        }
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        fn dfs(
            node: Rc<RefCell<Node>>,
            mp: &mut std::collections::HashMap<i32, Rc<RefCell<Node>>>,
        ) -> Rc<RefCell<Node>> {
            let mut neighbors = Vec::new();
            let clone = Rc::new(RefCell::new(Node::new(node.borrow().val)));
            mp.insert(clone.borrow().val, clone.clone());
            for neighbor_rc in node.borrow().neighbors.iter() {
                if let Some(neighbor_clone_rc) = mp.get(&neighbor_rc.borrow().val) {
                    neighbors.push(neighbor_clone_rc.clone());
                } else {
                    neighbors.push(dfs(neighbor_rc.clone(), mp));
                }
            }
            clone.borrow_mut().neighbors = neighbors;
            clone
        }

        node.as_ref()?;
        let mut mp = std::collections::HashMap::new();
        Some(dfs(node?, &mut mp))
    }
}

#[test]
fn test_clone_graph() -> Result<(), Box<dyn std::error::Error>> {
    let node1 = Rc::new(RefCell::new(Node::new(1)));
    let node2 = Rc::new(RefCell::new(Node::new(2)));
    let node3 = Rc::new(RefCell::new(Node::new(3)));
    let node4 = Rc::new(RefCell::new(Node::new(4)));
    node1.borrow_mut().neighbors = vec![node2.clone(), node4.clone()];
    node2.borrow_mut().neighbors = vec![node1.clone(), node3.clone()];
    node3.borrow_mut().neighbors = vec![node2.clone(), node4.clone()];
    node4.borrow_mut().neighbors = vec![node1.clone(), node3.clone()];
    let clone = Solution::clone_graph(Some(node1));
    assert_eq!(clone.as_ref().ok_or("")?.borrow().val, 1);
    assert_eq!(clone.as_ref().ok_or("")?.borrow().neighbors.len(), 2);
    assert_eq!(clone.as_ref().ok_or("")?.borrow().neighbors[0].borrow().val, 2);
    assert_eq!(clone.as_ref().ok_or("")?.borrow().neighbors[1].borrow().val, 4);
    Ok(())
}
