/*
 * LeetCode for Rust: Two City Scheduling
 * 2020/7/6
 * hustccc
 * Manjaro
 */

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

//思路时将Vector以某种规则进行排序，然后到两个城市花费相差较小的项就排到了中间，
//只需要将左边分到去A城市，右边分到去B城市就行了

impl Solution {
    pub fn two_city_sched_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut cost_vec = costs;
        let len = cost_vec.len();
        cost_vec.sort_unstable_by(|a, b| (b[1] - b[0]).cmp(&(a[1] - a[0])));
        let mut result = 0;
        let mut i = 0;
        while i < len/2 {
            result += cost_vec[i][0] + cost_vec[len-i-1][1];
            i += 1;
        }
        result
    }
}

//C++
class Solution {
    private:
        static int distance(vector<int>& v) {
            return v[1] - v[0];
        }
        static bool cmp(vector<int>& a, vector<int>& b) {
            return distance(lhs) > distance(rhs);
        }
    
    public:
        int twoCitySchedCost(vector<vector<int>>& costs) {
            sort(costs.begin(), costs.end(), cmp);
            int len = costs.size();
            int result = 0;
            for (int i=0; i<len/2; i++) {
                result += (costs[i][0] + costs[len-i-1][1]);
            }
            return result;
        }
    };