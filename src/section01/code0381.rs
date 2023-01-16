#![allow(dead_code)]

// 381. Insert Delete GetRandom O(1) - Duplicates allowed
// https://leetcode.com/problems/insert-delete-getrandom-o1-duplicates-allowed/
// https://leetcode.cn/problems/insert-delete-getrandom-o1-duplicates-allowed/
//
// RandomizedCollection is a data structure that contains a collection of numbers, possibly duplicates (i.e., a multiset). It should support inserting and removing specific elements and also removing a random element.
//
// Implement the RandomizedCollection class:
//
// - RandomizedCollection() Initializes the empty RandomizedCollection object.
// - bool insert(int val) Inserts an item val into the multiset, even if the item is already present. Returns true if the item is not present, false otherwise.
// - bool remove(int val) Removes an item val from the multiset if present. Returns true if the item is present, false otherwise. Note that if val has multiple occurrences in the multiset, we only remove one of them.
// - int getRandom() Returns a random element from the current multiset of elements. The probability of each element being returned is linearly related to the number of same values the multiset contains.
//
// You must implement the functions of the class such that each function works on average O(1) time complexity.
//
// Note: The test cases are generated such that getRandom will only be called if there is at least one item in the RandomizedCollection.
//
// Example 1:
//
// Input
// ["RandomizedCollection", "insert", "insert", "insert", "getRandom", "remove", "getRandom"]
// [[], [1], [1], [2], [], [1], []]
// Output
// [null, true, false, true, 2, true, 1]
//
// Explanation
// RandomizedCollection randomizedCollection = new RandomizedCollection();
// randomizedCollection.insert(1);   // return true since the collection does not contain 1.
//                                   // Inserts 1 into the collection.
// randomizedCollection.insert(1);   // return false since the collection contains 1.
//                                   // Inserts another 1 into the collection. Collection now contains [1,1].
// randomizedCollection.insert(2);   // return true since the collection does not contain 2.
//                                   // Inserts 2 into the collection. Collection now contains [1,1,2].
// randomizedCollection.getRandom(); // getRandom should:
//                                   // - return 1 with probability 2/3, or
//                                   // - return 2 with probability 1/3.
// randomizedCollection.remove(1);   // return true since the collection contains 1.
//                                   // Removes 1 from the collection. Collection now contains [1,2].
// randomizedCollection.getRandom(); // getRandom should return 1 or 2, both equally likely.
//
// Constraints:
//
// - -2^31 <= val <= 2^31 - 1
// - At most 2 * 105 calls in total will be made to insert, remove, and getRandom.
// - There will be at least one element in the data structure when getRandom is called.
//

pub struct RandomizedCollection {
    pub data: Vec<i32>,
    pub mp: std::collections::HashMap<i32, std::collections::BinaryHeap<usize>>,
}

impl RandomizedCollection {
    pub fn new() -> Self {
        Self {
            data: vec![],
            mp: std::collections::HashMap::new(),
        }
    }

    pub fn insert(&mut self, val: i32) -> bool {
        let ret = !self.mp.contains_key(&val);
        self.mp.entry(val).or_default().push(self.data.len());
        self.data.push(val);
        ret
    }

    pub fn remove(&mut self, val: i32) -> bool {
        self._remove(val).unwrap_or(false)
    }
    fn _remove(&mut self, val: i32) -> Option<bool> {
        if !self.mp.contains_key(&val) {
            return Some(false);
        }
        let v = self.mp.get_mut(&val)?;
        let i = v.pop()?;
        if v.is_empty() {
            self.mp.remove(&val);
        }
        let n = self.data.len();
        if i < n - 1 {
            let w = self.mp.get_mut(&self.data[n - 1])?;
            self.data[i] = self.data[n - 1];
            w.pop();
            w.push(i);
        }
        self.data.pop();
        Some(true)
    }

    pub fn get_random(&self) -> i32 {
        // use rand::Rng;
        // self.data[rand::thread_rng().gen_range(0..self.data.len())]
        let index = rand::random::<usize>() % self.data.len();
        self.data[index]
    }
}

#[test]
fn test() {
    let mut col = RandomizedCollection::new();
    assert_eq!(col.insert(1), true);
    assert_eq!(col.insert(1), false);
    assert_eq!(col.insert(2), true);
    let v = col.get_random();
    assert!(v == 2 || v == 1);
    assert_eq!(col.remove(1), true);
    let v = col.get_random();
    assert!(v == 2 || v == 1);
}
