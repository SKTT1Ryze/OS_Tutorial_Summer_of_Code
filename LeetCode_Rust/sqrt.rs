/*
 * LeetCode for Rust: Binary Tree Path
 * 2020/7/6
 * hustccc
 * Manjaro
 */

// Source : https://leetcode.com/problems/sqrtx/
// Author : hustccc
// Date   : 2020-07-06
/*
Implement int sqrt(int x).

Compute and return the square root of x, where x is guaranteed to be a non-negative integer.

Since the return type is an integer, the decimal digits are truncated and only the integer part of the result is returned.

Example 1:

Input: 4
Output: 2
Example 2:

Input: 8
Output: 2
Explanation: The square root of 8 is 2.82842..., and since the decimal part is truncated, 2 is returned.
*/            


impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let mut m = 0u64;
        let mut n = (x as u64)/2 + 1;
        while m < n {
            let temp = (m + n + 1) / 2 ;
            match temp * temp > x as u64 {
                true => {
                    n = temp - 1;
                },
                false => {
                    m = temp;
                },
            }
        }
        m as i32
    }
}