/// 600.不含连续1的非负整数
/// non-negative-integers-without-consecutive-ones
/// <p>给定一个正整数 <code>n</code> ，请你统计在&nbsp;<code>[0, n]</code> 范围的非负整数中，有多少个整数的二进制表示中不存在 <strong>连续的 1 </strong>。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1:</strong></p>
///
/// <pre>
/// <strong>输入:</strong> n = 5
/// <strong>输出:</strong> 5
/// <strong>解释:</strong>
/// 下面列出范围在 [0, 5] 的非负整数与其对应的二进制表示：
/// 0 : 0
/// 1 : 1
/// 2 : 10
/// 3 : 11
/// 4 : 100
/// 5 : 101
/// 其中，只有整数 3 违反规则（有两个连续的 1 ），其他 5 个满足规则。</pre>
///
/// <p><strong>示例 2:</strong></p>
///
/// <pre>
/// <strong>输入:</strong> n = 1
/// <strong>输出:</strong> 2
/// </pre>
///
/// <p><strong>示例 3:</strong></p>
///
/// <pre>
/// <strong>输入:</strong> n = 2
/// <strong>输出:</strong> 3
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示:</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= n &lt;= 10<sup>9</sup></code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/non-negative-integers-without-consecutive-ones/">不含连续1的非负整数</a>
pub struct Solution;

impl Solution {
    pub fn find_integers(n: i32) -> i32 {
        let m = i32::BITS as i32 - n.leading_zeros() as i32;

        let mut memo = vec![[-1, -1]; (m + 1) as usize];

        Self::dfs(m - 1, 0, true, n, &mut memo)
    }

    fn dfs(i: i32, pre: usize, is_limit: bool, n: i32, memo: &mut Vec<[i32; 2]>) -> i32 {
        if i < 0 {
            return 1;
        }

        if !is_limit && memo[i as usize][pre] != -1 {
            return memo[i as usize][pre];
        }

        let up = if is_limit { n >> i & 1 } else { 1 };
        let mut res = Self::dfs(i - 1, 0, is_limit && up == 0, n, memo);
        if pre == 0 && up == 1 {
            res += Self::dfs(i - 1, 1, is_limit, n, memo);
        }

        if !is_limit {
            memo[i as usize][pre] = res;
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::find_integers(5), 5)
    }
}
