#![allow(dead_code)]

/*

// 2029. Stone Game IX
// https://leetcode.com/problems/stone-game-ix/
// https://leetcode.cn/problems/stone-game-ix/
//
// Medium
//
// Alice and Bob continue their games with stones. There is a row of n stones, and each stone has an associated value. You are given an integer array stones, where stones[i] is the value of the ith stone.

Alice and Bob take turns, with Alice starting first. On each turn, the player may remove any stone from stones. The player who removes a stone loses if the sum of the values of all removed stones is divisible by 3. Bob will win automatically if there are no remaining stones (even if it is Alice's turn).

Assuming both players play optimally, return true if Alice wins and false if Bob wins.

Example 1:

Input: stones = [2,1]
Output: true
Explanation: The game will be played as follows:
- Turn 1: Alice can remove either stone.
- Turn 2: Bob removes the remaining stone.
The sum of the removed stones is 1 + 2 = 3 and is divisible by 3. Therefore, Bob loses and Alice wins the game.

Example 2:

Input: stones = [2]
Output: false
Explanation: Alice will remove the only stone, and the sum of the values on the removed stones is 2.
Since all the stones are removed and the sum of values is not divisible by 3, Bob wins the game.

Example 3:

Input: stones = [5,1,2,4,3]
Output: false
Explanation: Bob will always win. One possible way for Bob to win is shown below:
- Turn 1: Alice can remove the second stone with value 1. Sum of removed stones = 1.
- Turn 2: Bob removes the fifth stone with value 3. Sum of removed stones = 1 + 3 = 4.
- Turn 3: Alices removes the fourth stone with value 4. Sum of removed stones = 1 + 3 + 4 = 8.
- Turn 4: Bob removes the third stone with value 2. Sum of removed stones = 1 + 3 + 4 + 2 = 10.
- Turn 5: Alice removes the first stone with value 5. Sum of removed stones = 1 + 3 + 4 + 2 + 5 = 15.
Alice loses the game because the sum of the removed stones (15) is divisible by 3. Bob wins the game.

Constraints:

    1 <= stones.length <= 10^5
    1 <= stones[i] <= 10^4
*/

struct Solution;

impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        use std::cmp::{max, min};
        let (n0, n1, n2) = stones
            .into_iter()
            .map(|stone| ((stone % 3 == 0), (stone % 3 == 1), (stone % 3 == 2)))
            .map(|(n0, n1, n2)| (n0 as usize, n1 as usize, n2 as usize))
            .fold((0, 0, 0), |(nacc0, nacc1, nacc2), (n0, n1, n2)| {
                (nacc0 + n0, nacc1 + n1, nacc2 + n2)
            });

        let nx = max(n1, n2);
        let nn = min(n1, n2);

        (nn != 0 && n0.is_multiple_of(2)) || (nx > nn + 2 && n0 % 2 == 1)
    }
}

#[test]
fn test() {
    assert!(Solution::stone_game_ix(vec![2, 1]));
    assert!(!Solution::stone_game_ix(vec![2]));
    assert!(!Solution::stone_game_ix(vec![5, 1, 2, 4, 3]));
    assert!(!Solution::stone_game_ix(vec![1, 2, 3]));
}
