#![allow(dead_code)]

// 3545. Minimum Deletions for At Most K Distinct Characters
// https://leetcode.com/problems/minimum-deletions-for-at-most-k-distinct-characters
// https://leetcode.cn/problems/minimum-deletions-for-at-most-k-distinct-characters
//
// Easy
//
// You are given a string s consisting of lowercase English letters, and an integer k.
//
// Your task is to delete some (possibly none) of the characters in the string so that the number
// of distinct characters in the resulting string is at most k.
//
// Return the minimum number of deletions required to achieve this.
//
// Example 1:
//
// Input: s = "abc", k = 2
//
// Output: 1
//
// Explanation:
//
//     s has three distinct characters: 'a', 'b' and 'c', each with a frequency of 1.
//     Since we can have at most k = 2 distinct characters, remove all occurrences of any one character from the string.
//     For example, removing all occurrences of 'c' results in at most k distinct characters. Thus, the answer is 1.
//
// Example 2:
//
// Input: s = "aabb", k = 2
//
// Output: 0
//
// Explanation:
//
//     s has two distinct characters ('a' and 'b') with frequencies of 2 and 2, respectively.
//     Since we can have at most k = 2 distinct characters, no deletions are required. Thus, the answer is 0.
//
// Example 3:
//
// Input: s = "yyyzz", k = 1
//
// Output: 2
//
// Explanation:
//
//     s has two distinct characters ('y' and 'z') with frequencies of 3 and 2, respectively.
//     Since we can have at most k = 1 distinct character, remove all occurrences of any one character from the string.
//     Removing all 'z' results in at most k distinct characters. Thus, the answer is 2.
//
// Constraints:
//
//     1 <= s.length <= 16
//     1 <= k <= 16
//     s consists only of lowercase English letters.
//

struct Solution;

/*
class Solution {
public:
    int minDeletion(string s, int k) {
        unordered_map<char,int>map;
        for(auto value:s)
        {
            cout << value << " ";
            map[value]++;
        }
        if(map.size() <= k)
        {
            return 0;
        }
        int sum = 0;
        int count = map.size();
        while(count != k)
        {
            int mini = INT_MAX;
            int key = -1;
            for(auto i:map)
            {
                if(mini > i.second)
                {
                    key = i.first;
                    mini = i.second;
                }
            }
            sum += map[key];
            map.erase(key);
            count--;
        }
        return sum;
    }
};
*/

impl Solution {
    pub fn min_deletion(s: String, k: i32) -> i32 {
        let mut map = std::collections::HashMap::new();
        for value in s.chars() {
            *map.entry(value).or_insert(0) += 1;
        }
        if map.len() <= k as usize {
            return 0;
        }
        let mut sum = 0;
        let mut count = map.len();
        while count != k as usize {
            let (&key, mini) = map.iter().min_by_key(|&(_, &v)| v).unwrap();
            sum += mini;
            map.remove(&key);
            count -= 1;
        }
        sum
    }
}

#[test]
fn test() {
    assert_eq!(Solution::min_deletion("abc".to_string(), 2), 1);
    assert_eq!(Solution::min_deletion("aabb".to_string(), 2), 0);
    assert_eq!(Solution::min_deletion("yyyzz".to_string(), 1), 2);
}
