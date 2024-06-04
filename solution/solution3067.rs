/// 3067.在带权树网络中统计可连接服务器对数目
/// count-pairs-of-connectable-servers-in-a-weighted-tree-network
/// <p>给你一棵无根带权树，树中总共有 <code>n</code>&nbsp;个节点，分别表示 <code>n</code>&nbsp;个服务器，服务器从 <code>0</code>&nbsp;到 <code>n - 1</code>&nbsp;编号。同时给你一个数组&nbsp;<code>edges</code>&nbsp;，其中&nbsp;<code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>, weight<sub>i</sub>]</code>&nbsp;表示节点&nbsp;<code>a<sub>i</sub></code> 和&nbsp;<code>b<sub>i</sub></code>&nbsp;之间有一条双向边，边的权值为&nbsp;<code>weight<sub>i</sub></code>&nbsp;。再给你一个整数&nbsp;<code>signalSpeed</code>&nbsp;。</p>
///
/// <p>如果两个服务器 <code>a</code>&nbsp;，<code>b</code>&nbsp;和 <code>c</code>&nbsp;满足以下条件，那么我们称服务器 <code>a</code>&nbsp;和 <code>b</code>&nbsp;是通过服务器 <code>c</code>&nbsp;<strong>可连接的</strong>&nbsp;：</p>
///
/// <ul>
/// 	<li><code>a &lt; b</code>&nbsp;，<code>a != c</code> 且&nbsp;<code>b != c</code>&nbsp;。</li>
/// 	<li>从&nbsp;<code>c</code>&nbsp;到&nbsp;<code>a</code>&nbsp;的距离是可以被&nbsp;<code>signalSpeed</code>&nbsp;整除的。</li>
/// 	<li>从&nbsp;<code>c</code>&nbsp;到&nbsp;<code>b</code>&nbsp;的距离是可以被&nbsp;<code>signalSpeed</code>&nbsp;整除的。</li>
/// 	<li>从&nbsp;<code>c</code>&nbsp;到&nbsp;<code>b</code>&nbsp;的路径与从&nbsp;<code>c</code>&nbsp;到&nbsp;<code>a</code>&nbsp;的路径没有任何公共边。</li>
/// </ul>
///
/// <p>请你返回一个长度为 <code>n</code>&nbsp;的整数数组&nbsp;<code>count</code>&nbsp;，其中&nbsp;<code>count[i]</code> 表示通过服务器&nbsp;<code>i</code>&nbsp;<strong>可连接</strong>&nbsp;的服务器对的&nbsp;<strong>数目</strong>&nbsp;。</p>
///
/// <p>&nbsp;</p>
///
/// <p><b>示例 1：</b></p>
///
/// <p><img alt="" src="https://assets.leetcode.com/uploads/2024/01/21/example22.png" style="width: 438px; height: 243px; padding: 10px; background: #fff; border-radius: .5rem;" /></p>
///
/// <pre>
/// <b>输入：</b>edges = [[0,1,1],[1,2,5],[2,3,13],[3,4,9],[4,5,2]], signalSpeed = 1
/// <b>输出：</b>[0,4,6,6,4,0]
/// <b>解释：</b>由于 signalSpeed 等于 1 ，count[c] 等于所有从 c 开始且没有公共边的路径对数目。
/// 在输入图中，count[c] 等于服务器 c 左边服务器数目乘以右边服务器数目。
/// </pre>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <p><img alt="" src="https://assets.leetcode.com/uploads/2024/01/21/example11.png" style="width: 495px; height: 484px; padding: 10px; background: #fff; border-radius: .5rem;" /></p>
///
/// <pre>
/// <b>输入：</b>edges = [[0,6,3],[6,5,3],[0,3,1],[3,2,7],[3,1,6],[3,4,2]], signalSpeed = 3
/// <b>输出：</b>[2,0,0,0,0,0,2]
/// <b>解释：</b>通过服务器 0 ，有 2 个可连接服务器对(4, 5) 和 (4, 6) 。
/// 通过服务器 6 ，有 2 个可连接服务器对 (4, 5) 和 (0, 5) 。
/// 所有服务器对都必须通过服务器 0 或 6 才可连接，所以其他服务器对应的可连接服务器对数目都为 0 。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>2 &lt;= n &lt;= 1000</code></li>
/// 	<li><code>edges.length == n - 1</code></li>
/// 	<li><code>edges[i].length == 3</code></li>
/// 	<li><code>0 &lt;= a<sub>i</sub>, b<sub>i</sub> &lt; n</code></li>
/// 	<li><code>edges[i] = [a<sub>i</sub>, b<sub>i</sub>, weight<sub>i</sub>]</code><!-- notionvc: a2623897-1bb1-4c07-84b6-917ffdcd83ec --></li>
/// 	<li><code>1 &lt;= weight<sub>i</sub> &lt;= 10<sup>6</sup></code></li>
/// 	<li><code>1 &lt;= signalSpeed &lt;= 10<sup>6</sup></code></li>
/// 	<li>输入保证&nbsp;<code>edges</code>&nbsp;构成一棵合法的树。</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/count-pairs-of-connectable-servers-in-a-weighted-tree-network/">在带权树网络中统计可连接服务器对数目</a>
pub struct Solution;

// 树形动态规划
impl Solution {
    pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut nodes = vec![vec![]; n];
        for e in edges {
            nodes[e[0] as usize].push((e[1] as usize, e[2]));
            nodes[e[1] as usize].push((e[0] as usize, e[2]));
        }
        let mut ans = vec![0; n];
        for (i, node) in nodes.iter().enumerate().filter(|v| v.1.len() > 1) {
            let mut s = 0;
            for &(next, dist) in node {
                let cnt = Self::dfs(&nodes, signal_speed, i, next, dist);
                ans[i] += cnt * s;
                s += cnt;
            }
        }
        ans
    }

    fn dfs(
        node: &Vec<Vec<(usize, i32)>>,
        signal_speed: i32,
        fa: usize,
        cur: usize,
        total_dist: i32,
    ) -> i32 {
        let mut ans = if total_dist % signal_speed == 0 { 1 } else { 0 };
        for &(next, dist) in node[cur].iter() {
            if next != fa {
                ans += Self::dfs(node, signal_speed, cur, next, total_dist + dist);
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
            Solution::count_pairs_of_connectable_servers(
                vec![
                    vec![0, 6, 3],
                    vec![6, 5, 3],
                    vec![0, 3, 1],
                    vec![3, 2, 7],
                    vec![3, 1, 6],
                    vec![3, 4, 2]
                ],
                3
            ),
            vec![2, 0, 0, 0, 0, 0, 2]
        );
    }
}
