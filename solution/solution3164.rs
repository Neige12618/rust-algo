/// 3164.优质数对的总数 II
/// find-the-number-of-good-pairs-ii
/// <p>给你两个整数数组 <code>nums1</code> 和 <code>nums2</code>，长度分别为 <code>n</code> 和 <code>m</code>。同时给你一个<strong>正整数</strong> <code>k</code>。</p>
///
/// <p>如果 <code>nums1[i]</code> 可以被 <code>nums2[j] * k</code> 整除，则称数对 <code>(i, j)</code> 为 <strong>优质数对</strong>（<code>0 &lt;= i &lt;= n - 1</code>, <code>0 &lt;= j &lt;= m - 1</code>）。</p>
///
/// <p>返回<strong> 优质数对 </strong>的总数。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong><span class="example-io">nums1 = [1,3,4], nums2 = [1,3,4], k = 1</span></p>
///
/// <p><strong>输出：</strong><span class="example-io">5</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p>5个优质数对分别是 <code>(0, 0)</code>, <code>(1, 0)</code>, <code>(1, 1)</code>, <code>(2, 0)</code>, 和 <code>(2, 2)</code>。</p>
/// </div>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong><span class="example-io">nums1 = [1,2,4,12], nums2 = [2,4], k = 3</span></p>
///
/// <p><strong>输出：</strong><span class="example-io">2</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p>2个优质数对分别是 <code>(3, 0)</code> 和 <code>(3, 1)</code>。</p>
/// </div>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= n, m &lt;= 10<sup>5</sup></code></li>
/// 	<li><code>1 &lt;= nums1[i], nums2[j] &lt;= 10<sup>6</sup></code></li>
/// 	<li><code>1 &lt;= k &lt;= 10<sup>3</sup></code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/find-the-number-of-good-pairs-ii/">优质数对的总数 II</a>
pub struct Solution;

use std::{collections::HashMap, ops::Mul};

// 枚举
impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i64 {
        let nums1: Vec<_> = nums1
            .into_iter()
            .filter_map(|x| (x % k == 0).then(|| x / k))
            .collect();
        if nums1.is_empty() {
            return 0;
        }
        let m = nums1.iter().max().copied().unwrap() as usize + 1;
        let mut cnt1 = vec![0; m];
        for num in nums1 {
            cnt1[num as usize] += 1;
        }

        let mut cnt2 = HashMap::new();
        nums2.iter().for_each(|&i| {
            *cnt2.entry(i).or_insert(0) += 1;
        });
        cnt2.iter()
            .map(|(&i, &c)| {
                (i as usize..m)
                    .step_by(i as usize)
                    .map(|i| cnt1[i])
                    .sum::<i64>()
                    .mul(c)
            })
            .sum()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::number_of_pairs(vec![1, 3, 4], vec![1, 3, 4], 1),
            5
        );
        assert_eq!(Solution::number_of_pairs(vec![1, 2], vec![3, 2], 2), 0);
    }
}
