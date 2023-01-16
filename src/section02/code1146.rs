#![allow(dead_code)]

// 1146. Snapshot Array
// https://leetcode.com/problems/snapshot-array/
// https://leetcode.cn/problems/snapshot-array/
//
// Implement a SnapshotArray that supports the following interface:
//
// SnapshotArray(int length) initializes an array-like data structure with the given length. Initially, each element equals 0.
// void set(index, val) sets the element at the given index to be equal to val.
// int snap() takes a snapshot of the array and returns the snap_id: the total number of times we called snap() minus 1.
// int get(index, snap_id) returns the value at the given index, at the time we took the snapshot with the given snap_id
//
// Example 1:
//
// Input: ["SnapshotArray","set","snap","set","get"]
// [[3],[0,5],[],[0,6],[0,0]]
// Output: [null,null,0,null,5]
// Explanation:
// SnapshotArray snapshotArr = new SnapshotArray(3); // set the length to be 3
// snapshotArr.set(0,5);  // Set array[0] = 5
// snapshotArr.snap();  // Take a snapshot, return snap_id = 0
// snapshotArr.set(0,6);
// snapshotArr.get(0,0);  // Get the value of array[0] with snap_id = 0, return 5
//
// Constraints:
//
// - 1 <= length <= 5 * 10^4
// - 0 <= index < length
// - 0 <= val <= 10^9
// - 0 <= snap_id < (the total number of times we call snap())
// - At most 5 * 10^4 calls will be made to set, snap, and get.
//

use std::collections::HashMap;

#[derive(Clone)]
struct SnapshotArray {
    length: i32,
    array: Vec<HashMap<i32, i32>>,
    curr_array: HashMap<i32, i32>,
    snap_count: i32,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        let array = Vec::new();
        let curr_array = HashMap::new();
        let snap_count = -1;
        Self {
            length,
            array,
            curr_array,
            snap_count,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        self.curr_array.insert(index, val);
    }

    fn snap(&mut self) -> i32 {
        let to_insert = self.curr_array.clone();
        self.array.push(to_insert);
        self.snap_count += 1;
        self.snap_count
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        let array_iter = self.array.iter();
        for (count, val) in array_iter.enumerate() {
            if count as i32 == snap_id && val.contains_key(&index) {
                return val[&index];
            }
        }
        0
    }
}

#[test]
fn test() {
    let mut sa = SnapshotArray::new(3);
    sa.set(0, 5);
    assert_eq!(sa.snap(), 0);
    sa.set(0, 6);
    assert_eq!(sa.get(0, 0), 5);
    sa.set(0, 7);
    assert_eq!(sa.get(0, 0), 5);
    assert_eq!(sa.get(0, 1), 0);
    assert_eq!(sa.get(0, 2), 0);
    sa.set(1, 1);
    sa.set(2, 2);
    assert_eq!(sa.snap(), 1);
    assert_eq!(sa.get(0, 0), 5);
    assert_eq!(sa.get(0, 1), 7);
    assert_eq!(sa.get(1, 0), 0);
    assert_eq!(sa.get(1, 1), 1);
    assert_eq!(sa.get(2, 0), 0);
    assert_eq!(sa.get(2, 1), 2);
    sa.set(0, 8);
    assert_eq!(sa.snap(), 2);
    assert_eq!(sa.get(0, 0), 5);
    assert_eq!(sa.get(0, 1), 7);
    assert_eq!(sa.get(0, 2), 8);
    assert_eq!(sa.get(1, 0), 0);
    assert_eq!(sa.get(1, 1), 1);
    assert_eq!(sa.get(1, 2), 1);
    assert_eq!(sa.get(2, 0), 0);
    assert_eq!(sa.get(2, 1), 2);
    assert_eq!(sa.get(2, 2), 2);
}
