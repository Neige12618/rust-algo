/// 2903.找出满足差值条件的下标 I
///
/// <p>给你一个下标从 <strong>0</strong> 开始、长度为 <code>n</code> 的整数数组 <code>nums</code> ，以及整数 <code>indexDifference</code> 和整数 <code>valueDifference</code> 。</p>
///
/// <p>你的任务是从范围 <code>[0, n - 1]</code> 内找出&nbsp; <strong>2</strong> 个满足下述所有条件的下标 <code>i</code> 和 <code>j</code> ：</p>
///
/// <ul>
/// 	<li><code>abs(i - j) &gt;= indexDifference</code> 且</li>
/// 	<li><code>abs(nums[i] - nums[j]) &gt;= valueDifference</code></li>
/// </ul>
///
/// <p>返回整数数组 <code>answer</code>。如果存在满足题目要求的两个下标，则 <code>answer = [i, j]</code> ；否则，<code>answer = [-1, -1]</code> 。如果存在多组可供选择的下标对，只需要返回其中任意一组即可。</p>
///
/// <p><strong>注意：</strong><code>i</code> 和 <code>j</code> 可能 <strong>相等</strong> 。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [5,1,4,1], indexDifference = 2, valueDifference = 4
/// <strong>输出：</strong>[0,3]
/// <strong>解释：</strong>在示例中，可以选择 i = 0 和 j = 3 。
/// abs(0 - 3) &gt;= 2 且 abs(nums[0] - nums[3]) &gt;= 4 。
/// 因此，[0,3] 是一个符合题目要求的答案。
/// [3,0] 也是符合题目要求的答案。
/// </pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [2,1], indexDifference = 0, valueDifference = 0
/// <strong>输出：</strong>[0,0]
/// <strong>解释：</strong>
/// 在示例中，可以选择 i = 0 和 j = 0 。
/// abs(0 - 0) &gt;= 0 且 abs(nums[0] - nums[0]) &gt;= 0 。
/// 因此，[0,0] 是一个符合题目要求的答案。
/// [0,1]、[1,0] 和 [1,1] 也是符合题目要求的答案。
/// </pre>
///
/// <p><strong>示例 3：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [1,2,3], indexDifference = 2, valueDifference = 4
/// <strong>输出：</strong>[-1,-1]
/// <strong>解释：</strong>在示例中，可以证明无法找出 2 个满足所有条件的下标。
/// 因此，返回 [-1,-1] 。</pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= n == nums.length &lt;= 100</code></li>
/// 	<li><code>0 &lt;= nums[i] &lt;= 50</code></li>
/// 	<li><code>0 &lt;= indexDifference &lt;= 100</code></li>
/// 	<li><code>0 &lt;= valueDifference &lt;= 50</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/find-indices-with-index-and-value-difference-i/">找出满足差值条件的下标 I</a>
pub struct Solution;

/// 暴力搜索
impl Solution {
    pub fn find_indices(nums: Vec<i32>, index_difference: i32, value_difference: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if (i as i32 - j as i32).abs() >= index_difference
                    && (nums[i] - nums[j]).abs() >= value_difference
                {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![-1, -1]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::find_indices(vec![2, 1], 0, 0), vec![0, 0]);
    }
}
