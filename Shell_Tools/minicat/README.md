# 命令行工具minicat

## Usage
minicat 0.1.0
hustccc <1276675421@qq.com>
Usage

USAGE:
    minicat [OPTIONS] --file <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>        
    -l, --line <line>        
    -n, --number <number>    

可以通过命令`cargo run -- -help`查看用法

## 用法解释
+ minicat类似于Linux上的cat命令，用来查看一个文本信息
+ `--file`参数或者`-f`指定查看的文件
+ 可选参数`--line`或者`-l`指定显示的行数
+ 可选参数`--number`或者`-n`选择是否显示行号
