#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub val: T,
    pub next: Option<Box<ListNode<T>>>,
}

impl<T: Default + Copy> ListNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        ListNode { next: None, val }
    }

    pub fn from_vec(v: Vec<T>) -> Option<Box<ListNode<T>>> {
        let mut head = Some(Box::new(ListNode::new(T::default())));
        let mut tail = &mut head;
        for i in v {
            tail.as_mut()?.next = Some(Box::new(ListNode::new(i)));
            tail = &mut tail.as_mut()?.next;
        }
        head?.next
    }

    pub fn to_vec(&self) -> Vec<T> {
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

impl<T: std::fmt::Display> std::fmt::Display for ListNode<T> {
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
        write!(f, "{}", s)
    }
}
