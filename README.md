# mool

个人本科的毕业设计，使用 Rust 编写的一个 Torchscript 转 LLVM 的编译器。

## 支持的语法

- 函数声明、函数调用、函数返回
- 变量和作用域
- 类型和类型注解（int、float、bool、tensor）
- 张量加减乘除算子

支持的语法很少，但是 Rust 的 Parser 和 LLVM Codegen 的资料很少，对于刚入门不知道从何下手的人来说，可能有点参考价值。

## 运行环境

- LLVM 13

  不是绝对的，版本对不上或许也能跑起来，注意 `crates/*/Cargo.toml` 中的 `llvm-sys` 库的版本，与 LLVM 版本保持一致即可，比如 LLVM 13 对应 `llvm-sys = "130"`。

- Rust
  
  使用 rustup 切换到 stable 版本即可

## 运行代码

### 直接运行

```shell
cargo run example/torchscript/* -s torchscript
```

结果会保存到 `example/mool` 和 `example/llvm` 下

如果需要显示中间过程，加上`-d`:

```shell
cargo run example/torchscript/* -s torchscript -d
```

### 编译运行

可以先编译，然后运行编译后的命令行文件

编译：

```shell
sh ./bin/build.sh
```

运行命令行文件：

```shell
# 帮助
.bin/mool-cli -h

# 编译 torchscript 测试样例（普通模式）
.bin/mool-cli example/torchscript/* -s torchscript
# 编译 torchscript 测试样例（调试模式）
.bin/mool-cli example/torchscript/* -s torchscript -d
# 编译 mool 测试样例（普通模式）
.bin/mool-cli example/mool/* -s mool
# 编译 mool 测试样例（调试模式）
.bin/mool-cli example/mool/* -s mool -d
```
