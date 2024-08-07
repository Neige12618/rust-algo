/// 3102.最小化曼哈顿距离
/// minimize-manhattan-distances
/// <p>给你一个下标从 <strong>0</strong> 开始的数组 <code>points</code> ，它表示二维平面上一些点的整数坐标，其中 <code>points[i] = [x<sub>i</sub>, y<sub>i</sub>]</code> 。</p>
///
/// <p>两点之间的距离定义为它们的<span data-keyword="manhattan-distance">曼哈顿距离</span>。</p>
///
/// <p>请你恰好移除一个点，返回移除后任意两点之间的 <strong>最大 </strong>距离可能的 <strong>最小 </strong>值。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>points = [[3,10],[5,15],[10,2],[4,4]]
/// <strong>输出：</strong>12
/// <strong>解释：</strong>移除每个点后的最大距离如下所示：
/// - 移除第 0 个点后，最大距离在点 (5, 15) 和 (10, 2) 之间，为 |5 - 10| + |15 - 2| = 18 。
/// - 移除第 1 个点后，最大距离在点 (3, 10) 和 (10, 2) 之间，为 |3 - 10| + |10 - 2| = 15 。
/// - 移除第 2 个点后，最大距离在点 (5, 15) 和 (4, 4) 之间，为 |5 - 4| + |15 - 4| = 12 。
/// - 移除第 3 个点后，最大距离在点 (5, 15) 和 (10, 2) 之间的，为 |5 - 10| + |15 - 2| = 18 。
/// 在恰好移除一个点后，任意两点之间的最大距离可能的最小值是 12 。
/// </pre>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>points = [[1,1],[1,1],[1,1]]
/// <strong>输出：</strong>0
/// <strong>解释：</strong>移除任一点后，任意两点之间的最大距离都是 0 。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>3 &lt;= points.length &lt;= 10<sup>5</sup></code></li>
/// 	<li><code>points[i].length == 2</code></li>
/// 	<li><code>1 &lt;= points[i][0], points[i][1] &lt;= 10<sup>8</sup></code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/minimize-manhattan-distances/">最小化曼哈顿距离</a>
pub struct Solution;

// 枚举
impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        let mut x = [[i32::MAX, 0], [i32::MAX, 0], [i32::MIN, 0], [i32::MIN, 0]];
        let mut y = x.clone();
        let update = |vals: &mut [[i32; 2]; 4], val: i32, pos: i32| {
            if val < vals[0][0] {
                (vals[0], vals[1]) = ([val, pos], vals[0])
            } else if val < vals[1][0] {
                vals[1] = [val, pos];
            }

            if val > vals[2][0] {
                (vals[2], vals[3]) = ([val, pos], vals[2])
            } else if val > vals[3][0] {
                vals[3] = [val, pos];
            }
        };

        for (i, p) in points.into_iter().enumerate() {
            let (x_val, y_val) = (p[0] + p[1], p[1] - p[0]);
            update(&mut x, x_val, i as i32);
            update(&mut y, y_val, i as i32);
        }

        let mut res = i32::MAX;
        for i in [x[0][1], x[2][1], y[0][1], y[2][1]] {
            let x_max = if i != x[2][1] { x[2][0] } else { x[3][0] };
            let x_min = if i != x[0][1] { x[0][0] } else { x[1][0] };
            let y_max = if i != y[2][1] { y[2][0] } else { y[3][0] };
            let y_min = if i != y[0][1] { y[0][0] } else { y[1][0] };
            res = res.min((x_max - x_min).max(y_max - y_min));
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
            Solution::minimum_distance(vec![vec![3, 10], vec![5, 15], vec![10, 2], vec![4, 4]]),
            12
        )
    }
}
