/// 3099.哈沙德数
/// harshad-number
/// <p>如果一个整数能够被其各个数位上的数字之和整除，则称之为<strong> 哈沙德数</strong>（Harshad number）。给你一个整数 <code>x</code> 。如果 <code>x</code> 是 <strong>哈沙德数 </strong>，则返回<em> </em><code>x</code> 各个数位上的数字之和，否则，返回<em> </em><code>-1</code> 。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong> <span class="example-io">x = 18</span></p>
///
/// <p><strong>输出：</strong> <span class="example-io">9</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p><code>x</code> 各个数位上的数字之和为 <code>9</code> 。<code>18</code> 能被 <code>9</code> 整除。因此 <code>18</code> 是哈沙德数，答案是 <code>9</code> 。</p>
/// </div>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong> <span class="example-io">x = 23</span></p>
///
/// <p><strong>输出：</strong> <span class="example-io">-1</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p><code>x</code> 各个数位上的数字之和为 <code>5</code> 。<code>23</code> 不能被 <code>5</code> 整除。因此 <code>23</code> 不是哈沙德数，答案是 <code>-1</code> 。</p>
/// </div>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= x &lt;= 100</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/harshad-number/">哈沙德数</a>
pub struct Solution;

// 模拟
impl Solution {
    pub fn sum_of_the_digits_of_harshad_number(x: i32) -> i32 {
        let mut sum = 0;
        let mut t = x;

        while t != 0 {
            sum += t % 10;
            t /= 10;
        }

        if x % sum == 0 {
            sum
        } else {
            -1
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::sum_of_the_digits_of_harshad_number(18), 9);
    }
}
