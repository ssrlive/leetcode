#![allow(dead_code)]

// 133. Clone Graph
// https://leetcode.com/problems/clone-graph/
// https://leetcode.cn/problems/clone-graph/
//
// Given a reference of a node in a connected undirected graph.
//
// Return a deep copy (clone) of the graph.
//
// Each node in the graph contains a val (int) and a list (List[Node]) of its neighbors.
//

/*
// cpp solution
class Solution {
public:
    Node* cloneGraph(Node* node) {
        if (!node) {
            return nullptr;
        }
        unordered_map<int,Node*> visited;
        return dfs(node, visited);
    }

    Node * dfs(Node *node, unordered_map<int,Node*> &visited) {
        Node *root = new Node(node->val);
        visited[node->val] = root;
        for(auto it:node->neighbors) {
            if (!visited[it->val]) {
                root->neighbors.push_back(dfs(it, visited));
            } else {
                root->neighbors.push_back(visited[it->val]);
            }
        }
        return root;
    }
};
*/

struct Node {
    val: i32,
    neighbors: Vec<Option<Rc<RefCell<Node>>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Node {
            val,
            neighbors: Vec::new(),
        }
    }
}

impl Drop for Node {
    fn drop(&mut self) {
        // memory leak! this method will never be called.
        println!("drop node {}", self.val);
    }
}

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
        type OptNode = Option<Rc<RefCell<Node>>>;
        use std::collections::HashMap;

        fn node_deep_copy(node: &OptNode) -> OptNode {
            node.as_ref()
                .map(|rc| Rc::new(RefCell::new(Node::new(rc.borrow().val))))
        }

        fn dfs(node: &OptNode, visited: &mut HashMap<i32, OptNode>) -> OptNode {
            let root = node_deep_copy(node);
            let val = node.as_ref()?.borrow().val;
            visited.insert(val, root.clone());
            for neighbor in node.as_ref()?.borrow().neighbors.iter() {
                let neighbor_val = neighbor.as_ref()?.borrow().val;
                if let Some(neighbor) = visited.get(&neighbor_val) {
                    root.as_ref()?.borrow_mut().neighbors.push(neighbor.clone());
                } else {
                    let neighbor = dfs(neighbor, visited);
                    root.as_ref()?.borrow_mut().neighbors.push(neighbor);
                }
            }
            root
        }

        node.as_ref()?;
        let mut visited = HashMap::new();
        dfs(&node, &mut visited)
    }
}

#[test]
fn test_clone_graph() {
    fn _test_clone_graph() -> Option<()> {
        let node1 = Some(Rc::new(RefCell::new(Node::new(1))));
        let node2 = Some(Rc::new(RefCell::new(Node::new(2))));
        let node3 = Some(Rc::new(RefCell::new(Node::new(3))));
        let node4 = Some(Rc::new(RefCell::new(Node::new(4))));
        node1.as_ref()?.borrow_mut().neighbors = vec![node2.clone(), node4.clone()];
        node2.as_ref()?.borrow_mut().neighbors = vec![node1.clone(), node3.clone()];
        node3.as_ref()?.borrow_mut().neighbors = vec![node2.clone(), node4.clone()];
        node4.as_ref()?.borrow_mut().neighbors = vec![node1.clone(), node3.clone()];
        let result = Solution::clone_graph(node1);
        assert_eq!(result.as_ref()?.borrow().val, 1);
        assert_eq!(result.as_ref()?.borrow().neighbors.len(), 2);
        assert_eq!(result.as_ref()?.borrow().neighbors[0].as_ref()?.borrow().val, 2);
        assert_eq!(result.as_ref()?.borrow().neighbors[1].as_ref()?.borrow().val, 4);
        Some(())
    }
    _test_clone_graph().unwrap();
}
