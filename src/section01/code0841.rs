#![allow(dead_code)]

// 841. Keys and Rooms
// https://leetcode.com/problems/keys-and-rooms/
// https://leetcode.cn/problems/keys-and-rooms/
//
// There are n rooms labeled from 0 to n - 1 and all the rooms are locked except for room 0.
// Your goal is to visit all the rooms. However, you cannot enter a locked room without having its key.
//
// When you visit a room, you may find a set of distinct keys in it. Each key has a number on it,
// denoting which room it unlocks, and you can take all of them with you to unlock the other rooms.
//
// Given an array rooms where rooms[i] is the set of keys that you can obtain if you visited room i,
// return true if you can visit all the rooms, or false otherwise.
//
// Example 1:
//
// Input: rooms = [[1],[2],[3],[]]
// Output: true
// Explanation:
// We visit room 0 and pick up key 1.
// We then visit room 1 and pick up key 2.
// We then visit room 2 and pick up key 3.
// We then visit room 3.
// Since we were able to visit every room, we return true.
//
// Example 2:
//
// Input: rooms = [[1,3],[3,0,1],[2],[0]]
// Output: false
// Explanation: We can not enter room number 2 since the only key that unlocks it is in that room.
//
// Constraints:
//
// - n == rooms.length
// - 2 <= n <= 1000
// - 0 <= rooms[i].length <= 1000
// - 1 <= sum(rooms[i].length) <= 3000
// - 0 <= rooms[i][j] < n
// - All the values of rooms[i] are unique.
//

struct Solution;

impl Solution {
    pub fn can_visit_all_rooms(rooms: Vec<Vec<i32>>) -> bool {
        let mut visited = vec![false; rooms.len()];
        let mut stack = vec![0];
        while let Some(room) = stack.pop() {
            if visited[room] {
                continue;
            }
            visited[room] = true;
            for &key in &rooms[room] {
                stack.push(key as usize);
            }
        }
        visited.iter().all(|&v| v)
    }
}

#[test]
fn test() {
    let rooms = vec![vec![1], vec![2], vec![3], vec![]];
    assert!(Solution::can_visit_all_rooms(rooms));

    let rooms = vec![vec![1, 3], vec![3, 0, 1], vec![2], vec![0]];
    assert!(!Solution::can_visit_all_rooms(rooms));
}
