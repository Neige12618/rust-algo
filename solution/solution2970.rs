/// 2970.统计移除递增子数组的数目 I
/// count-the-number-of-incremovable-subarrays-i
/// <p>给你一个下标从 <strong>0</strong>&nbsp;开始的 <b>正</b>&nbsp;整数数组&nbsp;<code>nums</code>&nbsp;。</p>
///
/// <p>如果 <code>nums</code>&nbsp;的一个子数组满足：移除这个子数组后剩余元素 <strong>严格递增</strong>&nbsp;，那么我们称这个子数组为 <strong>移除递增</strong>&nbsp;子数组。比方说，<code>[5, 3, 4, 6, 7]</code>&nbsp;中的 <code>[3, 4]</code>&nbsp;是一个移除递增子数组，因为移除该子数组后，<code>[5, 3, 4, 6, 7]</code>&nbsp;变为&nbsp;<code>[5, 6, 7]</code>&nbsp;，是严格递增的。</p>
///
/// <p>请你返回 <code>nums</code>&nbsp;中 <b>移除递增</b>&nbsp;子数组的总数目。</p>
///
/// <p><b>注意</b>&nbsp;，剩余元素为空的数组也视为是递增的。</p>
///
/// <p><strong>子数组</strong> 指的是一个数组中一段连续的元素序列。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <pre>
/// <b>输入：</b>nums = [1,2,3,4]
/// <b>输出：</b>10
/// <b>解释：</b>10 个移除递增子数组分别为：[1], [2], [3], [4], [1,2], [2,3], [3,4], [1,2,3], [2,3,4] 和 [1,2,3,4]。移除任意一个子数组后，剩余元素都是递增的。注意，空数组不是移除递增子数组。
/// </pre>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <pre>
/// <b>输入：</b>nums = [6,5,7,8]
/// <b>输出：</b>7
/// <b>解释：</b>7<strong>&nbsp;</strong>个移除递增子数组分别为：[5], [6], [5,7], [6,5], [5,7,8], [6,5,7] 和 [6,5,7,8] 。
/// nums 中只有这 7 个移除递增子数组。
/// </pre>
///
/// <p><strong class="example">示例 3：</strong></p>
///
/// <pre>
/// <b>输入：</b>nums = [8,7,6,6]
/// <b>输出：</b>3
/// <b>解释：</b>3 个移除递增子数组分别为：[8,7,6], [7,6,6] 和 [8,7,6,6] 。注意 [8,7] 不是移除递增子数组因为移除 [8,7] 后 nums 变为 [6,6] ，它不是严格递增的。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= nums.length &lt;= 50</code></li>
/// 	<li><code>1 &lt;= nums[i] &lt;= 50</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/count-the-number-of-incremovable-subarrays-i/">统计移除递增子数组的数目 I</a>
pub struct Solution;

// 双指针
impl Solution {
    pub fn incremovable_subarray_count(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut res = 0;
        let mut l = 1;
        while l < n && nums[l - 1] < nums[l] {
            l += 1;
        }
        res += l as i32 + if l < n { 1 } else { 0 };
        for r in (0..n - 1).rev() {
            while l > 0 && nums[l - 1] >= nums[r + 1] {
                l -= 1;
            }
            res += l as i32 + if l <= r { 1 } else { 0 };
            if nums[r] >= nums[r + 1] {
                break;
            }
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::incremovable_subarray_count(vec![1, 2, 3, 4]), 10)
    }
}
