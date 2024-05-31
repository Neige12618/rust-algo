/// 2965.找出缺失和重复的数字
/// find-missing-and-repeated-values
/// <p>给你一个下标从<strong> 0 </strong>开始的二维整数矩阵 <code><font face="monospace">grid</font></code>，大小为 <code>n * n</code> ，其中的值在 <code>[1, n<sup>2</sup>]</code> 范围内。除了 <code>a</code> 出现 <strong>两次</strong>，<code>b</code> <strong>缺失</strong> 之外，每个整数都<strong> 恰好出现一次</strong> 。</p>
///
/// <p>任务是找出重复的数字<code>a</code> 和缺失的数字 <code>b</code> 。</p>
///
/// <p>返回一个下标从 0 开始、长度为 <code>2</code> 的整数数组 <code>ans</code> ，其中 <code>ans[0]</code> 等于 <code>a</code> ，<code>ans[1]</code> 等于 <code>b</code> 。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>grid = [[1,3],[2,2]]
/// <strong>输出：</strong>[2,4]
/// <strong>解释：</strong>数字 2 重复，数字 4 缺失，所以答案是 [2,4] 。
/// </pre>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>grid = [[9,1,7],[8,9,2],[3,4,6]]
/// <strong>输出：</strong>[9,5]
/// <strong>解释：</strong>数字 9 重复，数字 5 缺失，所以答案是 [9,5] 。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>2 &lt;= n == grid.length == grid[i].length &lt;= 50</code></li>
/// 	<li><code>1 &lt;= grid[i][j] &lt;= n * n</code></li>
/// 	<li>对于所有满足<code>1 &lt;= x &lt;= n * n</code> 的 <code>x</code> ，恰好存在一个 <code>x</code> 与矩阵中的任何成员都不相等。</li>
/// 	<li>对于所有满足<code>1 &lt;= x &lt;= n * n</code> 的 <code>x</code> ，恰好存在一个 <code>x</code> 与矩阵中的两个成员相等。</li>
/// 	<li>除上述的两个之外，对于所有满足<code>1 &lt;= x &lt;= n * n</code> 的 <code>x</code> ，都恰好存在一对 <code>i, j</code> 满足 <code>0 &lt;= i, j &lt;= n - 1</code> 且 <code>grid[i][j] == x</code> 。</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/find-missing-and-repeated-values/">找出缺失和重复的数字</a>
pub struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut cnt = vec![0; grid.len() * grid.len() + 1];
        for row in grid.iter() {
            for &v in row.iter() {
                cnt[v as usize] += 1;
            }
        }
        let mut ans = vec![0; 2];
        for (i, &v) in cnt.iter().enumerate() {
            if v == 2 {
                ans[0] = i as i32;
            }
            if v == 0 {
                ans[1] = i as i32;
            }
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec![vec![1, 3], vec![2, 2]]),
            vec![2, 4]
        );
        assert_eq!(
            Solution::find_missing_and_repeated_values(vec![
                vec![9, 1, 7],
                vec![8, 9, 2],
                vec![3, 4, 6]
            ]),
            vec![9, 5]
        )
    }
}
