/// 274.H 指数
///
/// 中等
///
/// 给你一个整数数组 citations ，其中 citations[i] 表示研究者的第 i 篇论文被引用的次数。
/// 计算并返回该研究者的 h 指数。
/// 根据维基百科上 h 指数的定义：h 代表“高引用次数” ，一名科研人员的 h 指数 是指他（她）
/// 至少发表了 h 篇论文，并且 至少 有 h 篇论文被引用次数大于等于 h 。如果 h 有多种可能的值，
/// h 指数 是其中最大的那个。
/// https://leetcode.cn/problems/h-index/description/
pub struct Solution;

/// 排序
impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort_unstable();
        let n = citations.len();
        let mut ans = 0;
        for i in (0..n).rev() {
            if citations[i] > ans {
                ans += 1;
            } else {
                break;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(Solution::h_index(vec![3, 0, 6, 1, 5]), 3);
    }
}
