#![allow(dead_code)]

/*

// 2007. Find Original Array From Doubled Array
// https://leetcode.com/problems/find-original-array-from-doubled-array/
// https://leetcode.cn/problems/find-original-array-from-doubled-array/
//
// Medium
//
// An integer array original is transformed into a doubled array changed by appending twice the value of every element in original, and then randomly shuffling the resulting array.

Given an array changed, return original if changed is a doubled array. If changed is not a doubled array, return an empty array. The elements in original may be returned in any order.

Example 1:

Input: changed = [1,3,4,2,6,8]
Output: [1,3,4]
Explanation: One possible original array could be [1,3,4]:
- Twice the value of 1 is 1 * 2 = 2.
- Twice the value of 3 is 3 * 2 = 6.
- Twice the value of 4 is 4 * 2 = 8.
Other original arrays could be [4,3,1] or [3,1,4].

Example 2:

Input: changed = [6,3,0,1]
Output: []
Explanation: changed is not a doubled array.

Example 3:

Input: changed = [1]
Output: []
Explanation: changed is not a doubled array.

Constraints:

    1 <= changed.length <= 10^5
    0 <= changed[i] <= 10^5
*/

struct Solution;

impl Solution {
    pub fn find_original_array(changed: Vec<i32>) -> Vec<i32> {
        let mut changed = changed;
        if changed.len() % 2 == 1 {
            return Vec::new();
        }

        let half_size = changed.len() / 2;
        let mut hm = std::collections::HashMap::with_capacity(half_size);
        let mut res = Vec::with_capacity(half_size);

        changed.sort_unstable();
        changed.iter().for_each(|&x| {
            let e = hm.entry(x).or_insert(0);
            match *e == 0 {
                true => {
                    hm.entry(2 * x).and_modify(|n| *n += 1).or_insert(1);
                    res.push(x);
                }
                false => *e -= 1,
            };
        });

        match res.len() == half_size {
            true => res,
            false => vec![],
        }
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![1, 3, 4, 2, 6, 8], vec![1, 3, 4]),
        (vec![6, 3, 0, 1], vec![]),
        (vec![1], vec![]),
    ];
    for (changed, expect) in cases {
        assert_eq!(Solution::find_original_array(changed), expect);
    }
}
