# rust algo

使用 rust 完成 leetcode 每日一题。

## 运行

### 前置要求

创建`.env`文件，并且在其中添加以下内容

```
LEETCODE_BASE_URL=https://leetcode.cn
LEETCODE_SESSION=<登陆后从浏览器中获取>
```

### 运行

在 bin 目录下运行

```shell
cargo run fetch
```

从 leetcode 获取每日一题，并保存到 `solution/solution<solution_id>.rs`

bin 目录下执行

```shell
cargo run submit
```

提交当天的题目

对于非每日一题的题目，可以执行

```
cargo run fetch -f <search_keyword>
```

来根据 search_keyword 搜索题目，并保存到 `solution/solution<solution_id>.rs`

对于非每日一题的题目，可以执行

```
cargo run submit -i <solution_id or title_slug>
```

来根据 solution_id 搜索题目，并提交

## 测试

在根目录下运行

```shell
cargo test -- solution<solution_id>
```
