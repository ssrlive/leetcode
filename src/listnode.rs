#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(v: &[i32]) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut tail = &mut head;
        for i in v {
            tail.as_mut()?.next = Some(Box::new(ListNode::new(*i)));
            tail = &mut tail.as_mut()?.next;
        }
        head?.next.take()
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut v = vec![];
        let mut p = Some(self);
        while let Some(node) = p {
            v.push(node.val);
            p = match &node.next {
                Some(node) => Some(node),
                None => None,
            };
        }
        v
    }
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut s = String::new();
        let mut node = Some(self);
        while let Some(n) = node {
            s.push_str(&n.val.to_string());
            node = n.next.as_deref();
            if node.is_some() {
                s.push_str(" -> ");
            }
        }
        write!(f, "{s}")
    }
}

// impl Drop for ListNode {
//     // Drop is called when the object goes out of scope.
//     // To avoid infinite recursion, we need to set the next pointer to None.
//     // https://matklad.github.io/2022/11/18/if-a-tree-falls-in-a-forest-does-it-overflow-the-stack.html
//     fn drop(&mut self) {
//         let mut next = self.next.take();
//         while let Some(mut node) = next {
//             next = node.next.take();
//         }
//     }
// }

// #[test]
// fn test_listnode() -> Result<(), Box<dyn std::error::Error>> {
//     // no stack overflow
//     let v = (0..100_000).collect::<Vec<i32>>();
//     let list = ListNode::from_vec(&v);
//     assert_eq!(list.as_ref().ok_or("")?.to_vec(), v);
//     Ok(())
// }
