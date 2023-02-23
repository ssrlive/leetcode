#![allow(dead_code)]

/*

// 1847. Closest Room
// https://leetcode.com/problems/closest-room/
// https://leetcode.cn/problems/closest-room/
//
// Hard
//
// There is a hotel with n rooms. The rooms are represented by a 2D integer array rooms where rooms[i] = [roomIdi, sizei] denotes that there is a room with room number roomIdi and size equal to sizei. Each roomIdi is guaranteed to be unique.

You are also given k queries in a 2D array queries where queries[j] = [preferredj, minSizej]. The answer to the jth query is the room number id of a room such that:

    The room has a size of at least minSizej, and
    abs(id - preferredj) is minimized, where abs(x) is the absolute value of x.

If there is a tie in the absolute difference, then use the room with the smallest such id. If there is no such room, the answer is -1.

Return an array answer of length k where answer[j] contains the answer to the jth query.

Example 1:

Input: rooms = [[2,2],[1,2],[3,2]], queries = [[3,1],[3,3],[5,2]]
Output: [3,-1,3]
Explanation: The answers to the queries are as follows:
Query = [3,1]: Room number 3 is the closest as abs(3 - 3) = 0, and its size of 2 is at least 1. The answer is 3.
Query = [3,3]: There are no rooms with a size of at least 3, so the answer is -1.
Query = [5,2]: Room number 3 is the closest as abs(3 - 5) = 2, and its size of 2 is at least 2. The answer is 3.

Example 2:

Input: rooms = [[1,4],[2,3],[3,5],[4,1],[5,2]], queries = [[2,3],[2,4],[2,5]]
Output: [2,1,3]
Explanation: The answers to the queries are as follows:
Query = [2,3]: Room number 2 is the closest as abs(2 - 2) = 0, and its size of 3 is at least 3. The answer is 2.
Query = [2,4]: Room numbers 1 and 3 both have sizes of at least 4. The answer is 1 since it is smaller.
Query = [2,5]: Room number 3 is the only room with a size of at least 5. The answer is 3.

Constraints:

    n == rooms.length
    1 <= n <= 10^5
    k == queries.length
    1 <= k <= 10^4
    1 <= roomIdi, preferredj <= 10^7
    1 <= sizei, minSizej <= 10^7
*/

struct Solution;

impl Solution {
    pub fn closest_room(rooms: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        use std::collections::BTreeSet;
        fn search_closest_room_id(tree_set: &BTreeSet<i32>, preferred_id: i32) -> i32 {
            let floor = tree_set.range(..=preferred_id).next_back();
            let ceiling = tree_set.range(preferred_id..).next();
            let mut ans_abs = i32::MAX;
            let mut ans = -1;
            if let Some(&floor) = floor {
                ans = floor;
                ans_abs = (preferred_id - floor).abs();
            }
            if let Some(&ceiling) = ceiling {
                if ans_abs > (preferred_id - ceiling).abs() {
                    ans = ceiling;
                }
            }
            ans
        }
        let mut rooms = rooms;
        rooms.sort_by(|a, b| b[1].cmp(&a[1]));
        let mut indexes = (0..queries.len()).collect::<Vec<_>>();
        indexes.sort_by(|a, b| queries[*b][1].cmp(&queries[*a][1]));
        let mut room_ids_so_far = BTreeSet::new();
        let mut ans = vec![-1; queries.len()];
        let mut i = 0;
        for index in indexes {
            while i < rooms.len() && rooms[i][1] >= queries[index][1] {
                room_ids_so_far.insert(rooms[i][0]);
                i += 1;
            }
            ans[index] = search_closest_room_id(&room_ids_so_far, queries[index][0]);
        }
        ans
    }
}

#[test]
fn test() {
    let cases = vec![
        (
            vec![vec![2, 2], vec![1, 2], vec![3, 2]],
            vec![vec![3, 1], vec![3, 3], vec![5, 2]],
            vec![3, -1, 3],
        ),
        (
            vec![vec![1, 4], vec![2, 3], vec![3, 5], vec![4, 1], vec![5, 2]],
            vec![vec![2, 3], vec![2, 4], vec![2, 5]],
            vec![2, 1, 3],
        ),
    ];
    for (rooms, queries, expected) in cases {
        assert_eq!(Solution::closest_room(rooms, queries), expected);
    }
}
