# DailySchedule

## **TOC**

 

*六月*                

| Mon                  | Tues                   | Wed                  | Thur                 | Fri                  | Sat                   | Sun                  |
| -------------------- | ---------------------- | -------------------- | -------------------- | -------------------- | --------------------- | -------------------- |
| 1   | 2   | 3  | 4   | 5   | 6  | 7  |
| 8  | 9   | 10 | 11 | 12  | 13 | 14 |
| 15  | 16 | 17 | 18 | 19  | 20  | 21 |
| 22  | 23 | 24 | 25  | 26  | 27  | 28  |
| 29  | 30 <br> ([D0](#0))   |                      |                      |                      |                       |                      |

*七月*

| Mon                  | Tues                 | Wed                  | Thur                 | Fri                  | Sat                  | Sun                  |
|----------------------|----------------------|----------------------|----------------------|----------------------|----------------------|----------------------|
|                      |                      | 1 <br> ([D1](#1))   | 2 <br> ([D2](#2))                  | 3 <br> ([D3](#3))                    | 4 <br> ([D4](#4))                    | 5 <br> ([D5](#5))                    |
| 6 <br> ([D6](#6))                   | 7 <br> ([D7](#7))                   | 8 <br> ([D8](#8))                    | 9 <br> ([D9](#9))| 10 <br> ([D10](#10))  | 11 <br> ([D11](#11)) | 12 <br> ([D12](#12)) |
| 13 <br> ([D13](#13)) | 14 <br> ([D14](#14)) | 15 <br> ([D15](#15)) | 16  <br> ([D16](#16))| 17 <br> ([D17](#17)) | 18 <br> ([D18](#18)) | 19 <br> ([D19](#19)) |
| 20 <br> ([D20](#20)) | 21 <br> ([D21](#21)) | 22 <br> ([D22](#22))  | 23 <br> ([D23](#23)) | 24 <br> ([D24](#24)) | 25 <br> ([D25](#25)) | 26 <br> ([D26](#26)) |
| 27 <br> ([D27](#27)) | 28 <br> ([D28](#28)) | 29 <br> ([D29](#29))  | 30 <br> ([D30](#30)) |                      |                      |                      |


## OLD TOC Day 0~60


* [Day 0](#0)  
* [Day   1](#Day001)   
* [Day   2](#Day002)   
* [Day   3](#Day003)  
* [Day   4](#Day004)  
* [Day   5](#Day005)  
* [Day   6](#Day006)  
* [Day   7](#Day007)  
* [Day   8](#Day008)  
* [Day   9](#Day009)  
* [Day  10](#Day010)  
* [Day  11](#Day011)  
* [Day  12](#Day012)  
* [Day  13](#Day013)  
* [Day  14](#Day014)   
* [Day  15](#Day015)  
* [Day  16](#Day016)  
* [Day  17](#Day017)  
* [Day  18](#Day018)  
* [Day  19](#Day019)  
* [Day  20](#Day020)  
* [Day  21](#Day021)  
* [Day  22](#Day022)  
* [Day  23](#Day023)  
* [Day  24](#Day024)  
* [Day  25](#Day025)
* [Day  26](#Day026)
* [Day  27](#Day027)  
* [Day  28](#Day028)  
* [Day  29](#Day029)  
* [Day  30](#Day030)  
* [Day  31](#Day031) 
* [Day  32](#Day032) 
* [Day  33](#Day033) 
* [Day  34](#Day034) 
* [Day  35](#Day035) 
* [Day  36](#Day036) 
* [Day  37](#Day037)  
* [Day  38](#Day038)  
* [Day  39](#Day039)  
* [Day  40](#Day040)⭐  
* [Day  41](#Day041)  
* [Day  42](#Day042)  
* [Day  43](#Day043)  
* [Day  44](#Day044)  
* [Day  45](#Day045)  
* [Day  46](#Day046)  
* [Day  47](#Day047)  
* [Day  48](#Day048)  
* [Day  49](#Day049)  
* [Day  50](#Day050)  
* [Day  51](#Day051)  
* [Day  52](#Day052)  
* [Day  53](#Day053)  
* [Day  54](#Day054)
* [Day  55](#Day055)
* [Day  56](#Day056)
* [Day  57](#Day057)
* [Day  58](#Day058)⭐
* [Day  59](#Day059)
* [Day  60](#Day060)

<span id="0"></span>

## Day 0
### 预期计划
+ 配置好rust语言在Linux上的开发环境，包括编译，运行和代码管理
+ 给rustup换成国内源
+ 初步了解rust语法

### 事件1： 决定参加2020年OS Tutorial Summer of Code活动
在github上看到活动信息，稍微思考后决定报名参加。我本来就喜欢Linux，对操作系统很感兴趣，一直想要自己写一个操作系统。遇到这次机会肯定不能放过。立马写好简历发给清华大学的陈渝老师，寄出投名状。  

### 事件2：阅读活动相关内容，要求
投递简历后，阅读github上活动主页的信息，觉得任务十分艰巨。我们学校操作系统原理，计算机组成原理，编译原理这三门课全都在大三上才开，而我刚刚大二学年结束。Rust语言，RISC-V体系结构也没接触过。时间紧迫，马上开始学习。  

### 事件3：着手开始学习Rust语言
查阅多方资料后，开始阅读《The Rust Programming Language》进行Rust语言的学习。  
[The Rust Programming Language](https://doc.rust-lang.org/book/)  
上面的链接很清楚完整地说明了rustup在Linux系统下的安装，下面这个链接说明了如何更rustup改为国内镜像源：  
[rust国内源](https://www.jianshu.com/p/cf1b534dbb16)  
按照文档《The Rust Programming Language》基于rust实现了一个猜数字小游戏，并通过这个简单的例子对rust语言有了初步的了解。  

### 问题
+ rustup在Linux系统上如何安装
+ 如何使用cargo管理rust项目和代码
+ 如何将rustup换成国内源

<span id="Day001"></span>

## Day 1 （2020-07-01）

### 预期计划
+ 完成《The Rust Programming Language》中前十个章节的学习
+ 动手实现文档中的实例代码并理解

### 事件1：继续学习rust语言
阅读文档[The Rust Programming Language](https://doc.rust-lang.org/book/) ，完成了以下模块的学习：  
+ rust中的常见编程概念
+ 认识rust中的所有权
+ 使用rust结构体
+ 枚举与模式匹配
+ 使用包，Crate和模块管理项目
+ rust中的常见集合vector，字符串和哈希map
+ 错误处理
+ 泛型，trait和生命周期

并动手实现了上面某些模块的示例代码。  

### 问题
+ 对于以上模块的学习，有些是一知半解，一些细节比较难掌握
+ rust语言和我之前学的C或C++之间有不少不同点
+ 泛型和trait这部分的内容难以理解


<span id="Day002"></span>
## Day 2 （2020-07-02）
### 预期计划
+ 完成《The Rust Programming Language》中第11,12章节的学习
+ 学习《Rust编程之道》第2,3,4,5章，重点巩固一下之前半知半解的内容
+ 着手开始做一些Rust编程小练习 

### 事件1：学习如何编写Rust自动化测试
Rust 是一个相当注重正确性的编程语言，不过正确性是一个难以证明的复杂主题。Rust 的类型系统在此问题上下了很大的功夫，不过它不可能捕获所有种类的错误。为此，Rust 也在语言本身包含了编写软件测试的支持。  
Rust为测试提供了许多注解和宏，这让我们可以很方便直观地进行软件功能测试以及Debug。  
首先学习了如何编写测试，然后动手实现一些测试代码，并理解测试的组织结构。  

### 事件2：完成一个I/O项目：构建一个命令行程序
基于Rust语言实现Linux系统下的经典命令行工具grep。  
grep最简单的使用场景是在特定文件中搜索指定字符串。为此，grep获取一个文件名和一个字符串作为参数，接着读取文件并找到其中包含字符串参数的行，然后打印出这些行。  
学习如何让命令行工具利用很多命令行工具中用到的终端功能。读取环境变量来使得用户可以配置工具的行为。打印到标准错误控制流（stderr） 而不是标准输出（stdout），这样用户可以选择将成功输出重定向到文件中的同时仍然在屏幕上显示错误信息。  
[minigrep](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Rust_Learning_Code/minigrep/src)

### 事件3：完成《Rust编程之道》第十章的完整实例代码，掌握Cargo和模块系统
编写一个命令行工具，可以接收一个CSV文件，并且可以指定固定的值来覆盖指定列的所有数据，然后将结果输出到新的CSV文件中。  
通过编写这个实例代码，从零开始实现一个完整功能包，掌握如何使用Cargo管理包和模块系统。  
同时也是对前面的理论学习的温故知新。  
[csv_challenge](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Rust_Learning_Code/csv_challenge/src)

### 事件4：学习迭代器与闭包
Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。可以在一个地方创建闭包，然后在不同的上下文中执行闭包运算。不同于函数，闭包允许捕获调用者作用域中的值。  
迭代器模式允许你对一个项的序列进行某些处理。迭代器（iterator）负责遍历序列中的每一项和决定序列何时结束的逻辑。当使用迭代器时，我们无需重新实现这些逻辑。  
闭包和迭代器概念在Rust语言中使用特别多，需要完全掌握。  

### 事件5：阅读《Rust编程之道》第2,3,4,5章
浏览《Rust编程之道》第2,3,4,5章，回顾之前的知识，并对梳理一遍之前半知半解的内容。  
重点理解泛型，trait，生命周期参数。  

### 事件6：着手开始做一些Rust小练习题
练习题链接：[exercises](https://github.com/rust-lang/rustlings)  
完成了以下练习题：  
+ variables
+ primitive_types
+ functions
+ if
+ struct

<span id="Day003"></span>
## Day 3 （2020-07-03）
### 预期计划
+ 继续完成一些Rust练习题
+ 学习《The Rust Programming Language》14，15章的内容
+ 查阅一些资料，初步了解一下RISC-V系统结构

### 事件1：完成一部分Rust小练习题
练习题链接：[exercises](https://github.com/rust-lang/rustlings)  
完成了以下练习题：  
+ test1.rs
+ strings
+ test2.rs
+ enums
+ tests
+ test3.rs
+ modules
+ macros
+ test4.rs

### 事件2：观看浙大《计算机组成与设计：RISV-V》慕课
观看浙大MOOC，学习了以下部分内容：  
+ 算术指令
+ 访存指令
+ 条件判断分支转移指令
+ 逻辑运算指令
+ 函数调用
+ 栈的使用
+ R,I,S,B,U,J型指令的机器码表示
+ 数据通路的实现
+ 控制器的实现

### 事件3： 继续完成一部分Rust小练习题
练习题链接：[exercises](https://github.com/rust-lang/rustlings)  
完成了以下练习题：  
+ move_semantics
+ error_hadling
+ options
+ clippys

### 事件4：学习更多关于Cargo和Crates.io的内容
+ 使用发布配置来自定义构建
+ 将库发布到Crates.io
+ 使用工作空间来构建更大的项目
+ 从Ctates.io安装二进制文件
+ 使用自定义的命令来扩展Cargo

### 事件5：学习智能指针
+ Box<T>，用于在堆上分配值
+  Rc<T>，一个引用计数类型，其数据可以有多个所有者
+  Ref<T>和RefMut<T>，通过RefCell<T>访问，一个在运行时而不是在编译时执行借用规则的类型。  

### 事件6：了解一下用Rust写OS的相关信息
+ 观看视频：《半个世纪过去了，是时候用Rust重写操作系统了吗》
+ 阅读PPT：《尝试用Rust写教学操作系统》
+ 阅读文章：《使用Rust编写操作系统》

<span id="Day004"></span>

## Day 4 （2020-07-04）
## 预期计划
+ 学习完《The Rust Programming Language》剩下章节的内容
+ 进一步学习RISC-V架构的知识
+ 继续完成一部分Rust小练习题
+ 看清华大学慕课回顾一下操作系统原理的知识
+ 完成rCore-Tutorial V3的环境配置，并开始Lab0

### 事件1：继续完成一部分Rust小练习题
练习题链接：[exercises](https://github.com/rust-lang/rustlings)  
完成了以下练习题：  
+ generics
+ traits
+ standard_library_types

### 事件2：学习Rust的并发编程
阅读《The Rust Programming Language》第十六章内容
+ 如何创建线程来同时运行多段代码
+ 消息传递并发，其中通道被用来在线程间传递消息
+ 共享状态并发，其中多个线程可以访问同一片区域
+ Sync和Send trait，将Rust的并发保证扩展到用户定义的以及标准库提供的类型中

### 事件3：完成全部Rust小练习题
练习题链接：[exercises](https://github.com/rust-lang/rustlings)  
完成了以下练习题：  
+ threads
+ conversions

![finish_rustlings](./img/finish_rustlings.png)

到此Rust小练习题全部完成！

### 事件4：阅读《RISC-V-Reader-Chinese》，进一步学习RISC-V架构的知识
+ 学习RV32F和RV32D
+ 学习RV32/64特权架构

### 事件5：观看清华大学操作系统慕课
+ 什么是操作系统
+ 从OS角度看操作系统

### 事件6：学习完剩下的Rust语言知识
+ Rust的面向对象编程特性
+ 模式匹配
+ Rust高级特征

### 事件7：实现Rust项目：构建多线程web server
[web_server](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Rust_Learning_Code/hello)  

![web_server](./img/web_server.png)

在完成项目的过程中梳理一遍Rust语法。  

### 事件8：开始Lab0
参考文章：[writing-an-os-in-rust](https://github.com/rustcc/writing-an-os-in-rust/blob/master/01-freestanding-rust-binary.md)  
和实验指导：[lab0](https://rcore-os.github.io/rCore-Tutorial-deploy/docs/lab-0/guide/intro.html)


<span id="Day005"></span>

## Day 5 （2020-07-05）
### 预期计划
+ 继续完成Lab0
+ 继续学习一部分操作系统的知识
+ 开始用Rust语言重新实现15道以上的编程练习题
+ 阅读RISC-V特权指令规范
+ 阅读论文《Rust语言操作系统的设计与实现》

### 事件1：继续完成Lab0
源码：  
[Lab0](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/rCore_Labs/Lab0/os)  
报告：  
[Lab0_Report](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/rCore_Labs/Lab0/Report)

### 事件2：加深对unsafe Rust的理解
不安全的超级力量：  
+ 解引用裸指针
+ 调用不安全的函数或方法
+ 访问和修改可变静态变量
+ 实现不安全trait
+ 访问`union`的字段

### 事件3：继续观看清华大学mooc学习操作系统知识
+ 中断，异常和系统调用的基本概念和原理
+ 硬件架构支持

今天由于完成Lab0花费的时间超出预期，几乎花了一整天，因此没能顺利完成预期计划，明天继续加油。  

<span id="Day006"></span>

## Day 6 （2020-07-06）
### 预期计划
+ 用Rust语言重新实现14道编程练习题
+ 阅读RISC-V特权指令规范
+ 阅读论文《Rust语言操作系统的设计与实现》
+ 继续学习操作系统相关内容

### 事件1：用Rust语言实现编程练习题
实现5道《笨方法学C》中的练习题：（这些都比较简单）  
+ [Arrays and Strings,9](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Learn_Rust_The_Hard_Way/arrays_and_strings)
+ [Array of Strings and Looping,10](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Learn_Rust_The_Hard_Way/arrays_of_strings_and_looping)
+ [Switch,13](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Learn_Rust_The_Hard_Way/switch_statement)
+ [Writing And Using Functions,14](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Learn_Rust_The_Hard_Way/writing_and_using_functions)
+ [Structs And Pointers To Them,16](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Learn_Rust_The_Hard_Way/struct_and_pointers_to_them)  

实现单链表：  [linked list](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Learn_Rust_The_Hard_Way/linked_list)  
实现双链表： [double linked list](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Learn_Rust_The_Hard_Way/double_linked_list)  
实现栈：  [stack](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Learn_Rust_The_Hard_Way/stack)  
实现一些排序算法：  [sort](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Learn_Rust_The_Hard_Way/sort)  
+ 冒泡排序
+ 选择排序
+ 插入排序
+ 希尔排序

用Rust做一些LeetCode题目：  [leetcode for rust](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/LeetCode_Rust)

今天一天都在编程，通过实践才发现Rust的语法和我之前学的语言有很大不同，今后需要多在代码实战中提升对Rust语言的掌握和理解。  
今天又是没能顺利完成预期计划，明天继续加油。  

<span id="Day007"></span>

## Day 7 （2020-07-07）
### 预期计划
+ 用Rust语言完成三个命令行工具
+ 阅读RISC-V特权指令规范
+ 阅读论文《Rust语言操作系统的设计与实现》
+ 继续学习操作系统相关内容

### 事件1：用Rust语言完成三个命令行工具
+ 实现minicat命令行工具（类似于Linux上的cat）[minicat](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Shell_Tools/minicat)
+ 实现miniecho命令行工具（类似于Linux上的echo）[minicopy](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Shell_Tools/miniecho)
+ 实现minicopy命令行工具（类似于Linux上的cp）[minicopy](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/Shell_Tools/minicopy)

到现在Rust语言的编程练习就告一段落，开始正式进入操作系统阶段。Rust语言的一些细节在做实验的过程中慢慢领悟和熟悉。  

### 事件2：阅读RISC-V特权指令规范
+ 理解RISC-V Privileged Software Stack Terminology
+ 了解Control and Status Registers (CSRs)

### 事件3：阅读论文《Rust语言操作系统的设计与实现》
阅读论文《Rust语言操作系统的设计与实现》，了解学长是怎样用Rust语言写操作系统的。  

### 事件4：阅读《writing-an-os-in-rust》
编写一个最小化内核。  

<span id="Day008"></span>

## Day 8 （2020-07-08）

### 预期计划
+ 继续阅读RISC-V特权指令规范
+ 继续阅读论文《Rust语言操作系统的设计与实现》
+ 阅读《writing-an-os-in-rust》

### 事件1：阅读RISC-V特权指令规范
笔记：[RISCV_note](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/blob/master/memo/RISCV_note.md)  

### 事件2：阅读论文《Rust语言操作系统的设计与实现》
看完十分佩服王润基同学，做了如此庞大且困难的工作。  
### 事件3：阅读《writing-an-os-in-rust》

### 事件4：观看清华大学操作系统线上课程
+ 系统调用

今天基本上看了一天的RISC-V英文文档，对特权指令有了更深一步的了解，通过后面做实验的过程继续进一步学习。  

<span id="Day009"></span>

## Day 9 （2020-07-09）

### 预期计划
+ 完成Lab1大部分内容
+ 继续观看清华大学操作系统线上课程

### 事件1：开始Lab1
+ 先看Lab0实验报告复习一下Lab0
+ 阅读实验指导
+ 提出了一个问题发布到issues上已经得到解决
+ 阅读开发规范
+ 阅读实验源码
+ 写实验学习报告
+ 尝试完成中断嵌套调用的功能

基本上弄懂了Lab1,对思考部分进行了思考，还有一些细节问题等吃完饭回来写实验报告的的时候进行梳理。  
### 事件2：写Lab1学习报告
[Lab1-report](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/blob/master/rCore_Labs/Lab1/Report/lab_1_report.md)  

### 事件3：观看清华大学操作系统线上课程

今天主要是仔细分析了Lab1代码和相关的中断处理过程，明天继续完善报告。  

<span id="Day010"></span>

## Day 10 （2020-07-10）

### 预期计划
+ 继续完善Lab1学习报告

### 事件1：继续完善Lab1学习报告
[Lab1-report](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/blob/master/rCore_Labs/Lab1/Report/lab_1_report.md)  

### 事件2：动手实现一遍Lab1
项目代码：[lab1-interrupt](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/rCore_Labs/Lab1/os)  

明天对Lab1进行改进。  

<span id="Day011"></span>

## Day 11 （2020-07-11）

### 预期计划
+ 改进Lab1
+ 完成Lab1学习报告

### 事件1：改进Lab1
+ 增加对系统调用，外部中断的处理
+ 添加中断嵌套的测试
+ 添加中断向量表/中断描述符
项目代码：[lab1-interrupt](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/rCore_Labs/Lab1/os)  

### 事件2：完成Lab1学习报告
[Lab1-report](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/blob/master/rCore_Labs/Lab1/Report/lab_1_report.md)  
### 事件3：观看清华大学操作系统线上课程，为Lab2做知识储备
+ 连续内存分配

<span id="Day012"></span>

## Day 12 （2020-07-12）

### 预期计划
+ 继续观看清华大学线上课程
+ 补充基础知识，为Lab2做准备

### 事件1：观看清华大学操作系统线上课程

### 事件2：浏览一下Lab2实验指导
这一章的实验指导中，你将会学到：  
+ 实现动态内存的分配
+ 了解 QEMU 模拟的 RISC-V Virt 计算机的物理内存
+ 通过页的方式对物理内存进行管理

这天准备在13号的考试，因此进度停滞了一点。

<span id="Day013"></span>

## Day 13 （2020-07-13）

### 预期计划
+ 看完清华大学操作系统课程内存这块的内容
+ 准备Lab2

### 事件1：观看清华大学操作系统线上课程
+ 段式存储管理
+ 虚拟存储管理

### 事件2：开始Lab2
+ 动态内存分配
+ 物理内存探测

<span id="Day014"></span>

## Day 14 （2020-07-14）

### 预期计划
+ 完成Lab2
+ 写Lab2学习报告

### 事件1：完成Lab2
[Lab2](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/rCore_Labs/Lab2/os)  
本章完成了动态分配内存的管理和物理内存的管理，我们通过划分出一段静态内存为操作系统实现了动态内存的分配；通过页的管理模式，实现了物理页的分配器。  
本章还只是物理内存的管理，后面为了进一步支持多线程的内存管理，我们将在下一章实现内存的虚拟化。  
### 事件2：写Lab2学习报告
Lab2 中模块之间的调用比较频繁，需要写的地方有点多。  
<span id="Day015"></span>

## Day 15 （2020-07-15）

### 预期计划
+ 完成Lab3
+ 思考如何改进Lab2和Lab3

### 事件1：完成Lab3
项目地址：[Lab3](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/rCore_Labs/Lab3/os)  
回顾本章，我们理清了虚拟地址和物理地址的概念和关系；并利用页表完成虚拟地址到物理地址的映射；最后实现了内核空间段的重映射。  
如果说本章和前一个章节是对空间的划分和管理，那么在下一个小节中，我们将实现对时间的划分和管理，也就是线程。  
### 准备改进Lab2和Lab3
想了一会，觉得内存分配算法这里有改进的空间。  

.bss 字段是什么含义？为什么我们要将动态分配的内存（堆）空间放在 .bss 段？  
> .bss 字段一般包含全局变量的名称和长度，在执行时由操作系统分配空间并初始化为 0 。
> 我们不是必须把内存空间放在 .bss 段，我们可以任意指定一段可以访问的内存空间。只不过这样做比较容易实现。



<span id="Day016"></span>

## Day 16 （2020-07-16）

### 预期计划
+ 改进Lab2和Lab3
+ 准备Lab4

### 事件1：改进Lab2和Lab3
+ 实现一个分配器
+ 思考Lab2和Lab3不足之处
项目地址：[Lab3](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/rCore_Labs/Lab3/os)  

我们在动态内存分配中实现了一个堆，它允许我们在内核代码中使用动态分配的内存，例如` Vec `，` Box `等等，如果我们在实现这个堆的过程中使用` Vec `而不是` [u8] `，会出现什么结果？  
> 程序陷入循环。  
> 

### 事件2：准备Lab4
这一章的实验指导中，你将会学到：  
+ 线程和进程的概念以及运行状态的表示
+ 线程的切换
+ 对 CPU 进行抽象在上面完成对线程的调度


<span id="Day017"></span>

## Day 17 （2020-07-17）

### 预期计划
+ 观看清华大学操作系统线上课程进程部分的内容
+ 开始Lab4

### 事件1：观看清华大学操作系统线上课程进程部分的内容
+ 进程和线程
+ 进程和线程控制
+ 处理机调度
+ 多处理机调度
+ 同步互斥
+ 信号量和管程
+ 死锁和并发错误检测

### 事件2：开始Lab4
+ 进程和线程
+ 线程的创建
+ 线程的切换
+ 线程的结束
+ 内核栈
+ 线程调度
<span id="Day018"></span>

## Day 18 （2020-07-18）
### 预期计划
+ 完成Lab4
+ 实现新的调度算法
### 事件1：完成Lab4
项目地址：[Lab4](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/rCore_Labs/Lab4/os)  
本章我们的工作有：  
+ 理清线程和进程的概念
+ 通过设置 Context，可以构造一个线程的初始状态
+ 通过 __restore 标签，直接进入第一个线程之中
+ 用 Context 来保存进程的状态，从而实现在时钟中断时切换线程
+ 实现内核栈，提供安全的中断处理空间
+ 实现调度器，完成线程的调度

<span id="Day019"></span>

## Day 19 （2020-07-19）
### 预期计划
+ 完成新的连续内存分配
+ 完成新的调度算法

### 事件1：新的内存分配算法
实现伙伴系统内存分配算法，出现许多 bug 。  
在线程切换之中，页表是如何切换的？页表的切换会不会影响程序/操作系统的运行？为什么？  
> 页表是调用` Thread::prepare() `的时候切换的。  
> 它不会影响执行，因为在中断期间是操作系统正在执行，而操作系统所用到的内核线性映射是存在于每个页表中的。  
> 

### 事件2：新的调度算法
stride scheduling 算法。  

今天一直在调bug

<span id="Day020"></span>

## Day 20 （2020-07-20）

### 预期计划
+ 修好bug
+ 继续完成新的调度算法

### 事件1：修好bug
实现了基于伙伴系统的 `VectorAllocator` ，重复写了5个版本，终于写好了一个比较满意的。  
源码地址：[buddy_system](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/blob/master/rCore_Labs/Lab4/os/src/algorithm/src/allocator/buddy_system_vector_allocator.rs)  

### 事件2：学习K210开发板
买了个 k210 开发板 Maixpy Go，准备搞事情。  
<span id="Day021"></span>

## Day 21 （2020-07-21）
### 预期计划
+ 完成Lab3和Lab4实验题
+ 继续学习k210开发板

### 事件1：完成Lab3和Lab4实验题
实现`Stride Scheduler`
为什么` Mapping `中的` page_tables `和` mapped_pairs `都保存了一些` FrameTracker `？两者有何不同？  
> 页表也是需要分配页面来存储的，因此，` page_tables `存放了所有页表所用到的页面，而` mapped_pairs `则存放了进程所用到的页面。  
> 

### 事件2：完成k210开发板的开发环境配置
准备实验一下吴一凡学长写的支持k210的Lab0

<span id="Day022"></span>

## Day 22 （2020-07-22）
### 预期计划
+ 学习Lab5和Lab6
+ 开始学习一下 zCore

### 事件1：学习Lab5和Lab6
动手复现了Lab5的代码。  
感觉 Lab5 这部分的内容我在 OS 原理课上掌握的不是很好，特别是设备树和 virtio 协议，需要回头补一下。  
### 事件2：阅读zCore文档
感觉有点难懂。  
<span id="Day023"></span>

## Day 23 （2020-07-23）
### 预期计划
+ 完成Lab6的学习
+ 继续阅读 zCore 的文档


使用条件变量之后，分别从线程和操作系统的角度而言读取字符的系统调用是阻塞的还是非阻塞的？  
> 对线程而言是阻塞的，对操作系统而言是非阻塞的。  
> 

### 事件1：完成Lab6
动手复现了Lab6的代码

### 事件2：继续阅读zCore的文档
zCore 文档阅读就先到这，先完成这个月的任务。  

### 事件3：在 k210 板子上跑通 Lab0 和 Lab1
<span id="Day024"></span>

## Day 24 （2020-07-24）
### 预期计划
+ 修改Lab4

### 事件1：修改Lab4
因为我做Lab4的时候助教学长还没更新Lab4的代码，因此要回头修改一下实现。  

<span id="Day025"></span>

## Day 25 （2020-07-25）
### 预期计划
+ 完成Lab6的实验题


### 事件1：完成Lab6的实验题
做了一天才写好三个系统调用，感觉学长们已经给系统调用的框架搭建好了，实现起来思路比较清晰。  

<span id="Day026"></span>

## Day 26 （2020-07-26）
### 预期计划
+ 整理报告
+ 开会

### 事件1：整理报告
攒下来的报告有点多，今天花时间整理。  

### 事件2：开会
开会时陈老师讲了第二阶段的注意事项。  
不知道能不能通过老师和助教们的审核，有点紧张。  

<span id="Day027"></span>

## Day 27 （2020-07-27）
### 预期计划
整理报告  

今天一天就是在整理之前做的实验的报告和总结报告，突然发现我写报告的文采还是可以的。明天就要出结果了，很是紧张。  

<span id="Day028"></span>

## Day 28 （2020-07-28）
晚上收到了通知，自己通过了 review ，心里特别高兴，终于可以告别家里蹲生活了。  

<span id="Day029"></span>

## Day 29 （2020-07-29）

<span id="Day029"></span>

## Day 29 （2020-07-29）

<span id="Day030"></span>

## Day 30 （2020-07-30）

<span id="Day031"></span>

## Day 31 （2020-07-31）

<span id="Day032"></span>

## Day 32 （2020-08-01）
这几天去广州大哥家住，参观了大哥的公司，文远知行，带亲戚家的小孩子去广州购书中心玩，等等。  

<span id="Day033"></span>

## Day 33 （2020-08-02）
动身去深圳，到酒店后带亲戚家的孩子去深圳南方书城玩。  

<span id="Day034"></span>

## Day 34 （2020-08-03）
线下实习第一天  
早上 9 点 到 11 点，向勇老师和王润基学长讲话。主要讲了 rCore 的开发历史和选题介绍。  
特别是向勇老师的讲话，讲得十分精彩，全程都把我吸引住了，听了受益匪浅。  
会议结束后，在 1 楼合了影，并且在陈睿老师的带领下参观了实验室，看到了很多前沿的科技成果。  
下午 2 点到 5 点半，我们在会议室里进行讨论，每个同学都上台讲了自己在这个活动中感兴趣的题目，选这个题目的理由，和想要达成的这个目标。  
我选择了 zCore 到 k210 开发板的移植，这个项目做起来需要的时间比较庞大，因此我打算在这个月的实习结束后依然继续进行这个项目，同时做好项目开发记录，以便这个项目进行不下去了，方便后人接手。  
还有很多同学提出了一些很有意思的东西，比如在 sel4 上跑 zCore和实现用 Rust 写的 SBI，等等。  
晚餐我们有个聚餐，餐桌上听向勇老师和清华的研究生学长们谈论清华计算机系的种种趣事，发现清华计算机系不只是充满了能力超强的学生，还有许多新鲜事。在一旁听着他们的谈论，一边感叹着我是不是在做梦，我正在和清华计算机系的一群人共事，一边想着要是当初我能考上清华计算机系有多好，那样就可以和这些有趣的灵魂朝夕相处了。  
晚上回到酒店i休息一下就睡觉了，期待着明天国科大老师的报告。  

<span id="Day035"></span>

## Day 35 （2020-08-04）
线下实习第二天  
早上 9 点到 11 点，我和 k210 小组的成员一起进行了简单的讨论。讨论的结果是我先跟着文档组一起学习 zCore ，然后再着手 zCore 到 k210 的移植工作。  
下午国科大“一生一芯”团队，余子濠博士和 5 位国科大的16级同学向我们分享了他们“一生一芯”项目过程中的经历和一些遇到的 Bug，然后还有感想。  
成果展示的时候的确被震撼到了，在自己写的 RISCV CPU 上跑起 Linux，简直太酷了！  
在他们分享遇到的困难和 Bug 的时候，我明白了他们付出的努力和一路走来的不容易。  
这次分享拓展了我的知识视野，能见到传说中的“一生一芯”团队真容，我感到十分荣幸。  


<span id="Day036"></span>

## Day 36 （2020-08-05）
线下实习第三天  
早上 9 点到 11 点半，向勇老师找我们 k210 组谈话，我们在清华操作系统教学网页上注册，录入了清华操作系统专题训练实验名单。向勇老师让我们每个人说说对自己这一个月能达成的目标的期望，并对此作出评价。向勇老师结合我的情况，决定把我调到 rCore 到 zCore 移植的小组中去。经过和老师协商，我目前先是学习一下 zCore，给 zCore 的代码加点注释，熟悉之后一步步把 zCore 在 RISCV 架构下的 qemu 中跑起来，然后再一步步移植到 k210 板子上去。  
下午 2 点到 5 点半，我们去参观了清华伯克利深圳研究院，听了谭章熹博士关于 RISCV 的学术报告，了解了他们正在做的项目 RIOS，是一个十分宏伟的项目。听完报告后肃然起敬。昨天的国科大“一生一芯”的项目就已经足够让我震惊了，这个项目更上一层楼。听内部人员说想要进入清华伯克利深圳研究院的最低要求是进入清华大学，这就足以说明这个研究院非常牛逼，看起来比鹏城实验室还要牛逼不少，我走在里面就像是在拜访圣地。  
晚上是开发 Maixpy 的工程师的讲座，向我们介绍了 Maixpy 的介绍和使用。

<span id="Day037"></span>

## Day 37 （2020-08-06）
线下实习第四天  
早上再经过和向勇老师讨论，最终定下了这个月的阶段性目标：  
目标：熟悉 zCore，将 zCore 的一部分在 RISCV 架构上的 QEMU 上运行。  
目前大概思路：zCore 中与硬件交互的模块是 HAL 硬件抽象层，将 zCore 移植到 RISCV 上需要将这部分进行修改使其兼容 RISCV 架构。目前的大概思路是先从 HAL 中的一部分 API 着手，然后再一步步增加对 HAL 中的 API 的 RISCV 下的重构。最终达到能在 RISCV 架构下的 QEMU 中运行 zCore 的核心部分。  
预测会遇到的困难：  
+ 需要时间学习 zCore
+ 对于一些基础的系统知识，需要补缺
+ 需要熟悉 QEMU
+ Debug 会比较困难

能获取到的帮助：  
+ 潘庆霖学长的毕设论文
+ 王润基学长和吴一凡学长在鹏城实验室的报告
+ zirkon 的官方文档和 RISCV 的官方文档
+ rCore 项目源码


下午是华为 OpenEuler 操作系统的架构师的分享。  
晚上是贾越凯学长关于 zCore 虚拟化的分享和王润基学长关于 zCore 内核对象和系统调用，zCore 硬件移植和驱动开发的分享。（讲得特别好）  

<span id="Day038"></span>

## Day 38 （2020-08-07）
线下实习第五天，也是最后一天  
早上 9 点到 11 点半是同学们为期 5 天的线下实习的总结报告，每个同学上台讲一下这个月要达成的阶段性目标，并且说一下预期会遇到的困难和实现的可能性。  
我也是按照昨天和向勇老师，还有王润基学长讨论的那样，比较紧张地讲了一下，具体的内容都昨天的日报。  
下午是早上没讲的同学继续讲。  
晚上是肖络元工程师介绍 zCore 应用开发。  

<span id="Day039"></span>

## Day 39 （2020-08-08）
今天从深圳做高铁回到广州大哥的家住几天。  
晚上根据 zCore-Tutorial 重现了内核对象的代码，加深了对内核对象的理解。  
下面定义内核对象的 trait ：  
```Rust
/// trait for kernel object
pub trait KernelObject: DowncastSync + Debug {
    /// get id of kernel object
    fn id(&self) -> KoID;
    /// get type of kernel object
    fn type_name(&self) -> &str;
    /// get name of kernel object
    fn name(&self) -> String;
    /// set name of kernel object
    fn set_name(&self, name: &str);
}
```  
下面编写一个宏自动为内核对象实现`KernelObject` trait：  
```Rust
#[macro_export]
macro_rules! impl_kobject {
    ($class:ident $( $fn:tt )*) => {
        // implement `KernelObject` trait for object
        impl KernelObject for $class {
            fn id(&self) -> KoID {
                self.base.id
            }
            fn type_name(&self) -> &str {
                stringify!($class)
            }
            fn name(&self) -> alloc::string::String {
                self.base.name()
            }
            fn set_name(&self, name: &str) {
                self.base.set_name(name)
            }
            $( $fn )*
        }
        impl core::fmt::Debug for $class {
            fn fmt(
                &self,
                f: &mut core::fmt::Formatter<'_>,
            ) -> core::result::Result<(),core::fmt::Error> {
                f.debug_tuple(&stringify!($class))
                    .field(&self.id())
                    .field(&self.name())
                    .finish()
            }
        }
    };
}
```
我们还实现了接口到具体类型的向下转换，并为上述逻辑写了单元测试。  
