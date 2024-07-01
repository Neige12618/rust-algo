/// 494.目标和
/// target-sum
/// <p>给你一个非负整数数组 <code>nums</code> 和一个整数 <code>target</code> 。</p>
///
/// <p>向数组中的每个整数前添加&nbsp;<code>'+'</code> 或 <code>'-'</code> ，然后串联起所有整数，可以构造一个 <strong>表达式</strong> ：</p>
///
/// <ul>
/// 	<li>例如，<code>nums = [2, 1]</code> ，可以在 <code>2</code> 之前添加 <code>'+'</code> ，在 <code>1</code> 之前添加 <code>'-'</code> ，然后串联起来得到表达式 <code>"+2-1"</code> 。</li>
/// </ul>
///
/// <p>返回可以通过上述方法构造的、运算结果等于 <code>target</code> 的不同 <strong>表达式</strong> 的数目。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [1,1,1,1,1], target = 3
/// <strong>输出：</strong>5
/// <strong>解释：</strong>一共有 5 种方法让最终目标和为 3 。
/// -1 + 1 + 1 + 1 + 1 = 3
/// +1 - 1 + 1 + 1 + 1 = 3
/// +1 + 1 - 1 + 1 + 1 = 3
/// +1 + 1 + 1 - 1 + 1 = 3
/// +1 + 1 + 1 + 1 - 1 = 3
/// </pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>nums = [1], target = 1
/// <strong>输出：</strong>1
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= nums.length &lt;= 20</code></li>
/// 	<li><code>0 &lt;= nums[i] &lt;= 1000</code></li>
/// 	<li><code>0 &lt;= sum(nums[i]) &lt;= 1000</code></li>
/// 	<li><code>-1000 &lt;= target &lt;= 1000</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/target-sum/">目标和</a>
pub struct Solution;

// 动态规划
impl Solution {
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let s = nums.iter().sum::<i32>() - target;
        if s < 0 || s % 2 == 1 {
            return 0;
        }
        let m = (s / 2) as usize;

        let mut f = vec![0; m as usize + 1];
        f[0] = 1;
        for &x in nums.iter() {
            let x = x as usize;
            for c in (x..=m).rev() {
                f[c] += f[c - x]
            }
        }

        return f[m];
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5)
    }
}
