#![allow(dead_code)]

// 1402. Reducing Dishes
// https://leetcode.com/problems/reducing-dishes/
// https://leetcode.cn/problems/reducing-dishes/
//
// Hard
//
// A chef has collected data on the satisfaction level of his n dishes. Chef can cook any dish in 1 unit of time.
//
// Like-time coefficient of a dish is defined as the time taken to cook that dish including previous
// dishes multiplied by its satisfaction level i.e. time[i] * satisfaction[i].
//
// Return the maximum sum of like-time coefficient that the chef can obtain after dishes preparation.
//
// Dishes can be prepared in any order and the chef can discard some dishes to get this maximum value.
//
// Example 1:
//
// Input: satisfaction = [-1,-8,0,5,-9]
// Output: 14
// Explanation: After Removing the second and last dish, the maximum total like-time coefficient will be equal to (-1*1 + 0*2 + 5*3 = 14).
// Each dish is prepared in one unit of time.
//
// Example 2:
//
// Input: satisfaction = [4,3,2]
// Output: 20
// Explanation: Dishes can be prepared in any order, (2*1 + 3*2 + 4*3 = 20)
//
// Example 3:
//
// Input: satisfaction = [-1,-4,-5]
// Output: 0
// Explanation: People do not like the dishes. No dish is prepared.
//
// Constraints:
//
// -    n == satisfaction.length
// -    1 <= n <= 500
// -    -1000 <= satisfaction[i] <= 1000
//

struct Solution;

impl Solution {
    pub fn max_satisfaction(satisfaction: Vec<i32>) -> i32 {
        let mut satisfaction = satisfaction;
        satisfaction.sort();
        let mut sum = 0;
        let mut total = 0;
        for i in (0..satisfaction.len()).rev() {
            sum += satisfaction[i];
            if sum > 0 {
                total += sum;
            } else {
                break;
            }
        }
        total
    }
}

#[test]
fn test() {
    assert_eq!(Solution::max_satisfaction(vec![-1, -8, 0, 5, -9]), 14);
    assert_eq!(Solution::max_satisfaction(vec![4, 3, 2]), 20);
    assert_eq!(Solution::max_satisfaction(vec![-1, -4, -5]), 0);
}
