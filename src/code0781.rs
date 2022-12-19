#![allow(dead_code)]

// 781. Rabbits in Forest
// https://leetcode.com/problems/rabbits-in-forest/
// https://leetcode.cn/problems/rabbits-in-forest/
//
// There is a forest with an unknown number of rabbits. We asked n rabbits "How many rabbits have the same color as you?"
// and collected the answers in an integer array answers where answers[i] is the answer of the ith rabbit.
//
// Given the array answers, return the minimum number of rabbits that could be in the forest.
//
// Example 1:
//
// Input: answers = [1,1,2]
// Output: 5
// Explanation:
// The two rabbits that answered "1" could both be the same color, say red.
// The rabbit that answered "2" can't be red or the answers would be inconsistent.
// Say the rabbit that answered "2" was blue.
// Then there should be 2 other blue rabbits in the forest that didn't answer into the array.
// The smallest possible number of rabbits in the forest is therefore 5: 3 that answered plus 2 that didn't.
//
// Example 2:
//
// Input: answers = [10,10,10]
// Output: 11
//
// Constraints:
//
// - 1 <= answers.length <= 1000
// - 0 <= answers[i] < 1000
//

struct Solution;

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut map = std::collections::HashMap::new();
        for answer in answers {
            if let Some(&v) = map.get(&answer) {
                if v == 0 {
                    count += answer + 1;
                    map.insert(answer, answer);
                } else {
                    map.insert(answer, v - 1);
                }
            } else {
                count += answer + 1;
                map.insert(answer, answer);
            }
        }
        count
    }
}

#[test]
fn test() {
    assert_eq!(Solution::num_rabbits(vec![1, 1, 2]), 5);
    assert_eq!(Solution::num_rabbits(vec![10, 10, 10]), 11);
}
