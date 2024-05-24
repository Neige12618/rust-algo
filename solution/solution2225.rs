/// 2225.找出输掉零场或一场比赛的玩家
/// 中等
///
/// 给你一个整数数组 matches 其中 matches[i] = [winneri, loseri] 表示在一场比赛中 winneri 击败了 loseri 。
///
/// 返回一个长度为 2 的列表 answer ：
/// answer[0] 是所有 没有 输掉任何比赛的玩家列表。
/// answer[1] 是所有恰好输掉 一场 比赛的玩家列表。
/// 两个列表中的值都应该按 递增 顺序返回。
///
/// 注意：
/// 只考虑那些参与 至少一场 比赛的玩家。
/// 生成的测试用例保证 不存在 两场比赛结果 相同 。
///
/// https://leetcode.cn/problems/find-players-with-zero-or-one-losses/description/
pub struct Solution;

use std::collections::HashMap;

/// 哈希
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![], vec![]];
        let mut lose_map = HashMap::new();

        for i in 0..matches.len() {
            let (a, b) = (matches[i][0], matches[i][1]);
            lose_map.entry(a).or_insert(0);
            *lose_map.entry(b).or_insert(0) += 1;
        }

        for (player, lose_count) in lose_map {
            if lose_count < 2 {
                result[lose_count as usize].push(player);
            }
        }

        result[0].sort_unstable();
        result[1].sort_unstable();
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_1() {
        assert_eq!(
            Solution::find_winners(vec![
                vec![1, 3],
                vec![2, 3],
                vec![3, 6],
                vec![5, 6],
                vec![5, 7],
                vec![4, 5],
                vec![4, 8],
                vec![4, 9],
                vec![10, 4],
                vec![10, 9]
            ]),
            vec![vec![1, 2, 10], vec![4, 5, 7, 8]]
        );
    }
}
