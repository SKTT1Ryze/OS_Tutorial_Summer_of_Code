# 命令行工具miniecho

## Usage
miniecho 0.1.0
hustccc <1276675421@qq.com>
Usage

USAGE:
    miniecho [OPTIONS] <content>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -a, --append <append>    
    -f, --file <file>        

ARGS:
    <content>    content

可以通过命令`cargo run -- -help`查看用法

## 用法解释
+ miniecho类似于Linux上的echo命令，用于输出字符串或者写入文件
+ 可选参数`--file`参数或者`-f`指定写入的文件，如果不指定则输出到标准输出
+ 可选参数`--append`或者`-a`通过后面跟随的`yes`或`no`判断是否为追加模式写入
