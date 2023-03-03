#![allow(dead_code)]

/*

// 2071. Maximum Number of Tasks You Can Assign
// https://leetcode.com/problems/maximum-number-of-tasks-you-can-assign/
// https://leetcode.cn/problems/maximum-number-of-tasks-you-can-assign/
//
// Hard
//
// You have n tasks and m workers. Each task has a strength requirement stored in a 0-indexed integer array tasks, with the ith task requiring tasks[i] strength to complete. The strength of each worker is stored in a 0-indexed integer array workers, with the jth worker having workers[j] strength. Each worker can only be assigned to a single task and must have a strength greater than or equal to the task's strength requirement (i.e., workers[j] >= tasks[i]).

Additionally, you have pills magical pills that will increase a worker's strength by strength. You can decide which workers receive the magical pills, however, you may only give each worker at most one magical pill.

Given the 0-indexed integer arrays tasks and workers and the integers pills and strength, return the maximum number of tasks that can be completed.

Example 1:

Input: tasks = [3,2,1], workers = [0,3,3], pills = 1, strength = 1
Output: 3
Explanation:
We can assign the magical pill and tasks as follows:
- Give the magical pill to worker 0.
- Assign worker 0 to task 2 (0 + 1 >= 1)
- Assign worker 1 to task 1 (3 >= 2)
- Assign worker 2 to task 0 (3 >= 3)

Example 2:

Input: tasks = [5,4], workers = [0,0,0], pills = 1, strength = 5
Output: 1
Explanation:
We can assign the magical pill and tasks as follows:
- Give the magical pill to worker 0.
- Assign worker 0 to task 0 (0 + 5 >= 5)

Example 3:

Input: tasks = [10,15,30], workers = [0,10,10,10,10], pills = 3, strength = 10
Output: 2
Explanation:
We can assign the magical pills and tasks as follows:
- Give the magical pill to worker 0 and worker 1.
- Assign worker 0 to task 0 (0 + 10 >= 10)
- Assign worker 1 to task 1 (10 + 10 >= 15)
The last pill is not given because it will not make any worker strong enough for the last task.

Constraints:

    n == tasks.length
    m == workers.length
    1 <= n, m <= 5 * 10^4
    0 <= pills <= m
    0 <= tasks[i], workers[j], strength <= 10^9
*/

/*
class Solution {
public:
    int maxTaskAssign(vector<int>& tasks, vector<int>& workers, int pills, int strength) {
        int n = tasks.size();
        int m = workers.size();
        sort(tasks.begin(), tasks.end());
        map<int,int> count;
        for(auto &strength : workers) count[strength]++;
        int l = 0, r = n, ans = 0;
        while(l<=r){
            int mid  = l + (r-l)/2;
            int chk  = check(tasks,mid,count,pills,strength);
            if(chk) {
                ans = mid;
                l   = mid+1;
            }
            else {
                r = mid-1;
            }
        }
        return ans;
    }
    int check(vector<int> &tasks, int take, map<int,int> count, int pills, int power){
        while(take>=1 && count.size()){
            auto it = count.end(); --it;
            if(tasks[take-1] <= it->first) {}
            else if(pills) {
                it = count.lower_bound(tasks[take-1]-power);
                if(it==count.end()) return 0;
                --pills;
            }
            else return 0;
            --take;
            (it->second)--;
            if(it->second == 0)
                count.erase(it);
        }
        return take==0;
    }
};
 */

struct Solution;

impl Solution {
    pub fn max_task_assign(tasks: Vec<i32>, workers: Vec<i32>, pills: i32, strength: i32) -> i32 {
        use std::collections::BTreeMap;
        fn check(tasks: &[i32], mut take: usize, mut count: BTreeMap<i32, i32>, mut pills: i32, power: i32) -> bool {
            while take >= 1 && !count.is_empty() {
                let mut idx = count.len() - 1;
                let mut it = count.iter_mut().nth(idx).unwrap();
                if tasks[take - 1] <= *it.0 {
                } else if pills > 0 {
                    let v = tasks[take - 1] - power;
                    let it2 = lower_bound(&count, &v);
                    if it2.is_err() {
                        return false;
                    }
                    idx = it2.unwrap();
                    it = count.iter_mut().nth(idx).unwrap();
                    pills -= 1;
                } else {
                    return false;
                }
                take -= 1;
                *it.1 -= 1;
                let (it_0, it_1) = (*it.0, *it.1);
                if it_1 == 0 {
                    count.remove(&it_0);
                }
            }
            take == 0
        }

        fn lower_bound<K: Ord, V>(btm: &BTreeMap<K, V>, x: &K) -> Result<usize, usize> {
            use std::cmp::Ordering::Less;
            btm.iter().position(|(k, _)| k.cmp(x) != Less).ok_or(btm.len())
        }

        let mut tasks = tasks;
        let n = tasks.len();
        tasks.sort();
        let mut count = BTreeMap::new();
        for strength in workers {
            *count.entry(strength).or_insert(0) += 1;
        }
        let mut l = 0;
        let mut r = n;
        let mut ans = 0;
        while l <= r {
            let mid = l + (r - l) / 2;
            let chk = check(&tasks, mid, count.clone(), pills, strength);
            if chk {
                ans = mid;
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        ans as i32
    }
}

#[test]
fn test() {
    let cases = vec![
        (vec![3, 2, 1], vec![0, 3, 3], 1, 1, 3),
        (vec![5, 4], vec![0, 0, 0], 1, 5, 1),
        (vec![10, 15, 30], vec![0, 10, 10, 10, 10], 3, 10, 2),
    ];
    for (t, w, pills, strength, expected) in cases {
        assert_eq!(Solution::max_task_assign(t, w, pills, strength), expected);
    }
}
