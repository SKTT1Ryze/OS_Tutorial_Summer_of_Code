# rCore Tutorial Lab 学习报告

## **TOC**
* [Lab0](#lab0)  
* [Lab1](#lab1)  
* [Lab2](#lab2)  
* [Lab3](#lab3)  
* [Lab4](#lab4)  
* [Lab5](#lab5)  
* [Lab6](#lab6)  

<span id="lab0"></span>

## Lab0

<span id="lab1"></span>
## Lab1
### 引言
本文是本人在详细阅读` rCore-Tutorial Lab1 `的实验指导，并仔细分析了` rCore `中` interrupt `部分的代码之后，结合` RISC-V `特权指令规范文档，按照实验指导中文档格式规范编写的学习报告，对` RISC-V `架构下中断处理机制做了一遍梳理，并结合代码来分析` rCore `在中断机制这个模块中是怎么实现的。另外，本人对实验指导和实验源码中提出的几个思考作出了自己的看法，并提出了对源码中某处实现方式合理性的疑问和改进方法。最后，本人尝试在现有代码基础上，为` rCore `实现中断嵌套调用的处理机制，包括提出实现思路和尝试修改代码实现。  
本次实验学习报告将紧密结合代码来进行对中断处理机制的梳理，中间穿插` RISC-V `架构知识，目的是通过实践代码来直观地理解操作系统是如何处理中断机制的。  

### 什么是中断
首先来简单地了解一下什么是中断。  
中断这个概念在很多教科书，网站上都有或相同或不同的介绍，下面是本人觉得比较准确的一个说法：  
**中断是一种使 CPU 中止正在执行的程序而转去处理特殊事件的操作，这些引起中断的事件称为中断源，它们可能是来自外设的输入输出请求，也可能是计算机的一些异常事故或其他原因**  
此概念引用自清华大学出版的《80x86汇编语言程序设计》一书。  
中断有以下三种：  
+ 异常（Exception）：指令产生的，通常无法预料的错误。例如：访问无效地址，除零操作；
+ 陷阱（Trap）：一系列强行导致中断的指令，例如：系统调用；
+ 硬件中断（Hardware Interrupt）：由 CPU 之外的硬件产生的异步中断，例如：时钟中断。  

中断的作用：  
+ 处理 CPU 某些错误；
+ 提供程序调试功能（断点中断和单步中断）；
+ 与外部设备进行 I/O 通信。  

### 中断流程
+ 中断源产生中断
+ 获取中断入口
+ 进入中断入口
+ 保存当前上下文
+ 进入中断处理程序
+ 处理中断
+ 中断返回
+ 恢复上下文
+ 继续执行程序

在Linux等成熟的操作系统中，中断机制还要更为复杂，比如在Linux安全模式下的中断是通过中断描述符来定位中断处理程序。不过大体上的流程是一样的。在这里只是根据` rCore `的代码实现来分析中断处理过程。  
在分析中断过程之前，还需要补充几个基础概念。  

### 上下文（Context）
上下文可以理解为当前系统的寄存器状态，在进入中断处理程序前需要保存当前上下文。
在` rCore `中，上下文使用一个数据结构来抽象：  
` os/src/interrupt/context.rs `  
```Rust
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Context {
    /// 通用寄存器
    pub x: [usize; 32],
    /// 保存诸多状态位的特权态寄存器
    pub sstatus: Sstatus,
    /// 保存中断地址的特权态寄存器
    pub sepc: usize,
}
```
可以看到这个` context `数据结构保存了 32 个通用寄存器，` sstatus `特权态寄存器和` sepc `特权态寄存器。  
这里我们不将` scause `和` stval `寄存器放在` Context `中，至于为什么这么做本人的猜测会在后面提到，这也将结合到其中一个思考题来综合考虑。  

### 特权级（Privilege Levels）
在` RISC-V `架构中，目前定义了三个特权级：  
+ Machine (M)
+ Supervisor (S)
+ User (U)

其中 Machine 特权级级别最高，Supervisor 特权级其次，User 特权级最低。  
特权级用来为 software 的不同部分提供保护，尝试进行当前特权级不允许的操作将会引起异常。  
更多关于` RISC-V `架构的详细内容请查阅[RISC-V特权指令规范](https://riscv.org/specifications/privileged-isa/)  

### 特权级寄存器
这里集中梳理一遍在中断处理中主要涉及到的几个 S 特权级寄存器，即` Supervisor CRSs `。  
#### Supervisor Trap Vector Base Address Register (stvec)
在官方文档中对` stvec `的描述：  
> The stvec register is an SXLEN-bit read/write register that holds trap vector configuration, consisting of a vector base address (BASE) and a vector mode (MODE).  

结合下面这幅图来理解：  
![stvec](/home/hustccc/OS_Tutorial_Summer_of_Code/rCore_Labs/Lab1/Report/img/stvec.png)  
` stvec `寄存器是保存发生异常时 CPU 需要跳转到的地址。其中 BASE 字段保存着有效的虚拟地址或物理地址，这个地址必须四字节对齐。MODE 字段将会决定寻址方式。  

![MODE](/home/hustccc/OS_Tutorial_Summer_of_Code/rCore_Labs/Lab1/Report/img/stvec_way.png)  
也就是说，MODE 字段为 Direct（0）的话，BASE 字段直接指向需要跳转的地址；若 MODE 字段为 Vectored 的话，BASE + 4 × cause 指向需要跳转的地址。  

#### Supervisor Exception Program Counter （sepc）
在官方文档中对` sepc `的描述：  
> sepc is a WARL register that must be able to hold all valid physical and virtual addresses. It
need not be capable of holding all possible invalid addresses. Implementations may convert some
invalid address patterns into other invalid addresses prior to writing them to sepc.   
> When a trap is taken into S-mode, sepc is written with the virtual address of the instruction
that was interrupted or that encountered the exception. Otherwise, sepc is never written by the
implementation, though it may be explicitly written by software.  
在发生异常时，` sepc `将会保存发生异常的指令的地址。  

#### Supervisor Status Register (sstatus)
在官方文档中对` sstatus `的描述：  
> The sstatus register is an SXLEN-bit read/write register formatted as shown in Figure 4.1 for
RV32 and Figure 4.2 for RV64. The sstatus register keeps track of the processor’s current operating
state.  

结合下面这幅图来理解：  

![sstatus](/home/hustccc/OS_Tutorial_Summer_of_Code/rCore_Labs/Lab1/Report/img/sstatus.png)  
` sstatus `是` supervisor `模式下的状态寄存器，它保存着全局中断使能，以及许多其他状态。  
需要注意的一点是，CPU 在 S 模式下运行时，只有在全局中断使能位 sstatus.SIE 置 1 时才会产生中断。每个中断在控制状态寄存器` sie `中都有自己的使能位，位置对应于一个中断代码。  

####  Supervisor Interrupt Registers (sip and sie)
分别简单说明一下这两个特权级寄存器：  
+ ` sie `指出 CPU 目前能处理和必须忽略的中断；
+ ` sip `列出目前正准备处理的中断。

将上面三个控制状态寄存器合在一起考虑，如果 sstatus.SIE = 1, sie[7] = 1，且 sip[7] = 1，则可以处理机器的时钟中断。  

#### Supervisor Cause Register (scause)
在官方文档中对` scause `的描述：  
> The scause register is an SXLEN-bit read-write register formatted as shown in Figure 4.9. When a trap is taken into S-mode, scause is written with a code indicating the event that caused the trap.
Otherwise, scause is never written by the implementation, though it may be explicitly written by
software.  
> The Interrupt bit in the scause register is set if the trap was caused by an interrupt. The Exception Code field contains a code identifying the last exception. Table 4.2 lists the possible exception codes for the current supervisor ISAs. The Exception Code is a WLRL field, so is only guaranteed to hold supported exception codes.  

也就是说` scause `指示发生异常的种类，Interrupt 字段置 1 表示这个` trap `是中断引起的。Exception Code 字段将发生异常的原因更细地分类。更多内容请查阅文档[RISC-V特权指令规范](https://riscv.org/specifications/privileged-isa/)  

#### Supervisor Trap Value (stval) Register
在官方文档中对` stval `的描述：  
> The stval register is an SXLEN-bit read-write register formatted as shown in Figure 4.10. When
a trap is taken into S-mode, stval is written with exception-specific information to assist software
in handling the trap. Otherwise, stval is never written by the implementation, though it may
be explicitly written by software. The hardware platform will specify which exceptions must set
stval informatively and which may unconditionally set it to zero.  
简单地说就是它保存了` trap `的附加信息：出错的地址或者非法指令本身，对于其他异常它的值为 0 。  

#### Supervisor Scratch Register (sscratch)
在官方文档中对` sscratch `的描述：  
> The sscratch register is an SXLEN-bit read/write register, dedicated for use by the supervisor.
Typically, sscratch is used to hold a pointer to the hart-local supervisor context while the hart is
executing user code. At the beginning of a trap handler, sscratch is swapped with a user register
to provide an initial working register.  

在核（` hart `）运行用户态代码的时候，` sscratch `用来保存一个指向内核态上下文的指针。在` trap handler `的开始部分，` sscratch `和一个用户寄存器交换值来提供一个`initial working register`。  
这个寄存器的说明比较抽象，我们会在后面实验过程中分析相关代码来感受这个寄存器的用法和功能。  
这八个控制状态寄存器（CSR）是` supervisor`模式下异常处理的必要部分。这里只是简单地说明一下，更全面的内容请查阅文档[RISC-V特权指令规范](https://riscv.org/specifications/privileged-isa/)  

### 特权级指令（Supervisor Instructions）
由于这次实验涉及到的 CSR Intruction 并不复杂，数量也不多，因此这里照搬实验指导中相关的介绍。更详细的内容请查阅文档[RISC-V特权指令规范](https://riscv.org/specifications/privileged-isa/)  
+ ` csrrw dst, csr, src ` (CSR Read Write)：同时读写的原子操作，将指定 CSR 的值写入` dst `，同时将` src `的值写入 CSR。
+ ` csrr dst, csr `(CSR Read)：仅读取一个 CSR 寄存器。
+ ` csrw csr, src `(CSR Write)：仅写入一个 CSR 寄存器。
+ `csrc(i) csr, rs1 `(CSR Clear)：将 CSR 寄存器中指定的位清零，` csrc `使用通用寄存器作为 mask ，` csrci `则使用立即数。
+ ` csrs(i) csr, rs1 `(CSR Set)：将 CSR 寄存器中指定的位置 1 ，` csrc `使用通用寄存器作为 mask ，` csrci `则使用立即数。

下面将正式进入中断过程分析。  

















<span id="lab2"></span>

## Lab2

<span id="lab3"></span>
## Lab3

<span id="lab4"></span>
## Lab4

<span id="lab5"></span>
## Lab5

<span id="lab6"></span>

## Lab6