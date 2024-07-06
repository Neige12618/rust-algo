/// 3101.交替子数组计数
/// count-alternating-subarrays
/// <p>给你一个<span data-keyword="binary-array">二进制数组 </span><code>nums</code> 。</p>
///
/// <p>如果一个<span data-keyword="subarray-nonempty">子数组</span>中 <strong>不存在 </strong>两个 <strong>相邻 </strong>元素的值 <strong>相同</strong> 的情况，我们称这样的子数组为 <strong>交替子数组 </strong>。</p>
///
/// <p>返回数组 <code>nums</code> 中交替子数组的数量。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong> <span class="example-io">nums = [0,1,1,1]</span></p>
///
/// <p><strong>输出：</strong> <span class="example-io">5</span></p>
///
/// <p><strong>解释：</strong></p>
/// <!-- 解释示例1的交替子数组 -->
///
/// <p>以下子数组是交替子数组：<code>[0]</code> 、<code>[1]</code> 、<code>[1]</code> 、<code>[1]</code> 以及 <code>[0,1]</code> 。</p>
/// </div>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong> <span class="example-io">nums = [1,0,1,0]</span></p>
///
/// <p><strong>输出：</strong> <span class="example-io">10</span></p>
///
/// <p><strong>解释：</strong></p>
/// <!-- 解释示例2的交替子数组 -->
///
/// <p>数组的每个子数组都是交替子数组。可以统计在内的子数组共有 10 个。</p>
/// </div>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
/// 	<li><code>nums[i]</code> 不是 <code>0</code> 就是 <code>1</code> 。</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/count-alternating-subarrays/">交替子数组计数</a>
pub struct Solution;

// 遍历
impl Solution {
    pub fn count_alternating_subarrays(nums: Vec<i32>) -> i64 {
        nums.chunk_by(|a, b| a != b)
            .map(|v| (v.len() * (v.len() + 1) / 2) as i64)
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::count_alternating_subarrays(vec![0, 1, 1, 1]), 5)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::count_alternating_subarrays(vec![1, 0, 1, 0]), 10)
    }
}
