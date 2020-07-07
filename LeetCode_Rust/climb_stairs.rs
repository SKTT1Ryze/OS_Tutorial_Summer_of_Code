/*
 * LeetCode for Rust: Binary Tree Path
 * 2020/7/6
 * hustccc
 * Manjaro
 */

// Source : https://oj.leetcode.com/problems/climbing-stairs/
// Author : hustccc
// Date   : 2020-06-07

/********************************************************************************** 
* 
* You are climbing a stair case. It takes n steps to reach to the top.
* 
* Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
* 
*               
**********************************************************************************/

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n <= 3 {
            true => n,
            false => {
                let mut vec = vec![2,3];
                let v = &mut vec;
                for i in 0..n-4 {
                    let t = v[0] + v[1];
                    v[0] = v[1];
                    v[1] = t;
                }
                v[1]
            }
        }
    }
}

//C++
class Solution {
    public:
        int climbStairs(int n) {
           if (n<=3) return n;
           int x[2]={2,3};
           for(int i=0; i<=n-4; i++){
               int t = x[0] + x[1];
               x[0] = x[1];
               x[1] = t;
           }
           return x[1];
        }
    };
