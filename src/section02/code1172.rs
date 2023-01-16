#![allow(dead_code)]

// 1172. Dinner Plate Stacks
// https://leetcode.com/problems/dinner-plate-stacks/
// https://leetcode.cn/problems/dinner-plate-stacks/
//
// You have an infinite number of stacks arranged in a row and numbered (left to right) from 0,
// each of the stacks has the same maximum capacity.
//
// Implement the DinnerPlates class:
//
// DinnerPlates(int capacity) Initializes the object with the maximum capacity of the stacks capacity.
// void push(int val) Pushes the given integer val into the leftmost stack with a size less than capacity.
// int pop() Returns the value at the top of the rightmost non-empty stack and removes it from that stack,
//   and returns -1 if all the stacks are empty.
// int popAtStack(int index) Returns the value at the top of the stack with the given index index
//   and removes it from that stack or returns -1 if the stack with that given index is empty.
//
// Example 1:
//
// Input
// ["DinnerPlates", "push", "push", "push", "push", "push", "popAtStack", "push", "push", "popAtStack", "popAtStack", "pop", "pop", "pop", "pop", "pop"]
// [[2], [1], [2], [3], [4], [5], [0], [20], [21], [0], [2], [], [], [], [], []]
// Output
// [null, null, null, null, null, null, 2, null, null, 20, 21, 5, 4, 3, 1, -1]
//
// Explanation:
// DinnerPlates D = DinnerPlates(2);  // Initialize with capacity = 2
// D.push(1);
// D.push(2);
// D.push(3);
// D.push(4);
// D.push(5);         // The stacks are now:  2  4
//                                            1  3  5
//                                            ﹈ ﹈ ﹈
// D.popAtStack(0);   // Returns 2.  The stacks are now:     4
//                                                        1  3  5
//                                                        ﹈ ﹈ ﹈
// D.push(20);        // The stacks are now: 20  4
//                                            1  3  5
//                                            ﹈ ﹈ ﹈
// D.push(21);        // The stacks are now: 20  4 21
//                                            1  3  5
//                                            ﹈ ﹈ ﹈
// D.popAtStack(0);   // Returns 20.  The stacks are now:     4 21
//                                                         1  3  5
//                                                         ﹈ ﹈ ﹈
// D.popAtStack(2);   // Returns 21.  The stacks are now:     4
//                                                         1  3  5
//                                                         ﹈ ﹈ ﹈
// D.pop()            // Returns 5.  The stacks are now:      4
//                                                         1  3
//                                                         ﹈ ﹈
// D.pop()            // Returns 4.  The stacks are now:   1  3
//                                                         ﹈ ﹈
// D.pop()            // Returns 3.  The stacks are now:   1
//                                                         ﹈
// D.pop()            // Returns 1.  There are no stacks.
// D.pop()            // Returns -1.  There are still no stacks.
//
// Constraints:
//
// - 1 <= capacity <= 2 * 10^4
// - 1 <= val <= 2 * 10^4
// - 0 <= index <= 10^5
// - At most 2 * 10^5 calls will be made to push, pop, and popAtStack.
//

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::convert::TryInto;

struct DinnerPlates {
    stacks: Vec<Option<i32>>,
    empty_positions: BinaryHeap<Reverse<usize>>,
    max_capacity: usize,
}

impl DinnerPlates {
    fn new(capacity: i32) -> Self {
        Self {
            stacks: Vec::new(),
            empty_positions: BinaryHeap::new(),
            max_capacity: capacity.try_into().unwrap(),
        }
    }

    fn push(&mut self, val: i32) {
        while let Some(Reverse(pos)) = self.empty_positions.pop() {
            if pos >= self.stacks.len() {
                self.empty_positions.clear();
                break;
            }
            if self.stacks[pos].is_none() {
                self.stacks[pos] = Some(val);
                return;
            }
        }
        self.stacks.push(Some(val));
    }

    fn pop(&mut self) -> i32 {
        while let Some(None) = self.stacks.last().copied() {
            self.stacks.pop();
        }
        match self.stacks.pop() {
            Some(Some(v)) => v,
            _ => -1,
        }
    }

    fn pop_at_stack(&mut self, index: i32) -> i32 {
        let index: usize = index.try_into().unwrap();
        let slice_start = index * self.max_capacity;
        if slice_start >= self.stacks.len() {
            return -1;
        }
        let slice_end = std::cmp::min(slice_start + self.max_capacity, self.stacks.len());
        for idx in (slice_start..slice_end).rev() {
            if let Some(v) = self.stacks[idx] {
                self.stacks[idx] = None;
                self.empty_positions.push(Reverse(idx));
                return v;
            }
        }
        -1
    }
}

#[test]
fn test() {
    let mut dinner_plates = DinnerPlates::new(2);
    dinner_plates.push(1);
    dinner_plates.push(2);
    dinner_plates.push(3);
    dinner_plates.push(4);
    dinner_plates.push(5);
    assert_eq!(dinner_plates.pop_at_stack(0), 2);
    dinner_plates.push(20);
    dinner_plates.push(21);
    assert_eq!(dinner_plates.pop_at_stack(0), 20);
    assert_eq!(dinner_plates.pop_at_stack(2), 21);
    assert_eq!(dinner_plates.pop(), 5);
    assert_eq!(dinner_plates.pop(), 4);
    assert_eq!(dinner_plates.pop(), 3);
    assert_eq!(dinner_plates.pop(), 1);
    assert_eq!(dinner_plates.pop(), -1);
}
