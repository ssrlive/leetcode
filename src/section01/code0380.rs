#![allow(dead_code)]

// 380. Insert Delete GetRandom O(1)
// https://leetcode.com/problems/insert-delete-getrandom-o1/
// https://leetcode.cn/problems/insert-delete-getrandom-o1/
//
// Implement the RandomizedSet class:
//
// - RandomizedSet() Initializes the RandomizedSet object.
// - bool insert(int val) Inserts an item val into the set if not present. Returns true if the item was not present, false otherwise.
// - bool remove(int val) Removes an item val from the set if present. Returns true if the item was present, false otherwise.
// - int getRandom() Returns a random element from the current set of elements (it's guaranteed that at least one element exists when this method is called). Each element must have the same probability of being returned.
//
// You must implement the functions of the class such that each function works in average O(1) time complexity.
//
// Example 1:
//
// Input
// ["RandomizedSet", "insert", "remove", "insert", "getRandom", "remove", "insert", "getRandom"]
// [[], [1], [2], [2], [], [1], [2], []]
// Output
// [null, true, false, true, 2, true, false, 2]
//
// Explanation
// RandomizedSet randomizedSet = new RandomizedSet();
// randomizedSet.insert(1); // Inserts 1 to the set. Returns true as 1 was inserted successfully.
// randomizedSet.remove(2); // Returns false as 2 does not exist in the set.
// randomizedSet.insert(2); // Inserts 2 to the set, returns true. Set now contains [1,2].
// randomizedSet.getRandom(); // getRandom() should return either 1 or 2 randomly.
// randomizedSet.remove(1); // Removes 1 from the set, returns true. Set now contains [2].
// randomizedSet.insert(2); // 2 was already in the set, so return false.
// randomizedSet.getRandom(); // Since 2 is the only number in the set, getRandom() will always return 2.
//
// Constraints:
//
// - -2^31 <= val <= 2^31 - 1
// - At most 2 * 105 calls will be made to insert, remove, and getRandom.
// - There will be at least one element in the data structure when getRandom is called.
//

struct RandomizedSet {
    map: std::collections::HashMap<i32, usize>,
    vec: Vec<i32>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            map: std::collections::HashMap::new(),
            vec: Vec::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if let std::collections::hash_map::Entry::Vacant(e) = self.map.entry(val) {
            e.insert(self.vec.len());
            self.vec.push(val);
            true
        } else {
            false
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        if let Some(&index) = self.map.get(&val) {
            let last = self.vec.pop().unwrap();
            if index != self.vec.len() {
                self.vec[index] = last;
                self.map.insert(last, index);
            }
            self.map.remove(&val);
            true
        } else {
            false
        }
    }

    fn get_random(&self) -> i32 {
        let index = rand::Rng::random::<u64>(&mut rand::rng()) as usize % self.vec.len();
        self.vec[index]
    }
}

#[test]
fn test() {
    let mut randomized_set = RandomizedSet::new();
    assert!(randomized_set.insert(1));
    assert!(!randomized_set.remove(2));
    assert!(randomized_set.insert(2));
    let v = randomized_set.get_random();
    assert!(v == 1 || v == 2);
    assert!(randomized_set.remove(1));
    assert!(!randomized_set.insert(2));
    assert_eq!(randomized_set.get_random(), 2);
}
