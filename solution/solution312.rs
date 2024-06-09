/// 312.戳气球
/// burst-balloons
/// <p>有 <code>n</code> 个气球，编号为<code>0</code> 到 <code>n - 1</code>，每个气球上都标有一个数字，这些数字存在数组&nbsp;<code>nums</code>&nbsp;中。</p>
///
/// <p>现在要求你戳破所有的气球。戳破第 <code>i</code> 个气球，你可以获得&nbsp;<code>nums[i - 1] * nums[i] * nums[i + 1]</code> 枚硬币。&nbsp;这里的 <code>i - 1</code> 和 <code>i + 1</code> 代表和&nbsp;<code>i</code>&nbsp;相邻的两个气球的序号。如果 <code>i - 1</code>或 <code>i + 1</code> 超出了数组的边界，那么就当它是一个数字为 <code>1</code> 的气球。</p>
///
/// <p>求所能获得硬币的最大数量。</p>
///
/// <p>&nbsp;</p>
/// <strong>示例 1：</strong>
///
/// <pre>
/// <strong>输入：</strong>nums = [3,1,5,8]
/// <strong>输出：</strong>167
/// <strong>解释：</strong>
/// nums = [3,1,5,8] --&gt; [3,5,8] --&gt; [3,8] --&gt; [8] --&gt; []
/// coins =  3*1*5    +   3*5*8   +  1*3*8  + 1*8*1 = 167</pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [1,5]
/// <strong>输出：</strong>10
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>n == nums.length</code></li>
/// 	<li><code>1 &lt;= n &lt;= 300</code></li>
/// 	<li><code>0 &lt;= nums[i] &lt;= 100</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/burst-balloons/">戳气球</a>
pub struct Solution;

// 区间动态规划
impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut coins = vec![1];
        coins.extend(nums);
        coins.push(1);
        let mut dp = vec![vec![0; n + 2]; n + 2];

        for i in (0..n).rev() {
            for j in i + 2..=n + 1 {
                for k in i + 1..j {
                    dp[i][j] = dp[i][j].max(coins[i] * coins[k] * coins[j] + dp[i][k] + dp[k][j]);
                }
            }
        }

        dp[0][n + 1]
    }

    #[allow(unused)]
    fn dfs(nums: &Vec<i32>, left: usize, right: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if left + 1 == right {
            0
        } else if left + 2 == right {
            nums[left] * nums[left + 1] * nums[right]
        } else if memo[left][right] != -1 {
            memo[left][right]
        } else {
            let mut ans = 0;
            for i in left + 1..right {
                ans = ans.max(
                    nums[left] * nums[i] * nums[right]
                        + Self::dfs(nums, left, i, memo)
                        + Self::dfs(nums, i, right, memo),
                );
            }
            memo[left][right] = ans;
            ans
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::max_coins(vec![3, 1, 5, 8]), 167);
    }
}
