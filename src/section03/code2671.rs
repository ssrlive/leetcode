#![allow(dead_code)]

// 2671. Frequency Tracker
// https://leetcode.com/problems/frequency-tracker/
// https://leetcode.cn/problems/frequency-tracker/
//
// Medium
//
// Design a data structure that keeps track of the values in it and answers some queries regarding their frequencies.
//
// Implement the FrequencyTracker class.
//
//     FrequencyTracker(): Initializes the FrequencyTracker object with an empty array initially.
//     void add(int number): Adds number to the data structure.
//     void deleteOne(int number): Deletes one occurrence of number from the data structure. The data structure may not contain number, and in this case nothing is deleted.
//     bool hasFrequency(int frequency): Returns true if there is a number in the data structure that occurs frequency number of times, otherwise, it returns false.
//
// Example 1:
//
// Input
// ["FrequencyTracker", "add", "add", "hasFrequency"]
// [[], [3], [3], [2]]
// Output
// [null, null, null, true]
//
// Explanation
// FrequencyTracker frequencyTracker = new FrequencyTracker();
// frequencyTracker.add(3); // The data structure now contains [3]
// frequencyTracker.add(3); // The data structure now contains [3, 3]
// frequencyTracker.hasFrequency(2); // Returns true, because 3 occurs twice
//
// Example 2:
//
// Input
// ["FrequencyTracker", "add", "deleteOne", "hasFrequency"]
// [[], [1], [1], [1]]
// Output
// [null, null, null, false]
//
// Explanation
// FrequencyTracker frequencyTracker = new FrequencyTracker();
// frequencyTracker.add(1); // The data structure now contains [1]
// frequencyTracker.deleteOne(1); // The data structure becomes empty []
// frequencyTracker.hasFrequency(1); // Returns false, because the data structure is empty
//
// Example 3:
//
// Input
// ["FrequencyTracker", "hasFrequency", "add", "hasFrequency"]
// [[], [2], [3], [1]]
// Output
// [null, false, null, true]
//
// Explanation
// FrequencyTracker frequencyTracker = new FrequencyTracker();
// frequencyTracker.hasFrequency(2); // Returns false, because the data structure is empty
// frequencyTracker.add(3); // The data structure now contains [3]
// frequencyTracker.hasFrequency(1); // Returns true, because 3 occurs once
//
// Constraints:
//
//     1 <= number <= 10^5
//     1 <= frequency <= 10^5
//     At most, 2 * 10^5 calls will be made to add, deleteOne, and hasFrequency in total.
//

pub struct FrequencyTracker {
    numbers: std::collections::HashMap<i32, i32>,
    frequencies: std::collections::HashMap<i32, i32>,
}

impl FrequencyTracker {
    pub fn new() -> Self {
        Self {
            numbers: std::collections::HashMap::new(),
            frequencies: std::collections::HashMap::new(),
        }
    }

    fn update(&mut self, number: i32, dir: i32) {
        let old_feq = *self.numbers.entry(number).or_default();
        self.frequencies.entry(old_feq).and_modify(|e| *e -= 1);
        self.numbers.entry(number).and_modify(|e| *e += dir).or_insert(1);
        self.frequencies.entry(old_feq + dir).and_modify(|e| *e += 1).or_insert(1);
    }

    pub fn add(&mut self, number: i32) {
        self.update(number, 1);
    }

    fn delete_one(&mut self, number: i32) {
        if self.numbers.contains_key(&number) && self.numbers[&number] > 0 {
            self.update(number, -1);
        }
    }

    fn has_frequency(&self, frequency: i32) -> bool {
        self.frequencies.contains_key(&frequency) && self.frequencies[&frequency] > 0
    }
}

#[test]
fn test() {
    let mut frequency_tracker = FrequencyTracker::new();
    frequency_tracker.add(3);
    frequency_tracker.add(3);
    assert!(frequency_tracker.has_frequency(2));

    let mut frequency_tracker = FrequencyTracker::new();
    frequency_tracker.add(1);
    frequency_tracker.delete_one(1);
    assert!(!frequency_tracker.has_frequency(1));

    let mut frequency_tracker = FrequencyTracker::new();
    assert!(!frequency_tracker.has_frequency(2));
    frequency_tracker.add(3);
    assert!(frequency_tracker.has_frequency(1));
}
