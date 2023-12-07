#![allow(dead_code)]

// 2954. Count the Number of Infection Sequences
// https://leetcode.com/problems/count-the-number-of-infection-sequences/
// https://leetcode.cn/problems/count-the-number-of-infection-sequences/
//
// Hard
//
// You are given an integer n and a 0-indexed integer array sick which is sorted in increasing order.
//
// There are n children standing in a queue with positions 0 to n - 1 assigned to them.
// The array sick contains the positions of the children who are infected with an infectious disease.
// An infected child at position i can spread the disease to either of its immediate neighboring children
// at positions i - 1 and i + 1 if they exist and are currently not infected.
// At most one child who was previously not infected can get infected with the disease in one second.
//
// It can be shown that after a finite number of seconds, all the children in the queue will get infected with the disease.
// An infection sequence is the sequential order of positions in which all of the non-infected children get infected with the disease.
// Return the total number of possible infection sequences.
//
// Since the answer may be large, return it modulo 109 + 7.
//
// Note that an infection sequence does not contain positions of children who were already infected with the disease in the beginning.
//
// Example 1:
//
// Input: n = 5, sick = [0,4]
// Output: 4
// Explanation: Children at positions 1, 2, and 3 are not infected in the beginning. There are 4 possible infection sequences:
// - The children at positions 1 and 3 can get infected since their positions are adjacent to the infected children 0 and 4. The child at position 1 gets infected first.
// Now, the child at position 2 is adjacent to the child at position 1 who is infected and the child at position 3 is adjacent
// to the child at position 4 who is infected, hence either of them can get infected. The child at position 2 gets infected.
// Finally, the child at position 3 gets infected because it is adjacent to children at positions 2 and 4 who are infected. The infection sequence is [1,2,3].
// - The children at positions 1 and 3 can get infected because their positions are adjacent to the infected children 0 and 4. The child at position 1 gets infected first.
// Now, the child at position 2 is adjacent to the child at position 1 who is infected and the child at position 3 is adjacent
// to the child at position 4 who is infected, hence either of them can get infected. The child at position 3 gets infected.
// Finally, the child at position 2 gets infected because it is adjacent to children at positions 1 and 3 who are infected. The infection sequence is [1,3,2].
// - The infection sequence is [3,1,2]. The order of infection of disease in the children can be seen as: [0,1,2,3,4] => [0,1,2,3,4] => [0,1,2,3,4] => [0,1,2,3,4].
// - The infection sequence is [3,2,1]. The order of infection of disease in the children can be seen as: [0,1,2,3,4] => [0,1,2,3,4] => [0,1,2,3,4] => [0,1,2,3,4].
//
// Example 2:
//
// Input: n = 4, sick = [1]
// Output: 3
// Explanation: Children at positions 0, 2, and 3 are not infected in the beginning. There are 3 possible infection sequences:
// - The infection sequence is [0,2,3]. The order of infection of disease in the children can be seen as: [0,1,2,3] => [0,1,2,3] => [0,1,2,3] => [0,1,2,3].
// - The infection sequence is [2,0,3]. The order of infection of disease in the children can be seen as: [0,1,2,3] => [0,1,2,3] => [0,1,2,3] => [0,1,2,3].
// - The infection sequence is [2,3,0]. The order of infection of disease in the children can be seen as: [0,1,2,3] => [0,1,2,3] => [0,1,2,3] => [0,1,2,3].
//
// Constraints:
//
//     2 <= n <= 10^5
//     1 <= sick.length <= n - 1
//     0 <= sick[i] <= n - 1
//     sick is sorted in increasing order.
//

struct Solution;

impl Solution {
    const MOD: i64 = 1_000_000_007;
    pub fn number_of_sequence(n: i32, sick: Vec<i32>) -> i32 {
        let m = 100_000;
        let mut f = vec![0; m + 1];
        let mut v = vec![0; m + 1];

        f[0] = 1;

        for i in 1..=m {
            f[i] = (f[i - 1] * (i as i64)) % Self::MOD;
        }

        v[m] = Self::mod_pow(f[m], Self::MOD - 2);

        for i in (0..m).rev() {
            v[i] = (v[i + 1] * (i as i64 + 1)) % Self::MOD;
        }

        let mut r = 1;

        for i in 1..sick.len() {
            let g = (sick[i] - sick[i - 1] - 1) as i64;
            r = (r * Self::mod_pow(2, 0.max(g - 1))) % Self::MOD;
            let l = (sick[i] - i as i32) as i64;
            r = (r * Self::binom(l, g, &f, &v)) % Self::MOD;
        }

        let l = (n - sick.len() as i32) as i64;
        let p = sick.last().unwrap();
        let q = (n - p - 1) as i64;

        r = (r * Self::binom(l, q, &f, &v)) % Self::MOD;

        r as i32
    }

    pub fn mod_pow(x: i64, n: i64) -> i64 {
        if n == 0 {
            1 % Self::MOD
        } else {
            let mut u = Self::mod_pow(x, n / 2);
            u = (u * u) % Self::MOD;
            if n % 2 > 0 {
                u = (u * x) % Self::MOD;
            }
            u % Self::MOD
        }
    }

    pub fn binom(n: i64, k: i64, f: &[i64], v: &[i64]) -> i64 {
        1.max((((f[n as usize] * v[k as usize]) % Self::MOD) * v[n as usize - k as usize]) % Self::MOD)
    }
}

#[test]
fn test() {
    assert_eq!(Solution::number_of_sequence(5, vec![0, 4]), 4);
    assert_eq!(Solution::number_of_sequence(4, vec![1]), 3);
}
