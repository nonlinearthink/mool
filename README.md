# mool

个人本科的毕业设计，使用 Rust 编写的一个 Torchscript 转 LLVM 的编译器。

## 运行环境

- LLVM 13

  如果不使用 LLVM 13 的话可能需要更改所有 `crates/*/Cargo.toml` 中的 `llvm-sys` 库的版本，与 LLVM 版本保持一致，比如 LLVM 13 对应 `llvm-sys = "130"`

- Rustup
  
  任意支持 rust 2018 的 rustup 版本

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

或者，可以先编译，然后运行编译后的命令行文件

编译：

```shell
sh ./bin/build.sh
```

运行命令行文件：

```shell
.bin/mool-cli -h
```

## 支持的语法

- 函数声明、函数调用、函数返回
- 变量和作用域
- 类型和类型注解（int、float、bool、tensor）
- 张量加减乘除算子

支持的语法很少，但是 Rust 的 Parser 和 LLVM Codegen 的资料很少，对于刚入门不知道从何下手的人来说，可能有点参考价值。
