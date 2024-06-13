/// 2813.子序列最大优雅度
/// maximum-elegance-of-a-k-length-subsequence
/// <p>给你一个长度为 <code>n</code> 的二维整数数组 <code>items</code> 和一个整数 <code>k</code> 。</p>
///
/// <p><code>items[i] = [profit<sub>i</sub>, category<sub>i</sub>]</code>，其中 <code>profit<sub>i</sub></code> 和 <code>category<sub>i</sub></code> 分别表示第 <code>i</code> 个项目的利润和类别。</p>
///
/// <p>现定义&nbsp;<code>items</code> 的 <strong>子序列</strong> 的 <strong>优雅度</strong> 可以用 <code>total_profit + distinct_categories<sup>2</sup></code> 计算，其中 <code>total_profit</code> 是子序列中所有项目的利润总和，<code>distinct_categories</code> 是所选子序列所含的所有类别中不同类别的数量。</p>
///
/// <p>你的任务是从 <code>items</code> 所有长度为 <code>k</code> 的子序列中，找出 <strong>最大优雅度</strong> 。</p>
///
/// <p>用整数形式表示并返回 <code>items</code> 中所有长度恰好为 <code>k</code> 的子序列的最大优雅度。</p>
///
/// <p><strong>注意：</strong>数组的子序列是经由原数组删除一些元素（可能不删除）而产生的新数组，且删除不改变其余元素相对顺序。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>items = [[3,2],[5,1],[10,1]], k = 2
/// <strong>输出：</strong>17
/// <strong>解释：
/// </strong>在这个例子中，我们需要选出长度为 2 的子序列。
/// 其中一种方案是 items[0] = [3,2] 和 items[2] = [10,1] 。
/// 子序列的总利润为 3 + 10 = 13 ，子序列包含 2 种不同类别 [2,1] 。
/// 因此，优雅度为 13 + 2<sup>2</sup> = 17 ，可以证明 17 是可以获得的最大优雅度。
/// </pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>items = [[3,1],[3,1],[2,2],[5,3]], k = 3
/// <strong>输出：</strong>19
/// <strong>解释：</strong>
/// 在这个例子中，我们需要选出长度为 3 的子序列。
/// 其中一种方案是 items[0] = [3,1] ，items[2] = [2,2] 和 items[3] = [5,3] 。
/// 子序列的总利润为 3 + 2 + 5 = 10 ，子序列包含 3 种不同类别 [1, 2, 3] 。
/// 因此，优雅度为 10 + 3<sup>2</sup> = 19 ，可以证明 19 是可以获得的最大优雅度。</pre>
///
/// <p><strong>示例 3：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>items = [[1,1],[2,1],[3,1]], k = 3
/// <strong>输出：</strong>7
/// <strong>解释：
/// </strong>在这个例子中，我们需要选出长度为 3 的子序列。
/// 我们需要选中所有项目。
/// 子序列的总利润为 1 + 2 + 3 = 6，子序列包含 1 种不同类别 [1] 。
/// 因此，最大优雅度为 6 + 1<sup>2</sup> = 7 。</pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= items.length == n &lt;= 10<sup>5</sup></code></li>
/// 	<li><code>items[i].length == 2</code></li>
/// 	<li><code>items[i][0] == profit<sub>i</sub></code></li>
/// 	<li><code>items[i][1] == category<sub>i</sub></code></li>
/// 	<li><code>1 &lt;= profit<sub>i</sub> &lt;= 10<sup>9</sup></code></li>
/// 	<li><code>1 &lt;= category<sub>i</sub> &lt;= n </code></li>
/// 	<li><code>1 &lt;= k &lt;= n</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/maximum-elegance-of-a-k-length-subsequence/">子序列最大优雅度</a>
pub struct Solution;

use std::cmp::Reverse;

// 两步贪心
impl Solution {
    pub fn find_maximum_elegance(mut items: Vec<Vec<i32>>, k: i32) -> i64 {
        let k = k as usize;
        let n = items.len();

        let mut set = vec![false; n];
        let mut st = vec![];
        let mut count = 0;

        // 第一贪
        let mut profit = 0;
        items.sort_unstable_by_key(|v| Reverse(v[0]));
        for v in items.iter().take(k) {
            let index = v[1] as usize - 1;
            if !set[index] {
                set[index] = true;
                count += 1;
            } else {
                st.push(v[0]);
            }
            profit += v[0] as i64;
        }
        let mut res = profit + count * count;

        if st.is_empty() {
            return res;
        }

        // 第二贪
        for item in items.iter().skip(k) {
            let index = item[1] as usize - 1;
            if !set[index] {
                if let Some(v) = st.pop() {
                    profit += (item[0] - v) as i64;
                    set[index] = true;
                    count += 1;
                } else {
                    break;
                }
            }
            res = res.max(profit + count * count);
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
            Solution::find_maximum_elegance(vec![vec![1, 1], vec![2, 1], vec![3, 1]], 3),
            7
        );
        assert_eq!(
            Solution::find_maximum_elegance(vec![vec![3, 2], vec![5, 1], vec![10, 1]], 2),
            17
        )
    }

    #[test]
    fn test_2() {
        assert_eq!(
            Solution::find_maximum_elegance(
                vec![
                    vec![2, 2],
                    vec![8, 6],
                    vec![10, 6],
                    vec![2, 4],
                    vec![9, 5],
                    vec![4, 5]
                ],
                4
            ),
            39
        )
    }

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::find_maximum_elegance(
                vec![vec![2, 5], vec![2, 2], vec![7, 5], vec![2, 4], vec![6, 5]],
                2
            ),
            14
        )
    }
}
