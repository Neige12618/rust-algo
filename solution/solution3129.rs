/// 3129.找出所有稳定的二进制数组 I
/// find-all-possible-stable-binary-arrays-i
/// <p>给你 3 个正整数&nbsp;<code>zero</code>&nbsp;，<code>one</code>&nbsp;和&nbsp;<code>limit</code>&nbsp;。</p>
///
/// <p>一个 <span data-keyword="binary-array">二进制数组</span> <code>arr</code> 如果满足以下条件，那么我们称它是 <strong>稳定的</strong> ：</p>
///
/// <ul>
/// 	<li>0 在&nbsp;<code>arr</code>&nbsp;中出现次数 <strong>恰好</strong>&nbsp;为<strong>&nbsp;</strong><code>zero</code>&nbsp;。</li>
/// 	<li>1 在&nbsp;<code>arr</code>&nbsp;中出现次数 <strong>恰好</strong>&nbsp;为&nbsp;<code>one</code>&nbsp;。</li>
/// 	<li><code>arr</code> 中每个长度超过 <code>limit</code>&nbsp;的 <span data-keyword="subarray-nonempty">子数组</span> 都 <strong>同时</strong> 包含 0 和 1 。</li>
/// </ul>
///
/// <p>请你返回 <strong>稳定</strong>&nbsp;二进制数组的 <em>总</em> 数目。</p>
///
/// <p>由于答案可能很大，将它对&nbsp;<code>10<sup>9</sup> + 7</code>&nbsp;<b>取余</b>&nbsp;后返回。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <div class="example-block">
/// <p><span class="example-io"><b>输入：</b>zero = 1, one = 1, limit = 2</span></p>
///
/// <p><span class="example-io"><b>输出：</b>2</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p>两个稳定的二进制数组为&nbsp;<code>[1,0]</code> 和&nbsp;<code>[0,1]</code>&nbsp;，两个数组都有一个 0 和一个 1 ，且没有子数组长度大于 2 。</p>
/// </div>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong><span class="example-io">zero = 1, one = 2, limit = 1</span></p>
///
/// <p><span class="example-io"><b>输出：</b>1</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p>唯一稳定的二进制数组是&nbsp;<code>[1,0,1]</code>&nbsp;。</p>
///
/// <p>二进制数组&nbsp;<code>[1,1,0]</code> 和&nbsp;<code>[0,1,1]</code>&nbsp;都有长度为 2 且元素全都相同的子数组，所以它们不稳定。</p>
/// </div>
///
/// <p><strong class="example">示例 3：</strong></p>
///
/// <div class="example-block">
/// <p><span class="example-io"><b>输入：</b>zero = 3, one = 3, limit = 2</span></p>
///
/// <p><span class="example-io"><b>输出：</b>14</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p>所有稳定的二进制数组包括&nbsp;<code>[0,0,1,0,1,1]</code>&nbsp;，<code>[0,0,1,1,0,1]</code>&nbsp;，<code>[0,1,0,0,1,1]</code>&nbsp;，<code>[0,1,0,1,0,1]</code>&nbsp;，<code>[0,1,0,1,1,0]</code>&nbsp;，<code>[0,1,1,0,0,1]</code>&nbsp;，<code>[0,1,1,0,1,0]</code>&nbsp;，<code>[1,0,0,1,0,1]</code>&nbsp;，<code>[1,0,0,1,1,0]</code>&nbsp;，<code>[1,0,1,0,0,1]</code>&nbsp;，<code>[1,0,1,0,1,0]</code>&nbsp;，<code>[1,0,1,1,0,0]</code>&nbsp;，<code>[1,1,0,0,1,0]</code>&nbsp;和&nbsp;<code>[1,1,0,1,0,0]</code>&nbsp;。</p>
/// </div>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= zero, one, limit &lt;= 200</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/find-all-possible-stable-binary-arrays-i/">找出所有稳定的二进制数组 I</a>
pub struct Solution;

const MOD: i32 = 1_000_000_007;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let (zero, one, limit) = (zero as usize, one as usize, limit as usize);
        let mut dp = vec![vec![[0; 2]; one + 1]; zero + 1];
        (0..=zero.min(limit)).for_each(|i| dp[i][0][0] = 1);
        (0..=one.min(limit)).for_each(|j| dp[0][j][1] = 1);
        for i in 1..=zero {
            for j in 1..=one {
                dp[i][j][0] = ((dp[i - 1][j][0] + dp[i - 1][j][1]
                    - if i > limit {
                        dp[i - limit - 1][j][1]
                    } else {
                        0
                    })
                    % MOD
                    + MOD)
                    % MOD;

                dp[i][j][1] = ((dp[i][j - 1][1] + dp[i][j - 1][0]
                    - if j > limit {
                        dp[i][j - limit - 1][0]
                    } else {
                        0
                    })
                    % MOD
                    + MOD)
                    % MOD;
            }
        }
        (dp[zero][one][0] + dp[zero][one][1]) % MOD
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::number_of_stable_arrays(1, 1, 2), 2)
    }

    #[test]
    fn test_2() {
        assert_eq!(Solution::number_of_stable_arrays(1, 2, 1), 1)
    }
}
