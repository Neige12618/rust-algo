/// 2786.访问数组中的位置使分数最大
/// visit-array-positions-to-maximize-score
/// <p>给你一个下标从 <strong>0</strong>&nbsp;开始的整数数组&nbsp;<code>nums</code>&nbsp;和一个正整数&nbsp;<code>x</code>&nbsp;。</p>
///
/// <p>你 <strong>一开始</strong>&nbsp;在数组的位置 <code>0</code>&nbsp;处，你可以按照下述规则访问数组中的其他位置：</p>
///
/// <ul>
/// 	<li>如果你当前在位置&nbsp;<code>i</code>&nbsp;，那么你可以移动到满足&nbsp;<code>i &lt; j</code>&nbsp;的&nbsp;<strong>任意</strong>&nbsp;位置&nbsp;<code>j</code>&nbsp;。</li>
/// 	<li>对于你访问的位置 <code>i</code>&nbsp;，你可以获得分数&nbsp;<code>nums[i]</code>&nbsp;。</li>
/// 	<li>如果你从位置 <code>i</code>&nbsp;移动到位置 <code>j</code>&nbsp;且&nbsp;<code>nums[i]</code> 和&nbsp;<code>nums[j]</code>&nbsp;的 <strong>奇偶性</strong>&nbsp;不同，那么你将失去分数&nbsp;<code>x</code>&nbsp;。</li>
/// </ul>
///
/// <p>请你返回你能得到的 <strong>最大</strong>&nbsp;得分之和。</p>
///
/// <p><strong>注意</strong>&nbsp;，你一开始的分数为&nbsp;<code>nums[0]</code>&nbsp;。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre><b>输入：</b>nums = [2,3,6,1,9,2], x = 5
/// <b>输出：</b>13
/// <b>解释：</b>我们可以按顺序访问数组中的位置：0 -&gt; 2 -&gt; 3 -&gt; 4 。
/// 对应位置的值为 2 ，6 ，1 和 9 。因为 6 和 1 的奇偶性不同，所以下标从 2 -&gt; 3 让你失去 x = 5 分。
/// 总得分为：2 + 6 + 1 + 9 - 5 = 13 。
/// </pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre><b>输入：</b>nums = [2,4,6,8], x = 3
/// <b>输出：</b>20
/// <b>解释：</b>数组中的所有元素奇偶性都一样，所以我们可以将每个元素都访问一次，而且不会失去任何分数。
/// 总得分为：2 + 4 + 6 + 8 = 20 。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>2 &lt;= nums.length &lt;= 10<sup>5</sup></code></li>
/// 	<li><code>1 &lt;= nums[i], x &lt;= 10<sup>6</sup></code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/visit-array-positions-to-maximize-score/">访问数组中的位置使分数最大</a>
pub struct Solution;

// 动态规划
impl Solution {
    pub fn max_score(nums: Vec<i32>, x: i32) -> i64 {
        let x = x as i64;
        // 记录当前的上一个情况是奇数和偶数的最大值
        let mut dp = vec![0, 0];
        for &i in nums.iter().rev() {
            let v = i as i64;
            let r = (v % 2) as usize;
            dp[r] = dp[r].max(dp[r ^ 1] - x) + v;
        }
        dp[(nums[0] % 2) as usize]
    }

    #[allow(unused)]
    // memo[i][j] 表示从第i个元素开始选取一个子序列，第一个数的奇偶性为j的最大值
    fn dfs(nums: &Vec<i32>, i: usize, j: usize, x: i64, memo: &mut Vec<[i64; 2]>) -> i64 {
        if i == nums.len() {
            return 0;
        }

        if memo[i][j] != -1 {
            return memo[i][j];
        }
        // 寻找第一个奇偶性为j的值
        if nums[i] % 2 != j as i32 {
            memo[i][j] = Solution::dfs(nums, i + 1, j, x, memo);
            return memo[i][j];
        }
        // 找到后，从当前位置开始，寻找下一个奇偶性为j和j^1的值，取最大
        memo[i][j] = Self::dfs(nums, i + 1, j, x, memo)
            .max(Self::dfs(nums, i + 1, j ^ 1, x, memo) - x)
            + nums[i] as i64;
        memo[i][j]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::max_score(vec![2, 3, 6, 1, 9, 2], 5), 13)
    }
}
