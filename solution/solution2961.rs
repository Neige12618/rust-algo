/// 2961.双模幂运算
/// double-modular-exponentiation
/// <p>给你一个下标从 <strong>0 </strong>开始的二维数组 <code>variables</code> ，其中 <code>variables[i] = [a<sub>i</sub>, b<sub>i</sub>, c<sub>i,</sub> m<sub>i</sub>]</code>，以及一个整数 <code>target</code> 。</p>
///
/// <p>如果满足以下公式，则下标 <code>i</code> 是 <strong>好下标</strong>：</p>
///
/// <ul>
/// 	<li><code>0 &lt;= i &lt; variables.length</code></li>
/// 	<li><code>((a<sub>i</sub><sup>b<sub>i</sub></sup> % 10)<sup>c<sub>i</sub></sup>) % m<sub>i</sub> == target</code></li>
/// </ul>
///
/// <p>返回一个由<strong> 好下标 </strong>组成的数组，<strong>顺序不限</strong> 。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>variables = [[2,3,3,10],[3,3,3,1],[6,1,1,4]], target = 2
/// <strong>输出：</strong>[0,2]
/// <strong>解释：</strong>对于 variables 数组中的每个下标 i ：
/// 1) 对于下标 0 ，variables[0] = [2,3,3,10] ，(2<sup>3</sup> % 10)<sup>3</sup> % 10 = 2 。
/// 2) 对于下标 1 ，variables[1] = [3,3,3,1] ，(3<sup>3</sup> % 10)<sup>3</sup> % 1 = 0 。
/// 3) 对于下标 2 ，variables[2] = [6,1,1,4] ，(6<sup>1</sup> % 10)<sup>1</sup> % 4 = 2 。
/// 因此，返回 [0,2] 作为答案。
/// </pre>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>variables = [[39,3,1000,1000]], target = 17
/// <strong>输出：</strong>[]
/// <strong>解释：</strong>对于 variables 数组中的每个下标 i ：
/// 1) 对于下标 0 ，variables[0] = [39,3,1000,1000] ，(39<sup>3</sup> % 10)<sup>1000</sup> % 1000 = 1 。
/// 因此，返回 [] 作为答案。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= variables.length &lt;= 100</code></li>
/// 	<li><code>variables[i] == [a<sub>i</sub>, b<sub>i</sub>, c<sub>i</sub>, m<sub>i</sub>]</code></li>
/// 	<li><code>1 &lt;= a<sub>i</sub>, b<sub>i</sub>, c<sub>i</sub>, m<sub>i</sub> &lt;= 10<sup>3</sup></code></li>
/// 	<li><code><font face="monospace">0 &lt;= target &lt;= 10<sup>3</sup></font></code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/double-modular-exponentiation/">双模幂运算</a>
pub struct Solution;

impl Solution {
    pub fn get_good_indices(variables: Vec<Vec<i32>>, target: i32) -> Vec<i32> {
        let pow = |mut x, mut n, m| {
            let mut res = 1;
            while n > 0 {
                if n % 2 > 0 {
                    res = res * x % m;
                }
                x = x * x % m;
                n /= 2;
            }
            res
        };
        let check = |v: &Vec<_>| pow(pow(v[0], v[1], 10), v[2], v[3]) == target;
        variables
            .iter()
            .enumerate()
            .filter_map(|(i, v)| check(v).then_some(i as i32))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::get_good_indices(
                vec![vec![2, 3, 3, 10], vec![3, 3, 3, 1], vec![6, 1, 1, 4]],
                2
            ),
            vec![0, 2]
        )
    }
}
