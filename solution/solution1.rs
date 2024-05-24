/// 1.两数之和
///
/// <p>给定一个整数数组 <code>nums</code>&nbsp;和一个整数目标值 <code>target</code>，请你在该数组中找出 <strong>和为目标值 </strong><em><code>target</code></em>&nbsp; 的那&nbsp;<strong>两个</strong>&nbsp;整数，并返回它们的数组下标。</p>
///
/// <p>你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。</p>
///
/// <p>你可以按任意顺序返回答案。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [2,7,11,15], target = 9
/// <strong>输出：</strong>[0,1]
/// <strong>解释：</strong>因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
/// </pre>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [3,2,4], target = 6
/// <strong>输出：</strong>[1,2]
/// </pre>
///
/// <p><strong class="example">示例 3：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [3,3], target = 6
/// <strong>输出：</strong>[0,1]
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>2 &lt;= nums.length &lt;= 10<sup>4</sup></code></li>
/// 	<li><code>-10<sup>9</sup> &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
/// 	<li><code>-10<sup>9</sup> &lt;= target &lt;= 10<sup>9</sup></code></li>
/// 	<li><strong>只会存在一个有效答案</strong></li>
/// </ul>
///
/// <p>&nbsp;</p>
///
/// <p><strong>进阶：</strong>你可以想出一个时间复杂度小于 <code>O(n<sup>2</sup>)</code> 的算法吗？</p>
/// <a href="https://leetcode.cn/problems/two-sum/">两数之和</a>
pub struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new();
        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = map.get(&complement) {
                return vec![j as i32, i as i32];
            }
            map.insert(num, i);
        }

        vec![]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
