/// LCP 40.心算挑战
/// uOAnQW
/// 「力扣挑战赛」心算项目的挑战比赛中，要求选手从 `N` 张卡牌中选出 `cnt` 张卡牌，若这 `cnt` 张卡牌数字总和为偶数，则选手成绩「有效」且得分为 `cnt` 张卡牌数字总和。
/// 给定数组 `cards` 和 `cnt`，其中 `cards[i]` 表示第 `i` 张卡牌上的数字。 请帮参赛选手计算最大的有效得分。若不存在获取有效得分的卡牌方案，则返回 0。
///
/// **示例 1：**
/// >输入：`cards = [1,2,8,9], cnt = 3`
/// >
/// >输出：`18`
/// >
/// >解释：选择数字为 1、8、9 的这三张卡牌，此时可获得最大的有效得分 1+8+9=18。
///
/// **示例 2：**
/// >输入：`cards = [3,3,1], cnt = 1`
/// >
/// >输出：`0`
/// >
/// >解释：不存在获取有效得分的卡牌方案。
///
/// **提示：**
/// - `1 <= cnt <= cards.length <= 10^5`
/// - `1 <= cards[i] <= 1000`
/// <a href="https://leetcode.cn/problems/uOAnQW/">心算挑战</a>
pub struct Solution;

impl Solution {
    pub fn maxmium_score(mut cards: Vec<i32>, cnt: i32) -> i32 {
        cards.sort_unstable_by(|a, b| b.cmp(a));
        let cnt = cnt as usize;
        let s = cards[..cnt].iter().sum::<i32>();
        if s % 2 == 0 {
            return s;
        }

        let replace_sum = |x: i32| -> i32 {
            for &v in cards[cnt..].iter() {
                if v % 2 != x % 2 {
                    return s - x + v;
                }
            }
            0
        };

        let x = cards[cnt - 1];
        let mut ans = replace_sum(x);
        for &v in cards[..cnt - 1].iter().rev() {
            if v % 2 != x % 2 {
                ans = ans.max(replace_sum(v));
                break;
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
        assert_eq!(Solution::maxmium_score(vec![1, 2, 8, 9], 3), 18)
    }
}
