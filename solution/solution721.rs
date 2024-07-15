/// 721.账户合并
/// accounts-merge
/// <p>给定一个列表 <code>accounts</code>，每个元素 <code>accounts[i]</code>&nbsp;是一个字符串列表，其中第一个元素 <code>accounts[i][0]</code>&nbsp;是&nbsp;<em>名称 (name)</em>，其余元素是 <em><strong>emails</strong> </em>表示该账户的邮箱地址。</p>
///
/// <p>现在，我们想合并这些账户。如果两个账户都有一些共同的邮箱地址，则两个账户必定属于同一个人。请注意，即使两个账户具有相同的名称，它们也可能属于不同的人，因为人们可能具有相同的名称。一个人最初可以拥有任意数量的账户，但其所有账户都具有相同的名称。</p>
///
/// <p>合并账户后，按以下格式返回账户：每个账户的第一个元素是名称，其余元素是 <strong>按字符 ASCII 顺序排列</strong> 的邮箱地址。账户本身可以以 <strong>任意顺序</strong> 返回。</p>
///
/// <p>&nbsp;</p>
///
/// <p><strong>示例 1：</strong></p>
///
/// <pre>
/// <b>输入：</b>accounts = [["John", "johnsmith@mail.com", "john00@mail.com"], ["John", "johnnybravo@mail.com"], ["John", "johnsmith@mail.com", "john_newyork@mail.com"], ["Mary", "mary@mail.com"]]
/// <b>输出：</b>[["John", 'john00@mail.com', 'john_newyork@mail.com', 'johnsmith@mail.com'],  ["John", "johnnybravo@mail.com"], ["Mary", "mary@mail.com"]]
/// <b>解释：</b>
/// 第一个和第三个 John 是同一个人，因为他们有共同的邮箱地址 "johnsmith@mail.com"。
/// 第二个 John 和 Mary 是不同的人，因为他们的邮箱地址没有被其他帐户使用。
/// 可以以任何顺序返回这些列表，例如答案 [['Mary'，'mary@mail.com']，['John'，'johnnybravo@mail.com']，
/// ['John'，'john00@mail.com'，'john_newyork@mail.com'，'johnsmith@mail.com']] 也是正确的。
/// </pre>
///
/// <p><strong>示例 2：</strong></p>
///
/// <pre>
/// <strong>输入：</strong>accounts = [["Gabe","Gabe0@m.co","Gabe3@m.co","Gabe1@m.co"],["Kevin","Kevin3@m.co","Kevin5@m.co","Kevin0@m.co"],["Ethan","Ethan5@m.co","Ethan4@m.co","Ethan0@m.co"],["Hanzo","Hanzo3@m.co","Hanzo1@m.co","Hanzo0@m.co"],["Fern","Fern5@m.co","Fern1@m.co","Fern0@m.co"]]
/// <strong>输出：</strong>[["Ethan","Ethan0@m.co","Ethan4@m.co","Ethan5@m.co"],["Gabe","Gabe0@m.co","Gabe1@m.co","Gabe3@m.co"],["Hanzo","Hanzo0@m.co","Hanzo1@m.co","Hanzo3@m.co"],["Kevin","Kevin0@m.co","Kevin3@m.co","Kevin5@m.co"],["Fern","Fern0@m.co","Fern1@m.co","Fern5@m.co"]]
/// </pre>
///
/// <p>&nbsp;</p>
///
/// <p><strong>提示：</strong></p>
///
/// <ul>
/// 	<li><code>1 &lt;= accounts.length &lt;= 1000</code></li>
/// 	<li><code>2 &lt;= accounts[i].length &lt;= 10</code></li>
/// 	<li><code>1 &lt;= accounts[i][j].length &lt;= 30</code></li>
/// 	<li><code>accounts[i][0]</code> 由英文字母组成</li>
/// 	<li><code>accounts[i][j] (for j &gt; 0)</code> 是有效的邮箱地址</li>
/// </ul>
/// <a href="https://leetcode.cn/problems/accounts-merge/">账户合并</a>
pub struct Solution;

use std::collections::{HashMap, HashSet};

// 哈希 dfs
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut email_to_idx = HashMap::new();
        for (i, account) in accounts.iter().enumerate() {
            for email in account.iter().skip(1) {
                email_to_idx.entry(email).or_insert(Vec::new()).push(i);
            }
        }

        fn dfs(
            i: usize,
            accounts: &Vec<Vec<String>>,
            email_to_idx: &HashMap<&String, Vec<usize>>,
            vis: &mut Vec<bool>,
            email_set: &mut HashSet<String>,
        ) {
            vis[i] = true;
            for email in accounts[i].iter().skip(1) {
                if email_set.insert(email.clone()) {
                    for &j in email_to_idx.get(email).unwrap() {
                        if !vis[j] {
                            dfs(j, accounts, email_to_idx, vis, email_set);
                        }
                    }
                }
            }
        }

        let mut ans = vec![];
        let mut vis = vec![false; accounts.len()];
        for (i, account) in accounts.iter().enumerate() {
            if vis[i] {
                continue;
            }
            let mut email_set = HashSet::new();
            dfs(i, &accounts, &email_to_idx, &mut vis, &mut email_set);

            let mut r = vec![account[0].clone()];
            r.extend(email_set);
            r[1..].sort_unstable();

            ans.push(r);
        }
        ans
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(
            Solution::accounts_merge(vec![
                vec![
                    "John".to_string(),
                    "johnsmith@mail.com".to_string(),
                    "john00@mail.com".to_string()
                ],
                vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
                vec![
                    "John".to_string(),
                    "johnsmith@mail.com".to_string(),
                    "john_newyork@mail.com".to_string()
                ],
                vec!["Mary".to_string(), "mary@mail.com".to_string()]
            ]),
            vec![
                vec![
                    "John".to_string(),
                    "john00@mail.com".to_string(),
                    "john_newyork@mail.com".to_string(),
                    "johnsmith@mail.com".to_string()
                ],
                vec!["John".to_string(), "johnnybravo@mail.com".to_string()],
                vec!["Mary".to_string(), "mary@mail.com".to_string()]
            ]
        )
    }
}
