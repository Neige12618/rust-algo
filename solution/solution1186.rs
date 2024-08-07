/// 1186.删除一次得到子数组最大和
/// maximum-subarray-sum-with-one-deletion
/// <p>给你一个整数数组，返回它的某个&nbsp;<strong>非空</strong> 子数组（连续元素）在执行一次可选的删除操作后，所能得到的最大元素总和。换句话说，你可以从原数组中选出一个子数组，并可以决定要不要从中删除一个元素（只能删一次哦），（删除后）子数组中至少应当有一个元素，然后该子数组（剩下）的元素总和是所有子数组之中最大的。</p>
///
/// <p>注意，删除一个元素后，子数组 <strong>不能为空</strong>。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>arr = [1,-2,0,3]
/// <strong>输出：</strong>4
/// <strong>解释：</strong>我们可以选出 [1, -2, 0, 3]，然后删掉 -2，这样得到 [1, 0, 3]，和最大。</pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>arr = [1,-2,-2,3]
/// <strong>输出：</strong>3
/// <strong>解释：</strong>我们直接选出 [3]，这就是最大和。
/// </pre>
///
/// <p><strong>示例 3：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>arr = [-1,-1,-1,-1]
/// <strong>输出：</strong>-1
/// <strong>解释：</strong>最后得到的子数组不能为空，所以我们不能选择 [-1] 并从中删去 -1 来得到 0。
///      我们应该直接选择 [-1]，或者选择 [-1, -1] 再从中删去一个 -1。
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
/// <meta charset="UTF-8" />
///
/// <ul>
/// 	<li><code>1 &lt;= arr.length &lt;= 10<sup>5</sup></code></li>
/// 	<li><code>-10<sup>4</sup>&nbsp;&lt;= arr[i] &lt;= 10<sup>4</sup></code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/maximum-subarray-sum-with-one-deletion/">删除一次得到子数组最大和</a>
pub struct Solution;

// 动态规划
impl Solution {
    pub fn maximum_sum(arr: Vec<i32>) -> i32 {
        arr.iter().fold([i32::MIN / 2; 3], |i, &x| {
            let f1 = i[0].max(i[1] + x);
            let f0 = i[0].max(0) + x;
            [f0, f1, i[2].max(f0).max(f1)]
        })[2]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(Solution::maximum_sum(vec![1, -2, 0, 3]), 4)
    }
}
