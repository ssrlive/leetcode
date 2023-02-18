#![allow(dead_code)]

/*

// 1711. Count Good Meals
Medium
833
222
Companies

A good meal is a meal that contains exactly two different food items with a sum of deliciousness equal to a power of two.

You can pick any two different foods to make a good meal.

Given an array of integers deliciousness where deliciousness[i] is the deliciousness of the i​​​​​​th​​​​​​​​ item of food, return the number of different good meals you can make from this list modulo 109 + 7.

Note that items with different indices are considered different even if they have the same deliciousness value.

Example 1:

Input: deliciousness = [1,3,5,7,9]
Output: 4
Explanation: The good meals are (1,3), (1,7), (3,5) and, (7,9).
Their respective sums are 4, 8, 8, and 16, all of which are powers of 2.

Example 2:

Input: deliciousness = [1,1,1,3,3,3,7]
Output: 15
Explanation: The good meals are (1,1) with 3 ways, (1,3) with 9 ways, and (1,7) with 3 ways.

Constraints:

    1 <= deliciousness.length <= 10^5
    0 <= deliciousness[i] <= 2^20
*/

struct Solution;

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        fn get_pairs(val: i32, count: &HashMap<i32, i32>) -> i64 {
            let mut answer = 0_i64;
            for i in 0..22 {
                let power_of_two = i32::pow(2, i);
                let target = power_of_two - val;
                if count.contains_key(&target) {
                    let target_count = *count.get(&target).unwrap() as i64;
                    answer += if target == val { target_count - 1 } else { target_count };
                }
            }
            answer
        }

        let mut nums_count: HashMap<i32, i32> = HashMap::new();
        for num in &deliciousness {
            let mut count = 0;
            if nums_count.contains_key(num) {
                count = *nums_count.get(num).unwrap();
            }
            count += 1;
            nums_count.insert(*num, count);
        }

        let mut answer = 0;
        for num in deliciousness {
            answer += get_pairs(num, &nums_count);
        }
        answer /= 2;
        let mod_num = i64::pow(10, 9) + 7;
        answer %= mod_num;
        answer as _
    }
}

#[test]
fn test() {}
