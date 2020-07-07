# 命令行工具minicopy

## Usage
minicopy 0.1.0
hustccc <1276675421@qq.com>
Usage

USAGE:
    minicopy [OPTIONS] <source> <target>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --append <append>    

ARGS:
    <source>    source
    <target>    target 

可以通过命令`cargo run -- -help`查看用法

## 用法解释
+ minicopy类似于Linux上的cp命令，用来复制文件
+ `source`为源文件，`target`为目标文件
+ 可选参数`--append`或者`-a`通过后面跟随的`yes`或`no`判断是否为追加模式
