/// 2101.引爆最多的炸弹
/// detonate-the-maximum-bombs
/// <p>给你一个炸弹列表。一个炸弹的 <strong>爆炸范围</strong>&nbsp;定义为以炸弹为圆心的一个圆。</p>
///
/// <p>炸弹用一个下标从 <strong>0</strong>&nbsp;开始的二维整数数组&nbsp;<code>bombs</code>&nbsp;表示，其中&nbsp;<code>bombs[i] = [x<sub>i</sub>, y<sub>i</sub>, r<sub>i</sub>]</code>&nbsp;。<code>x<sub>i</sub></code> 和&nbsp;<code>y<sub>i</sub></code>&nbsp;表示第 <code>i</code>&nbsp;个炸弹的 X 和 Y 坐标，<code>r<sub>i</sub></code>&nbsp;表示爆炸范围的 <strong>半径</strong>&nbsp;。</p>
///
/// <p>你需要选择引爆 <strong>一个&nbsp;</strong>炸弹。当这个炸弹被引爆时，<strong>所有</strong> 在它爆炸范围内的炸弹都会被引爆，这些炸弹会进一步将它们爆炸范围内的其他炸弹引爆。</p>
///
/// <p>给你数组&nbsp;<code>bombs</code>&nbsp;，请你返回在引爆&nbsp;<strong>一个</strong>&nbsp;炸弹的前提下，<strong>最多</strong>&nbsp;能引爆的炸弹数目。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <p><img alt="" src="https://assets.leetcode.com/uploads/2021/11/06/desmos-eg-3.png" style="width: 300px; height: 300px;"></p>
///
/// <pre><b>输入：</b>bombs = [[2,1,3],[6,1,4]]
/// <b>输出：</b>2
/// <strong>解释：</strong>
/// 上图展示了 2 个炸弹的位置和爆炸范围。
/// 如果我们引爆左边的炸弹，右边的炸弹不会被影响。
/// 但如果我们引爆右边的炸弹，两个炸弹都会爆炸。
/// 所以最多能引爆的炸弹数目是 max(1, 2) = 2 。
/// </pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <p><img alt="" src="https://assets.leetcode.com/uploads/2021/11/06/desmos-eg-2.png" style="width: 300px; height: 300px;"></p>
///
/// <pre><b>输入：</b>bombs = [[1,1,5],[10,10,5]]
/// <b>输出：</b>1
/// <strong>解释：
/// </strong>引爆任意一个炸弹都不会引爆另一个炸弹。所以最多能引爆的炸弹数目为 1 。
/// </pre>
///
/// <p><strong>示例 3：</strong></p>
///
/// <p><img alt="" src="https://assets.leetcode.com/uploads/2021/11/07/desmos-eg1.png" style="width: 300px; height: 300px;"></p>
///
/// <pre><b>输入：</b>bombs = [[1,2,3],[2,3,1],[3,4,2],[4,5,3],[5,6,4]]
/// <b>输出：</b>5
/// <strong>解释：</strong>
/// 最佳引爆炸弹为炸弹 0 ，因为：
/// - 炸弹 0 引爆炸弹 1 和 2 。红色圆表示炸弹 0 的爆炸范围。
/// - 炸弹 2 引爆炸弹 3 。蓝色圆表示炸弹 2 的爆炸范围。
/// - 炸弹 3 引爆炸弹 4 。绿色圆表示炸弹 3 的爆炸范围。
/// 所以总共有 5 个炸弹被引爆。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= bombs.length&nbsp;&lt;= 100</code></li>
/// 	<li><code>bombs[i].length == 3</code></li>
/// 	<li><code>1 &lt;= x<sub>i</sub>, y<sub>i</sub>, r<sub>i</sub> &lt;= 10<sup>5</sup></code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/detonate-the-maximum-bombs/">引爆最多的炸弹</a>
pub struct Solution;

use std::collections::{HashMap, VecDeque};

// bfs
impl Solution {
    pub fn maximum_detonation(b: Vec<Vec<i32>>) -> i32 {
        let n = b.len();
        let is_connected = |u: usize, v: usize| -> bool {
            let dx = (b[u][0] - b[v][0]) as i64;
            let dy = (b[u][1] - b[v][1]) as i64;
            (b[u][2] as i64) * (b[u][2] as i64) >= (dx * dx + dy * dy)
        };

        let mut edges = HashMap::new();
        for i in 0..n {
            for j in 0..n {
                if i != j && is_connected(i, j) {
                    edges.entry(i).or_insert(vec![]).push(j);
                }
            }
        }

        let mut res = 0;
        for i in 0..n {
            let mut vis = vec![0; n];
            let mut cnt = 1;
            let mut q = VecDeque::with_capacity(n);
            q.push_back(i);
            vis[i] = 1;
            while let Some(cidx) = q.pop_front() {
                for &nidx in edges.get(&cidx).unwrap_or(&vec![]) {
                    if vis[nidx] == 1 {
                        continue;
                    }
                    cnt += 1;
                    q.push_back(nidx);
                    vis[nidx] = 1;
                }
            }
            res = res.max(cnt);
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::maximum_detonation(vec![vec![2, 1, 3], vec![6, 1, 4]]),
            2
        )
    }
}
