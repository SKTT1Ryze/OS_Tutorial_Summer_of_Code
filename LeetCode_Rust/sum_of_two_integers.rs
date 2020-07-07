/*
 * LeetCode for Rust: Sum Of Two Integers
 * 2020/7/6
 * hustccc
 * Manjaro
 */
// Source : https://leetcode.com/problems/sum-of-two-integers/description/
// Author : hustccc
// Date   : 2020-07-06

/*************************************************************************************** 
 *
 * Calculate the sum of two integers a and b, but you are not allowed to use the 
 * operator + and -.
 * 
 * Example:
 * Given a = 1 and b = 2, return 3.
 * 
 * 
 * Credits:Special thanks to @fujiaozhu for adding this problem and creating all test 
 * cases.
 ***************************************************************************************/

//LeetCode的入门经典
impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut x = a;
        let mut y = b;
        while y != 0 {
            let temp = x & y;
            x = x ^ y;
            y = temp << 1;
        }
        x
    }
}

//C++
class Solution {
    public:
        int getSum(int x, int y) {
            while (y != 0) {
                int temp = x & y;  
                x = x ^ y; 
                y = temp << 1;
            }
            return x;
        }
    };
    