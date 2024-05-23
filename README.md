# rust algo

使用 rust 完成 leetcode 每日一题。

## 运行

在根目录下运行

```shell
cargo run fetch
```

从 leetcode 获取每日一题，并保存到 `solution/solution<solution_id>.rs`

执行

```shell
cargo run submit
```

提交当天的题目

## 测试

在根目录下运行

```shell
cargo test -- solution<solution_id>
```
