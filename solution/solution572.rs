/// 572.另一棵树的子树
/// subtree-of-another-tree
/// <div class="original__bRMd">
/// <div>
/// <p>给你两棵二叉树 <code>root</code> 和 <code>subRoot</code> 。检验 <code>root</code> 中是否包含和 <code>subRoot</code> 具有相同结构和节点值的子树。如果存在，返回 <code>true</code> ；否则，返回 <code>false</code> 。</p>
///
/// <p>二叉树 <code>tree</code> 的一棵子树包括 <code>tree</code> 的某个节点和这个节点的所有后代节点。<code>tree</code> 也可以看做它自身的一棵子树。</p>
///
/// <p> </p>
///
/// <p><strong>示例 1：</strong></p>
/// <img alt="" src="https://assets.leetcode.com/uploads/2021/04/28/subtree1-tree.jpg" style="width: 532px; height: 400px;" />
/// <pre>
/// <strong>输入：</strong>root = [3,4,5,1,2], subRoot = [4,1,2]
/// <strong>输出：</strong>true
/// </pre>
///
/// <p><strong>示例 2：</strong></p>
/// <img alt="" src="https://assets.leetcode.com/uploads/2021/04/28/subtree2-tree.jpg" style="width: 502px; height: 458px;" />
/// <pre>
/// <strong>输入：</strong>root = [3,4,5,1,2,null,null,null,null,0], subRoot = [4,1,2]
/// <strong>输出：</strong>false
/// </pre>
///
/// <p> </p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>root</code> 树上的节点数量范围是 <code>[1, 2000]</code></li>
/// 	<li><code>subRoot</code> 树上的节点数量范围是 <code>[1, 1000]</code></li>
/// 	<li><code>-10<sup>4</sup> <= root.val <= 10<sup>4</sup></code></li>
/// 	<li><code>-10<sup>4</sup> <= subRoot.val <= 10<sup>4</sup></code></li>
/// </ul>
/// </div>
/// </div>
/// <a href="https://leetcode.cn/problems/subtree-of-another-tree/">另一棵树的子树</a>
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
pub struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn is_subtree(
        root: Option<Rc<RefCell<TreeNode>>>,
        sub_root: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        if root.is_none() {
            return false;
        }

        Self::same(&root, &sub_root)
            || Self::is_subtree(
                root.as_ref().unwrap().borrow().left.clone(),
                sub_root.clone(),
            )
            || Self::is_subtree(
                root.as_ref().unwrap().borrow().right.clone(),
                sub_root.clone(),
            )
    }

    fn same(p: &Option<Rc<RefCell<TreeNode>>>, q: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();

                p.val == q.val && Self::same(&p.left, &q.left) && Self::same(&p.right, &q.right)
            }
            _ => false,
        }
    }
}

pub fn is_sametree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match (root, sub_root) {
        (None, None) => true,
        (_, None) | (None, _) => false,
        (Some(root), Some(sub_root)) => {
            root.borrow().val == sub_root.borrow().val
                && is_sametree(root.borrow().left.clone(), sub_root.borrow().left.clone())
                && is_sametree(root.borrow().right.clone(), sub_root.borrow().right.clone())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::is_subtree(None, None), false)
    }
}
