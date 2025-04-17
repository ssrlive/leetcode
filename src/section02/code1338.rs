#![allow(dead_code)]

// 1338. Reduce Array Size to The Half
// https://leetcode.com/problems/reduce-array-size-to-the-half/
// https://leetcode.cn/problems/reduce-array-size-to-the-half/
//
// Medium
//
// You are given an integer array arr. You can choose a set of integers and remove all the occurrences of these integers in the array.
//
// Return the minimum size of the set so that at least half of the integers of the array are removed.
//
// Example 1:
//
// Input: arr = [3,3,3,3,5,5,5,2,2,7]
// Output: 2
// Explanation: Choosing {3,7} will make the new array [5,5,5,2,2] which has size 5 (i.e equal to half of the size of the old array).
// Possible sets of size 2 are {3,5},{3,2},{5,2}.
// Choosing set {2,7} is not possible as it will make the new array [3,3,3,3,5,5,5] which has a size greater than half of the size of the old array.
//
// Example 2:
//
// Input: arr = [7,7,7,7,7,7]
// Output: 1
// Explanation: The only possible set you can choose is {7}. This will make the new array empty.
//
// Constraints:
//
// -    2 <= arr.length <= 10^5
// -    arr.length is even.
// -    1 <= arr[i] <= 10^5
//

struct Solution;

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let n = arr.len();
        // Count frequencies
        let (map, max_freq) = arr
            .into_iter()
            .fold((HashMap::<i32, usize>::with_capacity(n), 0), |(mut map, max_freq), value| {
                let entry = map.entry(value).or_default();
                *entry += 1;
                let freq = *entry;
                (map, max_freq.max(freq))
            });
        // Bucket sort
        let freqs = map.into_iter().fold(vec![0; max_freq + 1], |mut freqs, (_, freq)| {
            freqs[freq] += 1;
            freqs
        });

        // Take greedily
        let mut rez = 0;
        let mut left = n.div_ceil(2);
        for freq in (1..=max_freq).rev() {
            let available = freqs[freq] * freq;
            if available >= left {
                // (left + freq - 1) / freq is left / freq rounded up to nearest integer
                return (rez + left.div_ceil(freq)) as _;
            }
            left -= available;
            rez += freqs[freq];
        }
        unreachable!()
    }
}

#[test]
fn test() {
    let arr = vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7];
    assert_eq!(Solution::min_set_size(arr), 2);

    let arr = vec![7, 7, 7, 7, 7, 7];
    assert_eq!(Solution::min_set_size(arr), 1);
}
