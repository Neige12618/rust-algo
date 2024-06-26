/// 2741.特别的排列
/// special-permutations
/// <p>给你一个下标从 <strong>0</strong>&nbsp;开始的整数数组&nbsp;<code>nums</code>&nbsp;，它包含 <code>n</code>&nbsp;个 <strong>互不相同</strong>&nbsp;的正整数。如果&nbsp;<code>nums</code>&nbsp;的一个排列满足以下条件，我们称它是一个特别的排列：</p>
///
/// <ul>
/// 	<li>对于&nbsp;<code>0 &lt;= i &lt; n - 1</code>&nbsp;的下标 <code>i</code>&nbsp;，要么&nbsp;<code>nums[i] % nums[i+1] == 0</code>&nbsp;，要么&nbsp;<code>nums[i+1] % nums[i] == 0</code>&nbsp;。</li>
/// </ul>
///
/// <p>请你返回特别排列的总数目，由于答案可能很大，请将它对<strong>&nbsp;</strong><code>10<sup>9&nbsp;</sup>+ 7</code>&nbsp;<strong>取余</strong>&nbsp;后返回。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre><strong>输入：</strong>nums = [2,3,6]
/// <b>输出：</b>2
/// <b>解释：</b>[3,6,2] 和 [2,6,3] 是 nums 两个特别的排列。
/// </pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre><b>输入：</b>nums = [1,4,3]
/// <b>输出：</b>2
/// <b>解释：</b>[3,1,4] 和 [4,1,3] 是 nums 两个特别的排列。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>2 &lt;= nums.length &lt;= 14</code></li>
/// 	<li><code>1 &lt;= nums[i] &lt;= 10<sup>9</sup></code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/special-permutations/">特别的排列</a>
pub struct Solution;

// 状态压缩 动态规划
impl Solution {
    pub fn special_perm(nums: Vec<i32>) -> i32 {
        fn dfs(s: usize, i: usize, nums: &Vec<i32>, f: &mut Vec<Vec<i64>>) -> i64 {
            if f[s][i] != -1 {
                return f[s][i];
            }
            if s == 0 {
                return 1;
            }
            f[s][i] = 0;
            let pre = nums[i];
            for (j, &x) in nums.iter().enumerate() {
                if s >> j & 1 == 1 && (pre % x == 0 || x % pre == 0) {
                    f[s][i] += dfs(s ^ (1 << j), j, &nums, f);
                }
            }
            f[s][i]
        }

        let n = nums.len();
        let u = (1 << n) - 1;
        let mut f = vec![vec![-1; n]; 1 << n];
        ((0..n)
            .map(|i| dfs(u ^ (1 << i), i, &nums, &mut f))
            .sum::<i64>()
            % 1_000_000_007) as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::special_perm(vec![2, 3, 6]), 2);
    }
}
