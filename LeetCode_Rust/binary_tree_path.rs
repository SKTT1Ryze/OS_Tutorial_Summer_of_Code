/*
 * LeetCode for Rust: Binary Tree Path
 * 2020/7/6
 * hustccc
 * Manjaro
 */
 
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
// 使用DFS来递归实现

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
        let mut result_paths = Vec::new();
        Solution::DFS(root, &mut result_paths, "");
        result_paths
    }

    fn DFS(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<String>, prev_path: &str) {
        if let Some(root) = root {
            let root = root.borrow();
            let current_path = match prev_path.len() {
                0 => root.val.to_string(),
                _ => prev_path.clone().to_owned() + "->" + &root.val.to_string(),
            };
            match root.left.is_none() && root.right.is_none() {
                true => {result.push(current_path);},
                false => {
                    Solution::DFS(root.left.clone(),result,&current_path);
                    Solution::DFS(root.right.clone(),result,&current_path);
                },
            }

        }
    }
}

//C++
class Solution {
    public:
        vector<string> ResultPaths;
        void DFS(TreeNode* root, string result)
        {
            result += "->" + to_string(root->val);
            if(root->left == NULL && root->right == NULL)
                ResultPaths.push_back(answer);
            else
            {
                if(root->left)
                    DFS(root->left, result);
                if(root->right)
                    DFS(root->right, result);
            }
        }
        vector<string> binaryTreePaths(TreeNode* root) {
            if(root != NULL)
            {
                DFS(root, "");
                for(int i = 0; i < ResultPaths.size(); i++)
                ResultPaths[i].erase(ResultPaths[i].begin(), ResultPaths[i].begin() + 2);
            }
            return ResultPaths;
        }
    };