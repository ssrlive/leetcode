#![allow(dead_code)]

// 641. Design Circular Queue
// https://leetcode.com/problems/design-circular-queue/
// https://leetcode.cn/problems/design-circular-queue/
//
// Design your implementation of the circular double-ended queue (deque).
//
// Implement the MyCircularDeque class:
//
// - MyCircularDeque(int k) Initializes the deque with a maximum size of k.
// - boolean insertFront() Adds an item at the front of Deque. Returns true if the operation is successful, or false otherwise.
// - boolean insertLast() Adds an item at the rear of Deque. Returns true if the operation is successful, or false otherwise.
// - boolean deleteFront() Deletes an item from the front of Deque. Returns true if the operation is successful, or false otherwise.
// - boolean deleteLast() Deletes an item from the rear of Deque. Returns true if the operation is successful, or false otherwise.
// - int getFront() Returns the front item from the Deque. Returns -1 if the deque is empty.
// - int getRear() Returns the last item from Deque. Returns -1 if the deque is empty.
// - boolean isEmpty() Returns true if the deque is empty, or false otherwise.
// - boolean isFull() Returns true if the deque is full, or false otherwise.
//
// Example 1:
//
// Input
// ["MyCircularDeque", "insertLast", "insertLast", "insertFront", "insertFront", "getRear", "isFull", "deleteLast", "insertFront", "getFront"]
// [[3], [1], [2], [3], [4], [], [], [], [4], []]
// Output
// [null, true, true, true, false, 2, true, true, true, 4]
//
// Explanation
// MyCircularDeque myCircularDeque = new MyCircularDeque(3);
// myCircularDeque.insertLast(1);  // return True
// myCircularDeque.insertLast(2);  // return True
// myCircularDeque.insertFront(3); // return True
// myCircularDeque.insertFront(4); // return False, the queue is full.
// myCircularDeque.getRear();      // return 2
// myCircularDeque.isFull();       // return True
// myCircularDeque.deleteLast();   // return True
// myCircularDeque.insertFront(4); // return True
// myCircularDeque.getFront();     // return 4
//
// Constraints:
//
// - 1 <= k <= 1000
// - 0 <= value <= 1000
// - At most 2000 calls will be made to insertFront, insertLast, deleteFront, deleteLast, getFront, getRear, isEmpty, isFull.
//

struct MyCircularDeque {
    data: Vec<i32>,
    head: usize,
    tail: usize,
    size: usize,
    capacity: usize,
}

impl MyCircularDeque {
    fn new(k: i32) -> Self {
        let capacity = k as usize;
        let data = vec![0; capacity];
        let head = 0;
        let tail = 0;
        let size = 0;
        Self {
            data,
            head,
            tail,
            size,
            capacity,
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.head = (self.head + self.capacity - 1) % self.capacity;
        self.data[self.head] = value;
        self.size += 1;
        true
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.tail] = value;
        self.tail = (self.tail + 1) % self.capacity;
        self.size += 1;
        true
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head = (self.head + 1) % self.capacity;
        self.size -= 1;
        true
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.tail = (self.tail + self.capacity - 1) % self.capacity;
        self.size -= 1;
        true
    }

    fn get_front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.head]
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[(self.tail + self.capacity - 1) % self.capacity]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.capacity
    }
}

#[test]
fn test() {
    let mut my_circular_deque = MyCircularDeque::new(3);
    assert!(my_circular_deque.insert_last(1));
    assert!(my_circular_deque.insert_last(2));
    assert!(my_circular_deque.insert_front(3));
    assert!(!my_circular_deque.insert_front(4));
    assert_eq!(my_circular_deque.get_rear(), 2);
    assert!(my_circular_deque.is_full());
    assert!(my_circular_deque.delete_last());
    assert!(my_circular_deque.insert_front(4));
    assert_eq!(my_circular_deque.get_front(), 4);
}
