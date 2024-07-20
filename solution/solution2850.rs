/// 2850.将石头分散到网格图的最少移动次数
/// minimum-moves-to-spread-stones-over-grid
/// <p>给你一个大小为 <code>3 * 3</code>&nbsp;，下标从 <strong>0</strong>&nbsp;开始的二维整数矩阵&nbsp;<code>grid</code>&nbsp;，分别表示每一个格子里石头的数目。网格图中总共恰好有&nbsp;<code>9</code>&nbsp;个石头，一个格子里可能会有 <strong>多个</strong>&nbsp;石头。</p>
///
/// <p>每一次操作中，你可以将一个石头从它当前所在格子移动到一个至少有一条公共边的相邻格子。</p>
///
/// <p>请你返回每个格子恰好有一个石头的 <strong>最少移动次数</strong>&nbsp;。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <p><img alt="" src="https://assets.leetcode.com/uploads/2023/08/23/example1-3.svg" style="width: 401px; height: 281px;" /></p>
///
/// <pre>
/// <b>输入：</b>grid = [[1,1,0],[1,1,1],[1,2,1]]
/// <b>输出：</b>3
/// <b>解释：</b>让每个格子都有一个石头的一个操作序列为：
/// 1 - 将一个石头从格子 (2,1) 移动到 (2,2) 。
/// 2 - 将一个石头从格子 (2,2) 移动到 (1,2) 。
/// 3 - 将一个石头从格子 (1,2) 移动到 (0,2) 。
/// 总共需要 3 次操作让每个格子都有一个石头。
/// 让每个格子都有一个石头的最少操作次数为 3 。
/// </pre>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <p><img alt="" src="https://assets.leetcode.com/uploads/2023/08/23/example2-2.svg" style="width: 401px; height: 281px;" /></p>
///
/// <pre>
/// <b>输入：</b>grid = [[1,3,0],[1,0,0],[1,0,3]]
/// <b>输出：</b>4
/// <b>解释：</b>让每个格子都有一个石头的一个操作序列为：
/// 1 - 将一个石头从格子 (0,1) 移动到 (0,2) 。
/// 2 - 将一个石头从格子 (0,1) 移动到 (1,1) 。
/// 3 - 将一个石头从格子 (2,2) 移动到 (1,2) 。
/// 4 - 将一个石头从格子 (2,2) 移动到 (2,1) 。
/// 总共需要 4 次操作让每个格子都有一个石头。
/// 让每个格子都有一个石头的最少操作次数为 4 。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>grid.length == grid[i].length == 3</code></li>
/// 	<li><code>0 &lt;= grid[i][j] &lt;= 9</code></li>
/// 	<li><code>grid</code>&nbsp;中元素之和为&nbsp;<code>9</code> 。</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/minimum-moves-to-spread-stones-over-grid/">将石头分散到网格图的最少移动次数</a>
pub struct Solution;

// dfs
impl Solution {
    pub fn minimum_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut from = vec![];
        let mut to = vec![];
        for (i, row) in grid.iter().enumerate() {
            for (j, &cnt) in row.iter().enumerate() {
                if cnt > 1 {
                    from.extend(vec![(i as i32, j as i32); cnt as usize - 1]);
                } else if cnt == 0 {
                    to.push((i as i32, j as i32));
                }
            }
        }

        let mut ans = i32::MAX;
        Self::permute(&mut from, &to, 0, &mut ans);

        ans
    }

    fn permute(from: &mut Vec<(i32, i32)>, to: &Vec<(i32, i32)>, i: usize, ans: &mut i32) {
        if i == from.len() {
            let mut total = 0;
            for (i, f) in from.iter().enumerate() {
                total += (f.0 - to[i].0).abs() + (f.1 - to[i].1).abs();
            }
            *ans = total.min(*ans);
            return;
        }
        Self::permute(from, to, i + 1, ans);
        for j in (i + 1)..from.len() {
            from.swap(i, j);
            Self::permute(from, to, i + 1, ans);
            from.swap(i, j);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::minimum_moves(vec![vec![1, 1, 0], vec![1, 1, 1], vec![1, 2, 1]]),
            3
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::minimum_moves(vec![vec![3, 0, 0], vec![0, 2, 1], vec![1, 0, 2]]),
            5
        )
    }
}
