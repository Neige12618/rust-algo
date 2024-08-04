/// 3098.求出所有子序列的能量和
/// find-the-sum-of-subsequence-powers
/// <p>给你一个长度为 <code>n</code>&nbsp;的整数数组&nbsp;<code>nums</code>&nbsp;和一个 <strong>正</strong>&nbsp;整数&nbsp;<code>k</code>&nbsp;。</p>
///
/// <p>一个 <span data-keyword="subsequence-array">子序列</span> 的 <strong>能量</strong>&nbsp;定义为子序列中&nbsp;<strong>任意</strong>&nbsp;两个元素的差值绝对值的 <strong>最小值</strong>&nbsp;。</p>
///
/// <p>请你返回 <code>nums</code>&nbsp;中长度 <strong>等于</strong>&nbsp;<code>k</code>&nbsp;的 <strong>所有</strong>&nbsp;子序列的 <strong>能量和</strong>&nbsp;。</p>
///
/// <p>由于答案可能会很大，将答案对&nbsp;<code>10<sup>9 </sup>+ 7</code>&nbsp;<strong>取余</strong>&nbsp;后返回。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <div class="example-block">
/// <p><span class="example-io"><b>输入：</b>nums = [1,2,3,4], k = 3</span></p>
///
/// <p><span class="example-io"><b>输出：</b>4</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p><code>nums</code>&nbsp;中总共有 4 个长度为 3 的子序列：<code>[1,2,3]</code>&nbsp;，<code>[1,3,4]</code>&nbsp;，<code>[1,2,4]</code>&nbsp;和&nbsp;<code>[2,3,4]</code>&nbsp;。能量和为 <code>|2 - 3| + |3 - 4| + |2 - 1| + |3 - 4| = 4</code>&nbsp;。</p>
/// </div>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <div class="example-block">
/// <p><span class="example-io"><b>输入：</b>nums = [2,2], k = 2</span></p>
///
/// <p><span class="example-io"><b>输出：</b>0</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p><code>nums</code>&nbsp;中唯一一个长度为 2 的子序列是&nbsp;<code>[2,2]</code>&nbsp;。能量和为&nbsp;<code>|2 - 2| = 0</code>&nbsp;。</p>
/// </div>
///
/// <p><strong class="example">示例 3：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong><span class="example-io">nums = [4,3,-1], k = 2</span></p>
///
/// <p><span class="example-io"><b>输出：</b>10</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p><code>nums</code>&nbsp;总共有 3 个长度为 2 的子序列：<code>[4,3]</code>&nbsp;，<code>[4,-1]</code>&nbsp;和&nbsp;<code>[3,-1]</code>&nbsp;。能量和为&nbsp;<code>|4 - 3| + |4 - (-1)| + |3 - (-1)| = 10</code>&nbsp;。</p>
/// </div>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>2 &lt;= n == nums.length &lt;= 50</code></li>
/// 	<li><code>-10<sup>8</sup> &lt;= nums[i] &lt;= 10<sup>8</sup> </code></li>
/// 	<li><code>2 &lt;= k &lt;= n</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/find-the-sum-of-subsequence-powers/">求出所有子序列的能量和</a>
pub struct Solution;

use std::collections::HashMap;
static MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn sum_of_powers(mut nums: Vec<i32>, k: i32) -> i32 {
        fn dfs(
            last_idx: usize,
            k: usize,
            min_diff: i32,
            dp: &mut HashMap<i64, i32>,
            nums: &Vec<i32>,
        ) -> i32 {
            if k == 0 {
                return min_diff;
            }

            let key = (((min_diff as i64) << 12) + ((last_idx as i64) << 6) + (k as i64)) as i64;
            if let Some(&value) = dp.get(&key) {
                return value;
            }

            let ans = (last_idx + 1..=nums.len() - k).fold(0, |ans, i| {
                (ans + dfs(i, k - 1, min_diff.min(nums[i] - nums[last_idx]), dp, nums)) % MOD
            });
            dp.insert(key, ans);
            ans
        }

        nums.sort_unstable();
        let (n, mut dp) = (nums.len(), HashMap::new());
        (0..n - k as usize + 1).fold(0, |ans, i| {
            (ans + dfs(i, k as usize - 1, nums[n - 1] - nums[0], &mut dp, &nums)) % MOD
        })
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::sum_of_powers(vec![1, 2, 3, 4], 3), 4)
    }
}
