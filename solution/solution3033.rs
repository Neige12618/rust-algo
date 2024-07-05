/// 3033.修改矩阵
/// modify-the-matrix
/// <p>给你一个下标从 <strong>0</strong> 开始、大小为 <code>m x n</code> 的整数矩阵 <code>matrix</code> ，新建一个下标从 <strong>0</strong> 开始、名为 <code>answer</code> 的矩阵。使 <code>answer</code> 与 <code>matrix</code> 相等，接着将其中每个值为 <code>-1</code> 的元素替换为所在列的 <strong>最大</strong> 元素。</p>
///
/// <p>返回矩阵 <code>answer</code> 。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
/// <img alt="" src="https://assets.leetcode.com/uploads/2023/12/24/matrix1.png" style="width: 491px; height: 161px;" />
/// <pre>
/// <strong>输入：</strong>matrix = [[1,2,-1],[4,-1,6],[7,8,9]]
/// <strong>输出：</strong>[[1,2,9],[4,8,6],[7,8,9]]
/// <strong>解释：</strong>上图显示了发生替换的元素（蓝色区域）。
/// - 将单元格 [1][1] 中的值替换为列 1 中的最大值 8 。
/// - 将单元格 [0][2] 中的值替换为列 2 中的最大值 9 。
/// </pre>
///
/// <p><strong class="example">示例 2：</strong></p>
/// <img alt="" src="https://assets.leetcode.com/uploads/2023/12/24/matrix2.png" style="width: 411px; height: 111px;" />
/// <pre>
/// <strong>输入：</strong>matrix = [[3,-1],[5,2]]
/// <strong>输出：</strong>[[3,2],[5,2]]
/// <strong>解释：</strong>上图显示了发生替换的元素（蓝色区域）。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>m == matrix.length</code></li>
/// 	<li><code>n == matrix[i].length</code></li>
/// 	<li><code>2 &lt;= m, n &lt;= 50</code></li>
/// 	<li><code>-1 &lt;= matrix[i][j] &lt;= 100</code></li>
/// 	<li>测试用例中生成的输入满足每列至少包含一个非负整数。</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/modify-the-matrix/">修改矩阵</a>
pub struct Solution;

// 遍历
impl Solution {
    pub fn modified_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; matrix[0].len()]; matrix.len()];
        let (m, n) = (matrix.len(), matrix[0].len());
        for j in 0..n {
            let mut max = -1;
            for i in 0..m {
                max = max.max(matrix[i][j]);
            }
            for i in 0..m {
                if matrix[i][j] == -1 {
                    ans[i][j] = max;
                } else {
                    ans[i][j] = matrix[i][j];
                }
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
            Solution::modified_matrix(vec![vec![3, -1], vec![5, 2]]),
            vec![vec![3, 2], vec![5, 2]]
        )
    }
}
