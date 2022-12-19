#![allow(dead_code)]

// 622. Design Circular Queue
// https://leetcode.com/problems/design-circular-queue/
// https://leetcode.cn/problems/design-circular-queue/
//
// Design your implementation of the circular queue. The circular queue is a linear data structure in which the operations
// are performed based on FIFO (First In First Out) principle, and the last position is connected back
// to the first position to make a circle. It is also called "Ring Buffer".
//
// One of the benefits of the circular queue is that we can make use of the spaces in front of the queue.
// In a normal queue, once the queue becomes full, we cannot insert the next element even if there is a space in front of the queue.
// But using the circular queue, we can use the space to store new values.
//
// Implement the MyCircularQueue class:
//
// - MyCircularQueue(k) Initializes the object with the size of the queue to be k.
// - int Front() Gets the front item from the queue. If the queue is empty, return -1.
// - int Rear() Gets the last item from the queue. If the queue is empty, return -1.
// - boolean enQueue(int value) Inserts an element into the circular queue. Return true if the operation is successful.
// - boolean deQueue() Deletes an element from the circular queue. Return true if the operation is successful.
// - boolean isEmpty() Checks whether the circular queue is empty or not.
// - boolean isFull() Checks whether the circular queue is full or not.
//
// You must solve the problem without using the built-in queue data structure in your programming language.
//
// Example 1:
//
// Input
// ["MyCircularQueue", "enQueue", "enQueue", "enQueue", "enQueue", "Rear", "isFull", "deQueue", "enQueue", "Rear"]
// [[3], [1], [2], [3], [4], [], [], [], [4], []]
// Output
// [null, true, true, true, false, 3, true, true, true, 4]
//
// Explanation
// MyCircularQueue myCircularQueue = new MyCircularQueue(3);
// myCircularQueue.enQueue(1); // return True
// myCircularQueue.enQueue(2); // return True
// myCircularQueue.enQueue(3); // return True
// myCircularQueue.enQueue(4); // return False
// myCircularQueue.Rear();     // return 3
// myCircularQueue.isFull();   // return True
// myCircularQueue.deQueue();  // return True
// myCircularQueue.enQueue(4); // return True
// myCircularQueue.Rear();     // return 4
//
// Constraints:
//
// - 1 <= k <= 1000
// - 0 <= value <= 1000
// - At most 3000 calls will be made to enQueue, deQueue, Front, Rear, isEmpty, and isFull.
//

struct MyCircularQueue {
    data: Vec<i32>,
    head: usize,
    tail: usize,
    size: usize,
}

impl MyCircularQueue {
    fn new(k: i32) -> Self {
        let data = vec![0; k as usize];
        MyCircularQueue {
            data,
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            return false;
        }
        self.data[self.tail] = value;
        self.tail = (self.tail + 1) % self.data.len();
        self.size += 1;
        true
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            return false;
        }
        self.head = (self.head + 1) % self.data.len();
        self.size -= 1;
        true
    }

    fn front(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        self.data[self.head]
    }

    fn rear(&self) -> i32 {
        if self.is_empty() {
            return -1;
        }
        let data_len = self.data.len();
        self.data[(self.tail + data_len - 1) % data_len]
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn is_full(&self) -> bool {
        self.size == self.data.len()
    }
}

#[test]
fn test() {
    let mut q = MyCircularQueue::new(3);
    assert_eq!(q.en_queue(1), true);
    assert_eq!(q.en_queue(2), true);
    assert_eq!(q.en_queue(3), true);
    assert_eq!(q.en_queue(4), false);
    assert_eq!(q.rear(), 3);
    assert_eq!(q.is_full(), true);
    assert_eq!(q.de_queue(), true);
    assert_eq!(q.en_queue(4), true);
    assert_eq!(q.rear(), 4);
}
