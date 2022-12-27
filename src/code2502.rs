#![allow(dead_code)]

// 2502. Design Memory Allocator
// https://leetcode.com/problems/design-memory-allocator/
// https://leetcode.cn/problems/design-memory-allocator/
//
// You are given an integer n representing the size of a 0-indexed memory array. All memory units are initially free.
//
// You have a memory allocator with the following functionalities:
//
// Allocate a block of size consecutive free memory units and assign it the id mID.
// Free all memory units with the given id mID.
// Note that:
//
// - Multiple blocks can be allocated to the same mID.
// - You should free all the memory units with mID, even if they were allocated in different blocks.
//
//Implement the Allocator class:
//
// - Allocator(int n) Initializes an Allocator object with a memory array of size n.
// - int allocate(int size, int mID) Find the leftmost block of size consecutive free memory units and allocate it with the id mID.
//   Return the block's first index. If such a block does not exist, return -1.
// - int free(int mID) Free all memory units with the id mID. Return the number of memory units you have freed.
//
// Example 1:
//
// Input
// ["Allocator", "allocate", "allocate", "allocate", "free", "allocate", "allocate", "allocate", "free", "allocate", "free"]
// [[10], [1, 1], [1, 2], [1, 3], [2], [3, 4], [1, 1], [1, 1], [1], [10, 2], [7]]
// Output
// [null, 0, 1, 2, 1, 3, 1, 6, 3, -1, 0]
//
// Explanation
// Allocator loc = new Allocator(10); // Initialize a memory array of size 10. All memory units are initially free.
// loc.allocate(1, 1); // The leftmost block's first index is 0. The memory array becomes [1,_,_,_,_,_,_,_,_,_]. We return 0.
// loc.allocate(1, 2); // The leftmost block's first index is 1. The memory array becomes [1,2,_,_,_,_,_,_,_,_]. We return 1.
// loc.allocate(1, 3); // The leftmost block's first index is 2. The memory array becomes [1,2,3,_,_,_,_,_,_,_]. We return 2.
// loc.free(2); // Free all memory units with mID 2. The memory array becomes [1,_, 3,_,_,_,_,_,_,_]. We return 1 since there is only 1 unit with mID 2.
// loc.allocate(3, 4); // The leftmost block's first index is 3. The memory array becomes [1,_,3,4,4,4,_,_,_,_]. We return 3.
// loc.allocate(1, 1); // The leftmost block's first index is 1. The memory array becomes [1,1,3,4,4,4,_,_,_,_]. We return 1.
// loc.allocate(1, 1); // The leftmost block's first index is 6. The memory array becomes [1,1,3,4,4,4,1,_,_,_]. We return 6.
// loc.free(1); // Free all memory units with mID 1. The memory array becomes [_,_,3,4,4,4,_,_,_,_]. We return 3 since there are 3 units with mID 1.
// loc.allocate(10, 2); // We can not find any free block with 10 consecutive free memory units, so we return -1.
// loc.free(7); // Free all memory units with mID 7. The memory array remains the same since there is no memory unit with mID 7. We return 0.
//
// Constraints:
//
// - 1 <= n, size, mID <= 1000
// - At most 1000 calls will be made to allocate and free.
//

pub struct Allocator {
    pub memory: Vec<i32>,
    pub free: Vec<(usize, usize)>,
}

impl Allocator {
    pub fn new(n: i32) -> Self {
        Allocator {
            memory: vec![-1; n as usize],
            free: vec![(0, n as usize)],
        }
    }

    pub fn allocate(&mut self, size: i32, m_id: i32) -> i32 {
        let size = size as usize;
        let m_id = m_id as usize;

        let mut i = 0;
        while i < self.free.len() {
            let (start, end) = self.free[i];
            if end - start >= size {
                self.free.remove(i);
                if start + size < end {
                    self.free.insert(i, (start + size, end));
                }
                for j in start..start + size {
                    self.memory[j] = m_id as i32;
                }
                return start as i32;
            }
            i += 1;
        }
        -1
    }

    pub fn free(&mut self, m_id: i32) -> i32 {
        let m_id = m_id as usize;
        let mut i = 0;
        let mut count = 0;
        while i < self.memory.len() {
            if self.memory[i] == m_id as i32 {
                self.memory[i] = -1;
                count += 1;
                let mut j = i + 1;
                while j < self.memory.len() && self.memory[j] == m_id as i32 {
                    self.memory[j] = -1;
                    count += 1;
                    j += 1;
                }
                self.free.push((i, j));
                i = j;
            } else {
                i += 1;
            }
        }
        self.free.sort_by(|a, b| a.0.cmp(&b.0));
        let mut i = 0;
        while !self.free.is_empty() && i < self.free.len() - 1 {
            let (start, end) = self.free[i];
            let (next_start, next_end) = self.free[i + 1];
            if end == next_start {
                self.free.remove(i);
                self.free[i] = (start, next_end);
            } else {
                i += 1;
            }
        }
        count
    }
}

#[test]
fn test() {
    let mut loc = Allocator::new(10);
    assert_eq!(loc.allocate(1, 1), 0);
    assert_eq!(loc.allocate(1, 2), 1);
    assert_eq!(loc.allocate(1, 3), 2);
    assert_eq!(loc.free(2), 1);
    assert_eq!(loc.allocate(3, 4), 3);
    assert_eq!(loc.allocate(1, 1), 1);
    assert_eq!(loc.allocate(1, 1), 6);
    assert_eq!(loc.free(1), 3);
    assert_eq!(loc.allocate(10, 2), -1);
    assert_eq!(loc.free(7), 0);

    let mut loc = Allocator::new(5);
    assert_eq!(loc.free(4), 0);
    assert_eq!(loc.allocate(9, 5), -1);
    assert_eq!(loc.allocate(5, 8), 0);
    assert_eq!(loc.allocate(4, 8), -1);
    assert_eq!(loc.allocate(3, 2), -1);
    assert_eq!(loc.free(6), 0);
    assert_eq!(loc.allocate(9, 9), -1);
    assert_eq!(loc.free(6), 0);
}
