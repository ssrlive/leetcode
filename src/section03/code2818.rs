#![allow(dead_code)]

// 2818. Apply Operations to Maximize Score
// https://leetcode.com/problems/apply-operations-to-maximize-score/
// https://leetcode.cn/problems/apply-operations-to-maximize-score/
//
// Hard
//
// You are given an array nums of n positive integers and an integer k.
//
// Initially, you start with a score of 1. You have to maximize your score by applying the following operation at most k times:
//
// Choose any non-empty subarray nums[l, ..., r] that you haven't chosen previously.
// Choose an element x of nums[l, ..., r] with the highest prime score. If multiple such elements exist, choose the one with the smallest index.
// Multiply your score by x.
// Here, nums[l, ..., r] denotes the subarray of nums starting at index l and ending at the index r, both ends being inclusive.
//
// The prime score of an integer x is equal to the number of distinct prime factors of x. For example,
// the prime score of 300 is 3 since 300 = 2 * 2 * 3 * 5 * 5.
//
// Return the maximum possible score after applying at most k operations.
//
// Since the answer may be large, return it modulo 109 + 7.
//
// Example 1:
//
// Input: nums = [8,3,9,3,8], k = 2
// Output: 81
// Explanation: To get a score of 81, we can apply the following operations:
// - Choose subarray nums[2, ..., 2]. nums[2] is the only element in this subarray. Hence, we multiply the score by nums[2].
//   The score becomes 1 * 9 = 9.
// - Choose subarray nums[2, ..., 3]. Both nums[2] and nums[3] have a prime score of 1, but nums[2] has the smaller index.
//   Hence, we multiply the score by nums[2]. The score becomes 9 * 9 = 81.
// It can be proven that 81 is the highest score one can obtain.
//
// Example 2:
//
// Input: nums = [19,12,14,6,10,18], k = 3
// Output: 4788
// Explanation: To get a score of 4788, we can apply the following operations:
// - Choose subarray nums[0, ..., 0]. nums[0] is the only element in this subarray.
//   Hence, we multiply the score by nums[0]. The score becomes 1 * 19 = 19.
// - Choose subarray nums[5, ..., 5]. nums[5] is the only element in this subarray.
//   Hence, we multiply the score by nums[5]. The score becomes 19 * 18 = 342.
// - Choose subarray nums[2, ..., 3]. Both nums[2] and nums[3] have a prime score of 2, but nums[2] has the smaller index.
//   Hence, we multipy the score by nums[2]. The score becomes 342 * 14 = 4788.
// It can be proven that 4788 is the highest score one can obtain.
//
// Constraints:
//
// 1 <= nums.length == n <= 10^5
// 1 <= nums[i] <= 10^5
// 1 <= k <= min(n * (n + 1) / 2, 10^9)
//

struct Solution;

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut temp = std::collections::BTreeMap::<i32, Vec<usize>>::new();

        for (i, &num) in nums.iter().enumerate().take(n) {
            let (mut a, mut cnt) = (num, 0);
            for m in 2..=num {
                if m * m > a {
                    break;
                }
                if a % m != 0 {
                    continue;
                }

                cnt += 1;
                while a % m == 0 {
                    a /= m;
                }
            }
            if a > 1 {
                cnt += 1;
            }
            temp.entry(cnt).or_default().push(i);
        }

        let mut s = std::collections::BTreeSet::<usize>::new();
        let mut pq = std::collections::BinaryHeap::new();
        let mut temp = temp.into_iter().map(|it| it.1).collect::<Vec<_>>();

        temp.reverse();
        for v in temp {
            let sz = v.len();

            for j in 0..sz {
                // left: the number of indexes from v[j]'s
                // left (inclusively) that pick nums[v[j]] as x
                let (mut left, mut right) = (v[j] + 1, n - v[j]);

                if j > 0 {
                    left = v[j] - v[j - 1];
                }

                if let Some(idx) = s.range(v[j]..).next() {
                    right = right.min(*idx - v[j]);
                }
                if let Some(idx) = s.range(..v[j]).next_back() {
                    left = left.min(v[j] - *idx);
                }

                pq.push((nums[v[j]] as i64, left as i32 * right as i32));
            }
            for &item in v.iter().take(sz) {
                s.insert(item);
            }
        }

        let (mut ret, mut k) = (1, k);
        const MOD: i64 = 1_000_000_007;

        while let Some((val, cnt)) = pq.pop() {
            ret = (ret * Self::power(val, k.min(cnt) as i64)) % MOD;
            if k <= cnt {
                break;
            }
            k -= cnt;
        }

        ret as _
    }

    fn power(base: i64, p: i64) -> i64 {
        const MOD: i64 = 1_000_000_007;
        let (mut ret, mut base, mut p) = (1, base, p);
        while p > 0 {
            if p % 2 != 0 {
                ret = (ret * base) % MOD;
            }
            base = (base * base) % MOD;
            p /= 2;
        }
        ret
    }
}

#[test]
fn test() {
    assert_eq!(Solution::maximum_score(vec![8, 3, 9, 3, 8], 2), 81);
    assert_eq!(Solution::maximum_score(vec![19, 12, 14, 6, 10, 18], 3), 4788);
}
