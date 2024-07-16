/// 2956.找到两个数组中的公共元素
/// find-common-elements-between-two-arrays
/// <p>给你两个下标从 <strong>0</strong>&nbsp;开始的整数数组&nbsp;<code>nums1</code>&nbsp;和&nbsp;<code>nums2</code>&nbsp;，它们分别含有 <code>n</code>&nbsp;和 <code>m</code>&nbsp;个元素。请你计算以下两个数值：</p>
///
/// <ul>
/// 	<li><code>answer1</code>：使得&nbsp;<code>nums1[i]</code>&nbsp;在&nbsp;<code>nums2</code>&nbsp;中出现的下标&nbsp;<code>i</code>&nbsp;的数量。</li>
/// 	<li><code>answer2</code>：使得&nbsp;<code>nums2[i]</code>&nbsp;在&nbsp;<code>nums1</code>&nbsp;中出现的下标&nbsp;<code>i</code>&nbsp;的数量。</li>
/// </ul>
///
/// <p>返回 <code>[answer1, answer2]</code>。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong class="example">示例 1：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong><span class="example-io">nums1 = [2,3,2], nums2 = [1,2]</span></p>
///
/// <p><strong>输出：</strong><span class="example-io">[2,1]</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p><img src="https://assets.leetcode.com/uploads/2024/05/26/3488_find_common_elements_between_two_arrays-t1.gif" style="width: 225px; height: 150px;" /></p>
/// </div>
///
/// <p><strong class="example">示例 2：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong><span class="example-io">nums1 = [4,3,2,3,1], nums2 = [2,2,5,2,3,6]</span></p>
///
/// <p><strong>输出：</strong><span class="example-io">[3,4]</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p><code>nums1</code>&nbsp;中下标在 1，2，3 的元素在&nbsp;<code>nums2</code>&nbsp;中也存在。所以&nbsp;<code>answer1</code>&nbsp;为&nbsp;3。</p>
///
/// <p><code>nums2</code>&nbsp;中下标在 0，1，3，4 的元素在&nbsp;<code>nums1</code>&nbsp;中也存在。所以&nbsp;<code>answer2</code>&nbsp;为 4。</p>
/// </div>
///
/// <p><strong class="example">示例 3：</strong></p>
///
/// <div class="example-block">
/// <p><strong>输入：</strong><span class="example-io">nums1 = [3,4,2,3], nums2 = [1,5]</span></p>
///
/// <p><strong>输出：</strong><span class="example-io">[0,0]</span></p>
///
/// <p><strong>解释：</strong></p>
///
/// <p><code>nums1</code>&nbsp;和&nbsp;<code>nums2</code>&nbsp;中没有相同的数字，所以答案是 [0,0]。</p>
/// </div>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>n == nums1.length</code></li>
/// 	<li><code>m == nums2.length</code></li>
/// 	<li><code>1 &lt;= n, m &lt;= 100</code></li>
/// 	<li><code>1 &lt;= nums1[i], nums2[i] &lt;= 100</code></li>
/// </ul>
/// <a href="https://leetcode.cn/problems/find-common-elements-between-two-arrays/">找到两个数组中的公共元素</a>
pub struct Solution;

use std::collections::HashSet;

// 哈希 遍历
impl Solution {
    pub fn find_intersection_values(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let set1: HashSet<_> = HashSet::from_iter(nums1.iter());
        let set2: HashSet<_> = HashSet::from_iter(nums2.iter());
        let cnt1 = nums1.iter().filter(|x| set2.contains(x)).count() as i32;
        let cnt2 = nums2.iter().filter(|x| set1.contains(x)).count() as i32;
        vec![cnt1, cnt2]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_intersection_values(vec![2, 3, 2], vec![1, 2]),
            vec![2, 1]
        )
    }
}
