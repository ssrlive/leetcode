#![allow(dead_code)]

// 749. Contain Virus
// https://leetcode.com/problems/contain-virus/
//
// A virus is spreading rapidly, and your task is to quarantine the infected area by installing walls.
//
// The world is modeled as an m x n binary grid isInfected, where isInfected[i][j] == 0 represents uninfected cells,
// and isInfected[i][j] == 1 represents cells contaminated with the virus. A wall (and only one wall) can be installed
// between any two 4-directionally adjacent cells, on the shared boundary.
//
// Every night, the virus spreads to all neighboring cells in all four directions unless blocked by a wall.
// Resources are limited. Each day, you can install walls around only one region (i.e., the affected area (continuous block of infected cells)
// that threatens the most uninfected cells the following night). There will never be a tie.
//
// Return the number of walls used to quarantine all the infected regions. If the world will become fully infected, return the number of walls used.
//
// Example 1:
//
// Input: isInfected = [[0,1,0,0,0,0,0,1],[0,1,0,0,0,0,0,1],[0,0,0,0,0,0,0,1],[0,0,0,0,0,0,0,0]]
// Output: 10
// Explanation: There are 2 contaminated regions.
// On the first day, add 5 walls to quarantine the viral region on the left. The board after the virus spreads is:
//
// On the second day, add 5 walls to quarantine the viral region on the right. The virus is fully contained.
//
// Example 2:
//
// Input: isInfected = [[1,1,1],[1,0,1],[1,1,1]]
// Output: 4
// Explanation: Even though there is only one cell saved, there are 4 walls built.
// Notice that walls are only built on the shared boundary of two different cells.
//
// Example 3:
//
// Input: isInfected = [[1,1,1,0,0,0,0,0,0],[1,0,1,0,1,1,1,1,1],[1,1,1,0,0,0,0,0,0]]
// Output: 13
// Explanation: The region on the left only builds two new walls.
//
// Constraints:
//
// - m == isInfected.length
// - n == isInfected[i].length
// - 1 <= m, n <= 50
// - isInfected[i][j] is either 0 or 1.
// - There is always a contiguous viral region throughout the described process that will infect strictly more uncontaminated squares in the next round.
//

/*
// cpp solution
class Solution {
public:
    vector<vector<int> > g;
    int n, m, c, mx, w, r, ans, itr;
    unordered_set<int> s;

    int dfs(int i, int j){
        if(i<0 || i>=n || j<0 || j>=m || g[i][j]!=1)
            return 0;
        int ans=0;
        if(i+1<n && g[i+1][j]==0){
            s.insert((i+1)*m+j);
            ans++;
        }
        if(i-1>=0 && g[i-1][j]==0){
            s.insert((i-1)*m+j);
            ans++;
        }
        if(j+1<m && g[i][j+1]==0){
            s.insert(i*m+(j+1));
            ans++;
        }
        if(j-1>=0 && g[i][j-1]==0){
            s.insert(i*m+(j-1));
            ans++;
        }
        g[i][j]=c;
        ans+=dfs(i+1, j);
        ans+=dfs(i-1, j);
        ans+=dfs(i, j+1);
        ans+=dfs(i, j-1);
        return ans; // total number of walls needed to block this connected component
    }

    int containVirus(vector<vector<int>>& grid) {
        g=grid, n=g.size(), m=g[0].size(), ans=0;
        while(true){
            c=2, mx=0;
            for(int i=0; i<n; i++){
                for(int j=0; j<m; j++){
                    if(g[i][j]==1){
                        s.clear();
                        int walls=dfs(i, j);
                        if(mx<s.size()){
                            mx=s.size();
                            w=walls;
                            r=c;
                        }
                        c++;
                    }
                }
            }
            if(mx==0)
                break;
            ans+=w;
            for(int i=0; i<n; i++){
                for(int j=0; j<m; j++){
                    if(g[i][j]==r)
                        g[i][j]=1e9;
                    else if(g[i][j]>1 && g[i][j]!=1e9){
                        g[i][j]=1;
                        if(i+1<n && !g[i+1][j]) g[i+1][j]=1;
                        if(i-1>=0 && !g[i-1][j]) g[i-1][j]=1;
                        if(j+1<m && !g[i][j+1]) g[i][j+1]=1;
                        if(j-1>=0 && !g[i][j-1]) g[i][j-1]=1;
                    }
                }
            }
        }
        return ans;
    }
};
*/

struct MySolution {
    infected: Vec<Vec<i32>>,
    n: usize,
    m: usize,
    c: i32,
    mx: usize,
    w: i32,
    r: i32,
    ans: i32,
    itr: i32,
    s: std::collections::HashSet<usize>,
}

impl MySolution {
    fn new() -> Self {
        MySolution {
            infected: vec![vec![]],
            n: 0,
            m: 0,
            c: 2,
            mx: 0,
            w: 0,
            r: 0,
            ans: 0,
            itr: 0,
            s: std::collections::HashSet::new(),
        }
    }

    fn dfs(&mut self, i: i32, j: i32) -> i32 {
        if i < 0 || j < 0 {
            return 0;
        }
        let (i, j) = (i as usize, j as usize);
        if i >= self.n || j >= self.m || self.infected[i][j] != 1 {
            return 0;
        }
        let mut ans = 0;
        if i + 1 < self.n && self.infected[i + 1][j] == 0 {
            self.s.insert((i + 1) * self.m + j);
            ans += 1;
        }
        if i >= 1 && self.infected[i - 1][j] == 0 {
            self.s.insert((i - 1) * self.m + j);
            ans += 1;
        }
        if j + 1 < self.m && self.infected[i][j + 1] == 0 {
            self.s.insert(i * self.m + (j + 1));
            ans += 1;
        }
        if j >= 1 && self.infected[i][j - 1] == 0 {
            self.s.insert(i * self.m + (j - 1));
            ans += 1;
        }
        self.infected[i][j] = self.c;
        ans += self.dfs(i as i32 + 1, j as i32);
        ans += self.dfs(i as i32 - 1, j as i32);
        ans += self.dfs(i as i32, j as i32 + 1);
        ans += self.dfs(i as i32, j as i32 - 1);
        ans
    }

    fn contain_virus(&mut self, infected: Vec<Vec<i32>>) -> i32 {
        self.infected = infected;
        self.n = self.infected.len();
        self.m = self.infected[0].len();
        self.ans = 0;
        loop {
            self.c = 2;
            self.mx = 0;
            for i in 0..self.n {
                for j in 0..self.m {
                    if self.infected[i][j] == 1 {
                        self.s.clear();
                        let walls = self.dfs(i as i32, j as i32);
                        if self.mx < self.s.len() {
                            self.mx = self.s.len();
                            self.w = walls;
                            self.r = self.c;
                        }
                        self.c += 1;
                    }
                }
            }
            if self.mx == 0 {
                break;
            }
            self.ans += self.w;
            for i in 0..self.n {
                for j in 0..self.m {
                    if self.infected[i][j] == self.r {
                        self.infected[i][j] = 1e9 as i32;
                    } else if self.infected[i][j] > 1 && self.infected[i][j] != 1e9 as i32 {
                        self.infected[i][j] = 1;
                        if i + 1 < self.n && self.infected[i + 1][j] == 0 {
                            self.infected[i + 1][j] = 1;
                        }
                        if i >= 1 && self.infected[i - 1][j] == 0 {
                            self.infected[i - 1][j] = 1;
                        }
                        if j + 1 < self.m && self.infected[i][j + 1] == 0 {
                            self.infected[i][j + 1] = 1;
                        }
                        if j >= 1 && self.infected[i][j - 1] == 0 {
                            self.infected[i][j - 1] = 1;
                        }
                    }
                }
            }
        }
        self.ans
    }
}

struct Solution;

impl Solution {
    pub fn contain_virus(is_infected: Vec<Vec<i32>>) -> i32 {
        let mut my_solution = MySolution::new();
        my_solution.contain_virus(is_infected)
    }
}

#[test]
fn test() {
    let is_infected = vec![
        vec![0, 1, 0, 0, 0, 0, 0, 1],
        vec![0, 1, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(Solution::contain_virus(is_infected), 10);

    let is_infected = vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]];
    assert_eq!(Solution::contain_virus(is_infected), 4);

    let is_infected = vec![
        vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 1, 0, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(Solution::contain_virus(is_infected), 13);
}
