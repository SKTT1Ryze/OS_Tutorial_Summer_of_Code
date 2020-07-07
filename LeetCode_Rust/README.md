# Leet Code for Rust

## Sum Of Two Integers
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

## Sqrt(x)
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

## Climb Stairs
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

## Two City Sched Cost
// Source : https://leetcode.com/problems/two-city-scheduling/
// Author : hustccc
// Date   : 2020-07-06

/***************************************************************************************************** 
 *
 * There are 2N people a company is planning to interview. The cost of flying the i-th person to city 
 * A is costs[i][0], and the cost of flying the i-th person to city B is costs[i][1].
 * 
 * Return the minimum cost to fly every person to a city such that exactly N people arrive in each 
 * city.
 * 
 * Example 1:
 * 
 * Input: [[10,20],[30,200],[400,50],[30,20]]
 * Output: 110
 * Explanation: 
 * The first person goes to city A for a cost of 10.
 * The second person goes to city A for a cost of 30.
 * The third person goes to city B for a cost of 50.
 * The fourth person goes to city B for a cost of 20.
 * 
 * The total minimum cost is 10 + 30 + 50 + 20 = 110 to have half the people interviewing in each city.
 * 
 * Note:
 * 
 * 	1 <= costs.length <= 100
 * 	It is guaranteed that costs.length is even.
 * 	1 <= costs[i][0], costs[i][1] <= 1000
 ******************************************************************************************************/

## Binary Tree Paths
// Source : https://leetcode.com/problems/binary-tree-paths/
// Author : hustccc
// Date   : 2020-07-06

/*************************************************************************************** 
 *
 * Given a binary tree, return all root-to-leaf paths.
 * 
 * For example, given the following binary tree:
 * 
 *    1
 *  /   \
 * 2     3
 *  \
 *   5
 * 
 * All root-to-leaf paths are:
 * ["1->2->5", "1->3"]
 * 
 * Credits:
 * Special thanks to @jianchao.li.fighter for adding this problem and creating all test 
 * cases.
 *               
 ***************************************************************************************/