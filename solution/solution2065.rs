/// 2065.最大化一张图中的路径价值
/// maximum-path-quality-of-a-graph
/// <p>给你一张 <strong>无向</strong>&nbsp;图，图中有 <code>n</code>&nbsp;个节点，节点编号从 <code>0</code>&nbsp;到 <code>n - 1</code>&nbsp;（<strong>都包括</strong>）。同时给你一个下标从 <strong>0</strong>&nbsp;开始的整数数组&nbsp;<code>values</code>&nbsp;，其中&nbsp;<code>values[i]</code>&nbsp;是第 <code>i</code>&nbsp;个节点的 <strong>价值</strong>&nbsp;。同时给你一个下标从 <strong>0</strong>&nbsp;开始的二维整数数组&nbsp;<code>edges</code>&nbsp;，其中&nbsp;<code>edges[j] = [u<sub>j</sub>, v<sub>j</sub>, time<sub>j</sub>]</code>&nbsp;表示节点&nbsp;<code>u<sub>j</sub></code> 和&nbsp;<code>v<sub>j</sub></code>&nbsp;之间有一条需要&nbsp;<code>time<sub>j</sub></code>&nbsp;秒才能通过的无向边。最后，给你一个整数&nbsp;<code>maxTime</code>&nbsp;。</p>
///
/// <p><strong>合法路径</strong>&nbsp;指的是图中任意一条从节点&nbsp;<code>0</code>&nbsp;开始，最终回到节点 <code>0</code>&nbsp;，且花费的总时间 <strong>不超过</strong>&nbsp;<code>maxTime</code> 秒的一条路径。你可以访问一个节点任意次。一条合法路径的 <b>价值</b>&nbsp;定义为路径中 <strong>不同节点</strong>&nbsp;的价值 <strong>之和</strong>&nbsp;（每个节点的价值 <strong>至多</strong>&nbsp;算入价值总和中一次）。</p>
///
/// <p>请你返回一条合法路径的 <strong>最大</strong>&nbsp;价值。</p>
///
/// <p><strong>注意：</strong>每个节点 <strong>至多</strong>&nbsp;有 <strong>四条</strong>&nbsp;边与之相连。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <p><img alt="" src="https://assets.leetcode.com/uploads/2021/10/19/ex1drawio.png" style="width: 269px; height: 170px;" /></p>
///
/// <pre>
/// <b>输入：</b>values = [0,32,10,43], edges = [[0,1,10],[1,2,15],[0,3,10]], maxTime = 49
/// <b>输出：</b>75
/// <strong>解释：</strong>
/// 一条可能的路径为：0 -&gt; 1 -&gt; 0 -&gt; 3 -&gt; 0 。总花费时间为 10 + 10 + 10 + 10 = 40 &lt;= 49 。
/// 访问过的节点为 0 ，1 和 3 ，最大路径价值为 0 + 32 + 43 = 75 。
/// </pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <p><img alt="" src="https://assets.leetcode.com/uploads/2021/10/19/ex2drawio.png" style="width: 269px; height: 170px;" /></p>
///
/// <pre>
/// <b>输入：</b>values = [5,10,15,20], edges = [[0,1,10],[1,2,10],[0,3,10]], maxTime = 30
/// <b>输出：</b>25
/// <strong>解释：</strong>
/// 一条可能的路径为：0 -&gt; 3 -&gt; 0 。总花费时间为 10 + 10 = 20 &lt;= 30 。
/// 访问过的节点为 0 和 3 ，最大路径价值为 5 + 20 = 25 。
/// </pre>
///
/// <p><strong>示例 3：</strong></p>
///
/// <p><img alt="" src="https://assets.leetcode.com/uploads/2021/10/19/ex31drawio.png" style="width: 236px; height: 170px;" /></p>
///
/// <pre>
/// <b>输入：</b>values = [1,2,3,4], edges = [[0,1,10],[1,2,11],[2,3,12],[1,3,13]], maxTime = 50
/// <b>输出：</b>7
/// <strong>解释：</strong>
/// 一条可能的路径为：0 -&gt; 1 -&gt; 3 -&gt; 1 -&gt; 0 。总花费时间为 10 + 13 + 13 + 10 = 46 &lt;= 50 。
/// 访问过的节点为 0 ，1 和 3 ，最大路径价值为 1 + 2 + 4 = 7 。</pre>
///
/// <p><strong>示例 4：</strong></p>
///
/// <p><strong><img alt="" src="https://assets.leetcode.com/uploads/2021/10/21/ex4drawio.png" style="width: 270px; height: 71px;" /></strong></p>
///
/// <pre>
/// <b>输入：</b>values = [0,1,2], edges = [[1,2,10]], maxTime = 10
/// <b>输出：</b>0
/// <b>解释：</b>
/// 唯一一条路径为 0 。总花费时间为 0 。
/// 唯一访问过的节点为 0 ，最大路径价值为 0 。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>n == values.length</code></li>
/// 	<li><code>1 &lt;= n &lt;= 1000</code></li>
/// 	<li><code>0 &lt;= values[i] &lt;= 10<sup>8</sup></code></li>
/// 	<li><code>0 &lt;= edges.length &lt;= 2000</code></li>
/// 	<li><code>edges[j].length == 3 </code></li>
/// 	<li><code>0 &lt;= u<sub>j </sub>&lt; v<sub>j</sub> &lt;= n - 1</code></li>
/// 	<li><code>10 &lt;= time<sub>j</sub>, maxTime &lt;= 100</code></li>
/// 	<li><code>[u<sub>j</sub>, v<sub>j</sub>]</code>&nbsp;所有节点对 <strong>互不相同</strong>&nbsp;。</li>
/// 	<li>每个节点 <strong>至多有四条&nbsp;</strong>边。</li>
/// 	<li>图可能不连通。</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/maximum-path-quality-of-a-graph/">最大化一张图中的路径价值</a>
pub struct Solution;

// 暴力搜索
impl Solution {
    pub fn maximal_path_quality(values: Vec<i32>, edges: Vec<Vec<i32>>, max_time: i32) -> i32 {
        let n = values.len();
        let mut g = vec![vec![]; n];
        for e in edges {
            let (x, y, t) = (e[0] as usize, e[1] as usize, e[2]);
            g[x].push((y, t));
            g[y].push((x, t));
        }

        fn dfs(
            x: usize,
            sum_time: i32,
            sum_value: i32,
            vis: &mut Vec<bool>,
            g: &Vec<Vec<(usize, i32)>>,
            values: &Vec<i32>,
            max_time: i32,
        ) -> i32 {
            let mut ans = if x == 0 { sum_value } else { 0 };
            for &(y, t) in &g[x] {
                if sum_time + t > max_time {
                    continue;
                }
                if vis[y] {
                    ans = ans.max(dfs(y, sum_time + t, sum_value, vis, g, values, max_time));
                } else {
                    vis[y] = true;
                    ans = ans.max(dfs(
                        y,
                        sum_time + t,
                        sum_value + values[y],
                        vis,
                        g,
                        values,
                        max_time,
                    ));
                    vis[y] = false;
                }
            }
            ans
        }

        let mut vis = vec![false; n];
        vis[0] = true;
        dfs(0, 0, values[0], &mut vis, &g, &values, max_time)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::maximal_path_quality(
                vec![0, 32, 10, 43],
                vec![vec![0, 1, 10], vec![1, 2, 15], vec![0, 3, 10]],
                49
            ),
            75
        );
    }
}
