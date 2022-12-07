#![allow(dead_code)]

// 707. Design Linked List
// https://leetcode.com/problems/design-linked-list/
//
// Design your implementation of the linked list. You can choose to use a singly or doubly linked list.
// A node in a singly linked list should have two attributes: val and next. val is the value of the current node, and next is a pointer/reference to the next node.
// If you want to use the doubly linked list, you will need one more attribute prev to indicate the previous node in the linked list. Assume all nodes in the linked list are 0-indexed.
//
// Implement the MyLinkedList class:
//
// - MyLinkedList() Initializes the MyLinkedList object.
// - int get(int index) Get the value of the indexth node in the linked list. If the index is invalid, return -1.
// - void addAtHead(int val) Add a node of value val before the first element of the linked list. After the insertion, the new node will be the first node of the linked list.
// - void addAtTail(int val) Append a node of value val as the last element of the linked list.
// - void addAtIndex(int index, int val) Add a node of value val before the indexth node in the linked list. If index equals the length of the linked list, the node will be appended to the end of the linked list. If index is greater than the length, the node will not be inserted.
// - void deleteAtIndex(int index) Delete the indexth node in the linked list, if the index is valid.
//
// Example 1:
//
// Input
// ["MyLinkedList", "addAtHead", "addAtTail", "addAtIndex", "get", "deleteAtIndex", "get"]
// [[], [1], [3], [1, 2], [1], [1], [1]]
// Output
// [null, null, null, null, 2, null, 3]
//
// Explanation
// MyLinkedList myLinkedList = new MyLinkedList();
// myLinkedList.addAtHead(1);
// myLinkedList.addAtTail(3);
// myLinkedList.addAtIndex(1, 2);    // linked list becomes 1->2->3
// myLinkedList.get(1);              // return 2
// myLinkedList.deleteAtIndex(1);    // now the linked list is 1->3
// myLinkedList.get(1);              // return 3
//
// Constraints:
//
// - 0 <= index, val <= 1000
// - Please do not use the built-in LinkedList library.
// - At most 2000 calls will be made to get, addAtHead, addAtTail, addAtIndex and deleteAtIndex.
//
// Follow up: Could you solve the problem using one of the following data structures: array, linked list, string, stack, queue, hash table, binary search tree, heap, trie, etc?

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

#[derive(Debug)]
struct MyLinkedList {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
    len: usize,
}

impl MyLinkedList {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn get(&self, index: i32) -> i32 {
        self._get(index).unwrap_or(-1)
    }
    fn _get(&self, index: i32) -> Option<i32> {
        if index < 0 || index as usize >= self.len {
            return None;
        }

        let mut node = self.head.as_ref()?.clone();
        for _ in 0..index {
            let _node = node.borrow().next.as_ref()?.clone();
            node = _node;
        }

        let val = node.borrow().val;
        Some(val)
    }

    fn add_at_head(&mut self, val: i32) {
        let node = Rc::new(RefCell::new(Node::new(val)));
        if self.head.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node);
        } else {
            node.borrow_mut().next = self.head.clone();
            self.head = Some(node);
        }
        self.len += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        if self._add_at_tail(val).is_none() {
            println!("invalid val: {}", val);
        }
    }
    fn _add_at_tail(&mut self, val: i32) -> Option<()> {
        let node = Rc::new(RefCell::new(Node::new(val)));
        if self.tail.is_none() {
            self.head = Some(node.clone());
            self.tail = Some(node);
        } else {
            self.tail.as_ref()?.borrow_mut().next = Some(node.clone());
            self.tail = Some(node);
        }
        self.len += 1;
        Some(())
    }

    pub fn add_at_index(&mut self, index: i32, val: i32) {
        if self._add_at_index(index, val).is_none() {
            println!("invalid index: {}", index);
        }
    }
    fn _add_at_index(&mut self, index: i32, val: i32) -> Option<()> {
        if index < 0 || index as usize > self.len {
            return Some(());
        }

        if index == 0 {
            self.add_at_head(val);
            return Some(());
        }

        if index as usize == self.len {
            self.add_at_tail(val);
            return Some(());
        }

        let mut node = self.head.as_ref()?.clone();
        for _ in 0..index - 1 {
            let _node = node.borrow().next.as_ref()?.clone();
            node = _node;
        }

        let new_node = Rc::new(RefCell::new(Node::new(val)));
        new_node.borrow_mut().next = node.borrow().next.clone();
        node.borrow_mut().next = Some(new_node);
        self.len += 1;
        Some(())
    }

    pub fn delete_at_index(&mut self, index: i32) {
        if self._delete_at_index(index).is_none() {
            println!("invalid index: {}", index);
        }
    }
    fn _delete_at_index(&mut self, index: i32) -> Option<()> {
        if index < 0 || index as usize >= self.len {
            return Some(());
        }

        if index == 0 {
            let head = self.head.as_ref()?.borrow().next.clone();
            self.head = head;
            if self.head.is_none() {
                self.tail = None;
            }
            self.len -= 1;
            return Some(());
        }

        let mut node = self.head.as_ref()?.clone();
        for _ in 0..index - 1 {
            let _node = node.borrow().next.as_ref()?.clone();
            node = _node;
        }

        let next = node.borrow().next.as_ref()?.borrow().next.clone();
        node.borrow_mut().next = next;
        if index as usize == self.len - 1 {
            self.tail = Some(node);
        }
        self.len -= 1;
        Some(())
    }
}

#[test]
fn test() {
    let mut linked_list = MyLinkedList::new();
    linked_list.add_at_head(1);
    linked_list.add_at_tail(3);
    linked_list.add_at_index(1, 2);
    assert_eq!(linked_list.get(1), 2);
    linked_list.delete_at_index(1);
    assert_eq!(linked_list.get(1), 3);
}
