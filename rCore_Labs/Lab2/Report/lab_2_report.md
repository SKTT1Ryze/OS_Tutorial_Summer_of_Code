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
本文是本人在详细阅读` rCore-Tutorial Lab1 `的实验指导，并仔细分析了实验代码中` interrupt `部分的代码之后，结合` RISC-V `特权指令规范文档，按照实验指导中文档格式规范编写的学习报告，对` RISC-V `架构下中断处理机制做了一遍梳理，并结合代码来分析实验代码在中断机制这个模块中是怎么实现的。另外，本人对实验指导和实验源码中提出的几个思考作出了自己的看法，并提出了对源码中某处实现方式合理性的疑问和改进方法。最后，本人尝试在现有代码基础上，为实验代码仿照` Linux `内核添加了中断描述符的逻辑，包括提出实现思路和尝试修改代码实现。  
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
+ 开启中断使能
+ 保存当前上下文
+ 进入中断处理程序
+ 处理中断
+ 中断返回
+ 恢复上下文
+ 继续执行程序

在Linux等成熟的操作系统中，中断机制还要更为复杂，比如在Linux安全模式下的中断是通过中断描述符来定位中断处理程序。不过大体上的流程是一样的。在这里只是根据实验代码的实现来分析中断处理过程。  
在分析中断过程之前，还需要补充几个基础概念。  

### 上下文（Context）
上下文可以理解为当前系统的寄存器状态，在进入中断处理程序前需要保存当前上下文。
在实验代码中，上下文使用一个数据结构来抽象：  
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
![stvec](./img/stvec.png)  
` stvec `寄存器是保存发生异常时 CPU 需要跳转到的地址。其中 BASE 字段保存着有效的虚拟地址或物理地址，这个地址必须四字节对齐。MODE 字段将会决定寻址方式。  

![MODE](./img/stvec_way.png)  
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

![sstatus](./img/sstatus.png)  
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

### 获取中断入口和开启中断使能
在对 CRSs 介绍部分提到了` stvec `，这个寄存器保存着 CPU 发生异常时需要跳转的地址。在实验代码中，有一个用汇编语言写的函数` __interrupt `用于状态保存，调用中断处理程序等工作，这个函数的地址就是我们需要跳转的中断入口，而我们要做的，就是把这个中断入口写入到` stvec `中。  
本次实验所分析的代码全部在` os/src/interrupt `目录下，而获取中断入口和开启中断使能的工作在` os/src/interrupt/handler.rs `文件中完成。  
下面是` os/src/interrupt/handler.rs `中的部分源码：  
` os/src/interrupt/handler.rs `  
```Rust
/// 初始化中断处理
///
/// 把中断入口 `__interrupt` 写入 `stvec` 中，并且开启中断使能
pub fn init() {
    unsafe {
        extern "C" {
            /// `interrupt.asm` 中的中断入口
            fn __interrupt();
        }
        // 使用 Direct 模式，将中断入口设置为 `__interrupt`
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);

        // 开启外部中断使能
        sie::set_sext();

        // 在 OpenSBI 中开启外部中断
        *PhysicalAddress(0x0c00_2080).deref_kernel() = 1u32 << 10;
        // 在 OpenSBI 中开启串口
        *PhysicalAddress(0x1000_0004).deref_kernel() = 0x0bu8;
        *PhysicalAddress(0x1000_0001).deref_kernel() = 0x01u8;
        // 其他一些外部中断相关魔数
        *PhysicalAddress(0x0C00_0028).deref_kernel() = 0x07u32;
        *PhysicalAddress(0x0C20_1000).deref_kernel() = 0u32;
    }
}
```
上面提到过我们需要将中断入口写入到` stvec `中，实现一步的就是上面代码中的这一行：  
` os/src/interrupt/handler.rs `  
```Rust
stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
```
使用 Direct 模式，将中断入口设置为` __interrupt `。我们在上面提到过` stvec `的 MODE 字段将会决定目标地址的寻址方式。这里设置为 Direct ，意味着` __interrupt `即为跳转地址。` stvec::write `传入` __interrupt `和` stvec::TrapMode::Direct 参数`，将` stvec `的 BASE 字段设置为` __interrupt `的地址，MODE 字段设置为 Direct ，这样完成了中断入口的获取。可以看到，利用 Rust 的 riscv 库，可以很方便地完成这一工作。  
然后我们来看一下下一行代码：  
` os/src/interrupt/handler.rs `  
```Rust
sie::set_sext();
```
很容易可以猜出这行代码做了什么工作。之前介绍特权级寄存器的时候提到：  
` sie `指出 CPU 目前能处理和必须忽略的中断。  
因此这行代码的作用就是开启中断使能，但我们知道中断有很多种，分别对应` sie `中的各个使能位，这里开启的是哪种中断呢？  
Ctrl + 鼠标左键去看看源码，跳转到下面这个位置：  
`...sie.rs`
```Rust
set_clear_csr!(
    /// Supervisor External Interrupt Enable
    , set_sext, clear_sext, 1 << 9);
```
从注释 Supervisor External Interrupt Enable 可以看出是开启了外部中断使能。  
最后看剩下几行代码：  
` os/src/interrupt/handler.rs `  
```Rust
// 在 OpenSBI 中开启外部中断
*PhysicalAddress(0x0c00_2080).deref_kernel() = 1u32 << 10;
// 在 OpenSBI 中开启串口
*PhysicalAddress(0x1000_0004).deref_kernel() = 0x0bu8;
*PhysicalAddress(0x1000_0001).deref_kernel() = 0x01u8;
// 其他一些外部中断相关魔数
*PhysicalAddress(0x0C00_0028).deref_kernel() = 0x07u32;
*PhysicalAddress(0x0C20_1000).deref_kernel() = 0u32;
```
这里查看一下` deref_kernel() `的源码：  
` ...address.rs `
```Rust
/// 从物理地址经过线性映射取得 &mut 引用
pub fn deref_kernel<T>(self) -> &'static mut T {
	VirtualAddress::from(self).deref()
    }
```
再查看一下` deref() `的源码：  
` ...address.rs `
```Rust
pub fn deref<T>(self) -> &'static mut T {
        unsafe { &mut *(self.0 as *mut T) }
    }
```
可以分析出这个` PhysicalAddress `类中的方法会从物理地址经过线性映射到虚拟地址，并从中获得一个类型的引用。  
再结合中文注释猜测上面的代码块通过对特定物理地址映射到的虚拟地址的内存进行赋值，来完成在OpenSBI中开启外部中断的工作。开启中断后，就会进入中断入口进行一系列中断处理的过程。  
### 上下文保存
我们在前面已经提到了一个用于保存上下文的数据结构` Context `：  
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
在实验代码中，上下文的保存和恢复采用以下方法：  
先用栈上的一小段空间来把需要保存的全部通用寄存器和 CSR 寄存器保存在栈上，保存完之后在跳转到 Rust 编写的中断处理函数；而对于恢复，则直接把备份在栈上的内容写回寄存器。  
由于程序涉及到了寄存器级别的操作，因此我们使用汇编语言来实现，这部分操作由汇编文件` os/src/interrupt/interrupt.asm `来实现，为了文档的整洁，这里将该文件中的代码分模块进行分析。  
这个文件里首先定义了两个宏用于内存读写操作：  
` os/src/interrupt/interrupt.asm `  
```
# 宏：将寄存器存到栈上
.macro SAVE reg, offset
    sd  \reg, \offset*8(sp)
.endm

# 宏：将寄存器从栈中取出
.macro LOAD reg, offset
    ld  \reg, \offset*8(sp)
.endm
```
这段宏代码十分容易理解，中文注释也写出了它们的作用，就是通过传递两个参数，寄存器和相对于栈顶的偏移量，来进行对栈空间内存的读写操作。  
然后下面一段代码就是对上下文的保存：  
` os/src/interrupt/interrupt.asm `  
```
    .section .text
    .globl __interrupt
# 进入中断
# 保存 Context 并且进入 Rust 中的中断处理函数 interrupt::handler::handle_interrupt()
__interrupt:
    # 因为线程当前的栈不一定可用，必须切换到内核栈来保存 Context 并进行中断流程
    # 因此，我们使用 sscratch 寄存器保存内核栈地址
    # 思考：sscratch 的值最初是在什么地方写入的？

    # 交换 sp 和 sscratch（切换到内核栈）
    csrrw   sp, sscratch, sp
    # 在内核栈开辟 Context 的空间
    addi    sp, sp, -36*8
    
    # 保存通用寄存器，除了 x0（固定为 0）
    SAVE    x1, 1
    # 将本来的栈地址 sp（即 x2）保存
    csrr    x1, sscratch
    SAVE    x1, 2
    SAVE    x3, 3
    SAVE    x4, 4
    SAVE    x5, 5
    SAVE    x6, 6
    SAVE    x7, 7
    SAVE    x8, 8
    SAVE    x9, 9
    SAVE    x10, 10
    SAVE    x11, 11
    SAVE    x12, 12
    SAVE    x13, 13
    SAVE    x14, 14
    SAVE    x15, 15
    SAVE    x16, 16
    SAVE    x17, 17
    SAVE    x18, 18
    SAVE    x19, 19
    SAVE    x20, 20
    SAVE    x21, 21
    SAVE    x22, 22
    SAVE    x23, 23
    SAVE    x24, 24
    SAVE    x25, 25
    SAVE    x26, 26
    SAVE    x27, 27
    SAVE    x28, 28
    SAVE    x29, 29
    SAVE    x30, 30
    SAVE    x31, 31

    # 取出 CSR 并保存
    csrr    t0, sstatus
    csrr    t1, sepc
    SAVE    t0, 32
    SAVE    t1, 33
    # 调用 handle_interrupt，传入参数
    # context: &mut Context
    mv      a0, sp
    # scause: Scause
    csrr    a1, scause
    # stval: usize
    csrr    a2, stval
    jal handle_interrupt

```
线程当前的栈不一定可用，因此需要切换到内核栈来保存` Context `并进行中断流程。内核栈地址保存在` sscratch 中 `，因此交换` sp `和` sscratch `：  
` os/src/interrupt/interrupt.asm `  
```
csrrw   sp, sscratch, sp
```
现在` sp `指向了内核栈的地址，可以通过减少` sp `的值来在内核栈开辟一片空间来存储` Context `：  
` os/src/interrupt/interrupt.asm `  
```
addi    sp, sp, -36*8
```
这样就开辟了一片 36×8 大小的空间来保存` Context `，下面保存通用寄存器 x1～x2 ：  
` os/src/interrupt/interrupt.asm `  
```
# 保存通用寄存器，除了 x0（固定为 0）
    SAVE    x1, 1
    # 将本来的栈地址 sp（即 x2）保存
    csrr    x1, sscratch
    SAVE    x1, 2
    SAVE    x3, 3
    SAVE    x4, 4
    SAVE    x5, 5
    SAVE    x6, 6
    SAVE    x7, 7
    SAVE    x8, 8
    SAVE    x9, 9
    SAVE    x10, 10
    SAVE    x11, 11
    SAVE    x12, 12
    SAVE    x13, 13
    SAVE    x14, 14
    SAVE    x15, 15
    SAVE    x16, 16
    SAVE    x17, 17
    SAVE    x18, 18
    SAVE    x19, 19
    SAVE    x20, 20
    SAVE    x21, 21
    SAVE    x22, 22
    SAVE    x23, 23
    SAVE    x24, 24
    SAVE    x25, 25
    SAVE    x26, 26
    SAVE    x27, 27
    SAVE    x28, 28
    SAVE    x29, 29
    SAVE    x30, 30
    SAVE    x31, 31
```
注意这里对原来` x2 `（即` sp `）的保存，已经和` sscratch `交换，因此保存的是` sscratch `的值。  
下面是对` sstatus `和` sepc `的保存：  
` os/src/interrupt/interrupt.asm `  
```
# 取出 CSR 并保存
    csrr    t0, sstatus
    csrr    t1, sepc
    SAVE    t0, 32
    SAVE    t1, 33
```
这里先用` csrr `指令将` sstatus `和` sepc `取出到` t0 `和` t1 `中保存，然后再用 SAVE 宏保存在内核栈中。  
最后是调用中断处理函数之前的参数准备和跳转到中断处理函数中执行：  
` os/src/interrupt/interrupt.asm `  
```
# 调用 handle_interrupt，传入参数
    # context: &mut Context
    mv      a0, sp
    # scause: Scause
    csrr    a1, scause
    # stval: usize
    csrr    a2, stval
    jal handle_interrupt
```
在` RISC-V `架构的函数调用规范中，我们约定寄存器 a0～a7 用于保存调用参数，且 a0，a1 用于传递返回值。因此这里将指向一个` Context `的指针，` scause `和` stval `分别保存在` a0 `，` a1 `和` a2 `中作为参数传递。   
最后一条` jal `指令将跳转到 handle_interrupt 函数中执行并设置好返回地址。  
这里有个问题，为什么要传递` scause `和` stval `这两个参数？  
在前面对这两个特权级寄存器的介绍中提到过，` scause `指示发生异常的种类，而` stval `保存了` trap `的附加信息：出错的地址或者非法指令本身。因此我们通过传递这两个参数让中断处理程序知道引起中断的原因是什么，以便作出相应的弥补操作。不过貌似实验代码中并没有对` trap `参数进行任何处理，可能这是为了后面的完善开发提供的接口。  
### 进入中断处理程序处理中断
中断处理程序` handle_interrupt `在文件` handler.rs `中。  
` os/src/interrupt/handler.rs `  
```Rust
/// 中断的处理入口
///
/// `interrupt.asm` 首先保存寄存器至 Context，其作为参数和 scause 以及 stval 一并传入此函数
/// 具体的中断类型需要根据 scause 来推断，然后分别处理
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    // 返回的 Context 必须位于放在内核栈顶
    match scause.cause() {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // 系统调用
        Trap::Exception(Exception::UserEnvCall) => syscall_handler(context),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // 外部中断（键盘输入）
        Trap::Interrupt(Interrupt::SupervisorExternal) => supervisor_external(context),
        // 其他情况，终止当前线程
        _ => fault(context, scause, stval),
    }
}
```
这就是在` interrupt.asm `中跳转的中断处理函数了，可以看到这里使用 match 对` scause `进行模式匹配，来判断是哪种类型的中断，并且给出了对应的中断处理：  
` os/src/interrupt/handler.rs `  
```Rust
/// 处理 ebreak 断点
///
/// 继续执行，其中 `sepc` 增加 2 字节，以跳过当前这条 `ebreak` 指令
fn breakpoint(context: &mut Context) -> *mut Context {
    println!("Breakpoint at 0x{:x}", context.sepc);
    context.sepc += 2;
    context
}

/// 处理时钟中断
fn supervisor_timer(context: &mut Context) -> *mut Context {
    timer::tick();
    PROCESSOR.get().park_current_thread(context);
    PROCESSOR.get().prepare_next_thread()
}

/// 出现未能解决的异常，终止当前线程
fn fault(_context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    println!(
        "{:x?} terminated with {:x?}",
        PROCESSOR.get().current_thread(),
        scause.cause()
    );
    println!("stval: {:x}", stval);
    PROCESSOR.get().kill_current_thread();
    // 跳转到 PROCESSOR 调度的下一个线程
    PROCESSOR.get().prepare_next_thread()
}

/// 处理外部中断，只实现了键盘输入
fn supervisor_external(context: &mut Context) -> *mut Context {
    let mut c = console_getchar();
    if c <= 255 {
        if c == '\r' as usize {
            c = '\n' as usize;
        }
        STDIN.push(c as u8);
    }
    context
}
```
这些中断处理有些是用了 Rust 库，有些是使用了其他模块的函数，这里不再继续深入分析。  
这里我们可以看看` Trap `这个枚举定义：  
```Rust
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Trap {
    Interrupt(Interrupt),
    Exception(Exception),
}
```
这个枚举有两个成员，` Interrupt `和` Exception `，分别对应中断和异常。下面再看一下` Interrupt `和` Exception `这两种枚举类型的定义：  
```Rust
/// Interrupt
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interrupt {
    UserSoft,
    SupervisorSoft,
    UserTimer,
    SupervisorTimer,
    UserExternal,
    SupervisorExternal,
    Unknown,
}

/// Exception
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Exception {
    InstructionMisaligned,
    InstructionFault,
    IllegalInstruction,
    Breakpoint,
    LoadFault,
    StoreMisaligned,
    StoreFault,
    UserEnvCall,
    InstructionPageFault,
    LoadPageFault,
    StorePageFault,
    Unknown,
}
```
可以看到` Interrupt `和` Exception `也被细分成了多种类型，对于不同的类型操作系统会作出不同的相应操作。  
### 中断返回和恢复上下文
中断返回和恢复上下文的操作将又回到汇编文件` interrupt.asm `文件中分析。  
在前面的` handle_interrupt `函数中，返回了一个指向` Context `数据结构的指针，这个返回值保存在` a0 `中。实际上，这个指针也是指向内核栈栈顶的。因此下面这行代码将从` a0 `中恢复` sp `：  
` os/src/interrupt/interrupt.asm `  
```
mv      sp, a0
```
而本人对于实验代码中的这种做法持有疑问，这将会在后面的思考环节进行阐述。  
然后就是利用 LOAD 宏恢复` sstatus `，` sepc `：  
` os/src/interrupt/interrupt.asm `  
```
LOAD    t0, 32
LOAD    t1, 33
csrw    sstatus, t0
csrw    sepc, t1
```
最后将内核栈写入` sscratch `，此操作完成后` sscratch `将会和发生中断之前保持一致。  
` os/src/interrupt/interrupt.asm `  
```
addi    t0, sp, 36*8
csrw    sscratch, t0
```
然后是恢复通用寄存器，同样使用 LOAD 宏完成此操作：  
` os/src/interrupt/interrupt.asm `  
```
LOAD    x1, 1
LOAD    x3, 3
LOAD    x4, 4
LOAD    x5, 5
LOAD    x6, 6
LOAD    x7, 7
LOAD    x8, 8
LOAD    x9, 9
LOAD    x10, 10
LOAD    x11, 11
LOAD    x12, 12
LOAD    x13, 13
LOAD    x14, 14
LOAD    x15, 15
LOAD    x16, 16
LOAD    x17, 17
LOAD    x18, 18
LOAD    x19, 19
LOAD    x20, 20
LOAD    x21, 21
LOAD    x22, 22
LOAD    x23, 23
LOAD    x24, 24
LOAD    x25, 25
LOAD    x26, 26
LOAD    x27, 27
LOAD    x28, 28
LOAD    x29, 29
LOAD    x30, 30
LOAD    x31, 31
LOAD    x2, 2
```
注意，这里恢复` x2 `即恢复` sp `，放到最后恢复是为了上面可以正常使用 LOAD 宏。  
最后是中断返回：  
` os/src/interrupt/interrupt.asm `  
```
sret
```
到这里中断处理的代码分析就差不多结束了。本次实验代码还实现了一个比较特殊的中断：时钟中断。下面是对` os/src/interrupt/timer.rs `文件的代码分析。  

### 时钟中断
` os/src/interrupt/timer.rs `文件实现了预约和处理中断。  
中断计数和中断间隔定义：  
` os/src/interrupt/timer.rs `
```Rust
pub static mut TICKS: usize = 0;
static INTERVAL: usize = 100000;
```
和上面分析过的外部中断一样，这里需要设置` sie `开启时钟中断使能，并且预约第一次时钟中断：  
` os/src/interrupt/timer.rs `
```Rust
pub fn init() {
    unsafe {
        // 开启 STIE，允许时钟中断
        sie::set_stimer();
    }
    // 设置下一次时钟中断
    set_next_timeout();
}
```
下面是设置下一次时钟中断的函数实现：  
` os/src/interrupt/timer.rs `
```Rust
fn set_next_timeout() {
    set_timer(time::read() + INTERVAL);
}
```
其中` set_timer `函数是通过 SBI 提供的接口实现的：  
```Rust
pub fn set_timer(time: usize) {
    sbi_call(SBI_SET_TIMER, time, 0, 0);
}
```
` sbi_call `的实现：  
```Rust
/// SBI 调用
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        llvm_asm!("ecall"
            : "={x10}" (ret)
            : "{x10}" (arg0), "{x11}" (arg1), "{x12}" (arg2), "{x17}" (which)
            : "memory"      // 如果汇编可能改变内存，则需要加入 memory 选项
            : "volatile"); // 防止编译器做激进的优化（如调换指令顺序等破坏 SBI 调用行为的优化）
    }
    ret
}
```
其中使用到了内敛汇编，同时发现这段代码也是我们 Lab0 中实现的一部分。  
最后就是时钟中断的函数，这里的设计是每当中断计数到整除 100 时打印中断计数：  
` os/src/interrupt/timer.rs `
```Rust
pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            println!("{} tick", TICKS);
        }
    }
}
```
那么这个函数在什么时候调用呢？我们回到` os/src/interrupt/handler.rs `文件，里面处理时钟中断的函数：  
` os/src/interrupt/handler.rs `
```Rust
/// 处理时钟中断
fn supervisor_timer(context: &mut Context) -> *mut Context {
    timer::tick();
    PROCESSOR.get().park_current_thread(context);
    PROCESSOR.get().prepare_next_thread()
}
```
这样思路就很明了了：先是硬件发生时钟中断，然后设置` scause `为时钟中断对应的值，传递到中断处理函数` handle_interrupt `里面，然后根据` scause `执行处理时钟中断的函数，调用` tick（） `，最后中断返回。  

### 运行结果分析
本人参考实验指导，并且结合自己的知识，一步步再现了实验代码的中断模块。下面对运行结果进行测试。项目代码：[lab1-interrupt](https://github.com/SKTT1Ryze/OS_Tutorial_Summer_of_Code/tree/master/rCore_Labs/Lab1/os)  
在` main.rs `中加入死循环，让时钟中断一直触发：  
` os/src/main.rs `  
```Rust
// the first function to be called after _start
#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello, rCore-Tutorial!");
    println!("I have done Lab 1");
    //panic!("Hi,panic here...")
    interrupt::init();

    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    //unreachable!();
    loop{};
}
```
在时钟中断处理函数` tick() `中打印当前中断计数：  
` os/src/interrupt/timer.rs `
```Rust
pub fn tick() {
    set_next_timeout();
    unsafe {
        TICKS += 1;
        if TICKS % 100 == 0 {
            println!("{} tick", TICKS);
        }
    }
}
```
运行结果如下：  
![result](./img/result.png)  
然后我们尝试让它中断嵌套：  
```Rust
pub fn breakpoint(context: &mut Context) {
    println!("Breakpoint at 0x{:x}", context.sepc);
    println!("Another breakpoint interrupt start");
    unsafe {
        llvm_asm!("ebreak"::::"volatile");
    };
    println!("Another breakpoint interrupt end");
    context.sepc += 2;
    //println!("breakpoint interrupt return");
}
```
结果如下：  
![result 01](./img/result_01.png)  
可以看到程序陷入了无穷嵌套。  

### 思考
在分析 Lab1 的代码过程中，遇到一些问题，其中包括在源码中注释的思考题，和我本人对实验代码提出的一些疑问。这里将会集中进行探讨。  
#### 思考题1
思考：` sscratch `的值最初是在什么地方写入的？  
在前面提到：  
在核（` hart `）运行用户态代码的时候，` sscratch `用来保存一个指向内核态上下文的指针。在` trap handler `的开始部分，` sscratch `和一个用户寄存器交换值来提供一个`initial working register`。  
阅读` OpenSBI `代码，在以下文件中：  
` OpenSBI/lib/sbi/sbi_hart.c `  
```Rust
if (next_mode == PRV_S) {
		csr_write(CSR_STVEC, next_addr);
		csr_write(CSR_SSCRATCH, 0);
		csr_write(CSR_SIE, 0);
		csr_write(CSR_SATP, 0);
	} else if (next_mode == PRV_U) {
		csr_write(CSR_UTVEC, next_addr);
		csr_write(CSR_USCRATCH, 0);
		csr_write(CSR_UIE, 0);
	}
```
可以看出，在hart要切换MODE时，如果要切换到S态，那么将sscratch寄存器内写入0。  
因此最初写入是在bootloader（OpenSBI）加载完毕，即将切换到操作系统内核时，这一特权级切换过程中写入的。  
#### 思考题2
思考：` a0 `是在哪里被赋值的？（有两种情况）  
+ 在进入函数` handle_interrupt `之前的参数准备阶段被赋值；
+ 从` handle_interrupt `返回时作为返回参数被赋值。
#### 思考题3
思考：为什么不恢复` scause `和` stval `？如果不恢复，为什么之前要保存？  
本人认为` scause `和` stval `不需要恢复，之前也没有保存。  
之前提到过这两个寄存器的作用，` scause `指示发生异常的种类，而` stval `保存了` trap `的附加信息：出错的地址或者非法指令本身。因此这两个寄存器只在中断出现的时候派上用场，在一般情况下不影响程序的运行，而保存上下文的目的就是要保证中断处理完之后回到原来中断的地方程序能继续运行，从这个角度来看就不必保存这两个寄存器。而又回到之前为什么` scause `和` stval `包含在数据结构` Context `中的问题，既然不需要保存，自然就不需要放在` Context `里面了。  
#### 对实验代码的疑问
在恢复上下文的代码中，有这样一条指令：  
` os/src/interrupt/interrupt.asm `
```
mv      sp, a0
```
这是从` handle_interrupt ` 中的返回值` a0 `中读取` sp `，而` a0 ` 同时也是作为调用参数传入到` handle_interrupt `中的。  
疑问是：这样的实现方法不是会有风险吗？  
因为后面无论是恢复 CRSs 还是恢复通用寄存器，都与` sp `的值相关，如果返回值不对，或者说在` handle_interrupt `中修改了` a0 `的值，那么后面的恢复上下文过程就无法正确执行，导致系统崩溃。在` x86 `架构中的函数调用机制使用了一种栈帧结构，本人觉得与实验代码的恢复` sp `的机制相比，栈帧结构更为完全。  
另外，在实验代码有这么一行注释：  
```
// 返回的 Context 必须位于内核栈顶
```
也就是说这里返回的指针必须指向内核栈的栈顶。  
这不正反映了这个机制的不稳定性吗。  
### 小结
终于做完 Lab1 了，比想像中还要花费精力。对 Lab1 的修改版本将会在另外的报告中说明。从这个实验中不仅加深了对` Rust `语言的理解，还亲身感受到了如何用Rust语言编写操作系统的，收益良多。剩下的实验继续加油。  


<span id="lab2"></span>

## Lab2
### 引言
我们之前在 C/C++ 语言等中使用过` malloc/free `等动态内存分配方法，与在编译期就已完成的静态内存分配相比，动态的内存分配可以根据程序运行时状态修改内存申请的时机及大小，显得更为灵活，但是这是需要操作系统的支持的，同时也会带来一些开销。  
在` rCore `中也需要动态内存分配，比如` Box<T> `，` Rc<T> `和` Vec `等等。
### 实验内容
+ 实现动态内存的分配
+ 了解 QEMU 模拟的 RISC-V Virt 计算机的物理内存
+ 通过页的方式对物理内存进行管理
### 实验过程
#### 动态内存分配
为了解决外碎片问题，这里使用了伙伴系统内存分配算法。  
直接调用王嘉杰学长写的` Buddy System Allocator`  
` os/src/memory/heap.rs `  
```Rust
use super::config::KERNEL_HEAP_SIZE;
use buddy_system_allocator::LockedHeap;
/// Heap space for alloc memory
/// 
/// Size: [`KERNEL_HEAP_SIZE`]
/// This space will be in .bss segment
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0;KERNEL_HEAP_SIZE];

/// Heap allocator
/// 
/// ### `#[global_allocator]`
/// [`LockedHeap`] implements [`alloc::alloc::GlobalAlloc`] trait,
/// Can alloc space when heap is needed. such as: `Box`, `Arc`, etc.
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

/// Initialize OS heap space when running
pub fn init() {
    //use `HEAP_SPACE` as heap
    unsafe {
        HEAP.lock().init(
            HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE
        )
    }
}

/// Alloc space error, panic
#[alloc_error_handler]
fn alloc_error_handler(_: alloc::alloc::Layout) -> ! {
    panic!("Alloc error")
}
```
这里主要就是调库，然后告诉编译器使用定义的一段空间作为预留的堆，而`LockedHeap`实现了`alloc::alloc::GlobalAlloc` trait。  
同时本人也写了一个 Buddy System 的 demo，会在另外的报告中介绍。  
#### 物理内存探测
在 RISC-V 中，操作系统通过 bootloader，即 OpenSBI 固件来知道物理内存所在的物理地址。它完成对于包括物理内存在内的所有外设的扫描，将扫描结果以 DTB 的格式保存在物理内存中的某个地方，然后将其地址保存在` a1 `寄存器中返回。  
我们使用 [0x80000000, 0x88000000]作为 DRAM 物理内存地址范围。  
下面将 DRAM 物理内存结束地址硬编码到内核中：  
` os/src/memory/config.rs `
```Rust
//! some constant about memory

use super::address::*;
use lazy_static::*;
...
/// 可以访问的内存区域结束地址
pub const MEMORY_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x8800_0000);

lazy_static! {
    /// The address of end of kernel code, and the address of begin of space used to alloc
    pub static ref KERNEL_END_ADDRESS: VirtualAddress = VirtualAddress(kernel_end as usize);
}

extern "C" {
    /// The end of kernel code assigned by `linker.ld`
    /// 
    /// exist as var [`KERNEL_END_ADDRESS`]
    fn kernel_end();
}
```
我们得到了内核的结束虚拟地址` KERNEL_END_ADDRESS `  
#### 物理内存管理
我们将用一个新的结构来封装一下物理页，便于和其他类型地址区分和同时封装一些页帧和地址相互转换的功能。  
相关设置：  
` os/src/memory/config.rs `
```Rust
/// 页 / 帧大小，必须是 2^n
pub const PAGE_SIZE: usize = 4096;

/// MMIO 设备段内存区域起始地址
pub const DEVICE_START_ADDRESS: PhysicalAddress = PhysicalAddress(0x1000_0000);

/// MMIO 设备段内存区域结束地址
pub const DEVICE_END_ADDRESS: PhysicalAddress = PhysicalAddress(0x1001_0000);
```
下面是对物理页的概念进行封装：  
` os/src/memory/frame/frame_tracker.rs `
```Rust
//! 「`Box`」 [`FrameTracker`] to provide physical frame
#![allow(unused)]
use crate::memory::{address::*, FRAME_ALLOCATOR, PAGE_SIZE};

pub struct FrameTracker(pub(super) PhysicalPageNumber);

impl FrameTracker {
    /// PhysicalAddress of frame
    pub fn address(&self) -> PhysicalAddress {
        self.0.into()
    }

    /// PageNumber of frame
    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0
    }
}

impl Drop for FrameTracker {
    fn drop(&mut self) {
        FRAME_ALLOCATOR.lock().dealloc(self);
    }
}

/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::Deref for FrameTracker {
    type Target = [u8; PAGE_SIZE];
    fn deref(&self) -> &Self::Target {
        self.page_number().deref_kernel()
    }
}

/// `FrameTracker` 可以 deref 得到对应的 `[u8; PAGE_SIZE]`
impl core::ops::DerefMut for FrameTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.page_number().deref_kernel()
    }
}
```
分配器分配` FrameTracker `这个结构作为一个帧的标识，随着操作系统不再需要这个物理页，我们需要回收，这里利用 Rust 的 drop 机制在生命周期结束的时候自动实现回收。  
最后封装一个物理页分配器，具体算法用` Allocator `的 trait 封装起来，具体实现在` os/src/algorithm/src/allocator `中。  
` os/src/memory/frame/allocator.rs `
```Rust
//! 提供帧分配器 [`FRAME_ALLOCATOR`](FrameAllocator)
//!
//! 返回的 [`FrameTracker`] 类型代表一个帧，它在被 drop 时会自动将空间补回分配器中。

use super::*;
use crate::memory::*;
use algorithm::*;
use lazy_static::*;
use spin::Mutex;

lazy_static! {
    /// frame allocator
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<AllocatorImpl>> = Mutex::new(FrameAllocator::new(Range::from(
            PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))..PhysicalPageNumber::floor(MEMORY_END_ADDRESS),
        )
    ));
}
/// 基于线段树的帧分配 / 回收
pub struct FrameAllocator<T: Allocator> {
    /// begin of usable space
    start_ppn: PhysicalPageNumber,
    /// allocator
    allocator: T,
}

impl<T: Allocator> FrameAllocator<T> {
    pub fn new(range: impl Into<Range<PhysicalPageNumber>>+Copy) -> Self {
        //println!("Allocator size: {}", range.into().len());
        FrameAllocator {
            start_ppn: range.into().start,
            allocator: T::new(range.into().len()),
        }
    }
    /// alloc frame, if none return `Err`
    pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        self.allocator
            .alloc()
            .ok_or("no available frame to allocate")
            .map(|offset| FrameTracker(self.start_ppn+offset))
    }

    /// 将被释放的帧添加到空闲列表的尾部
    ///
    /// 这个函数会在 [`FrameTracker`] 被 drop 时自动调用，不应在其他地方调用
    pub(super) fn dealloc(&mut self, frame: &FrameTracker) {
        self.allocator.dealloc(frame.page_number()-self.start_ppn);
    }
}
```
这个分配器会以一个` PhysicalPageNumber `的 Range 初始化，然后把起始地址记录下来，把整个区间的长度告诉具体的分配器算法，当分配的时候就从算法中取得一个可用的位置作为 offset，再加上起始地址返回回去。  
` Allocator `trait 如下：  
```Rust
/// 分配器：固定容量，每次分配 / 回收一个元素
pub trait Allocator {
    /// create allocator with capacity
    fn new(capacity: usize) -> Self;
    /// alloc a item. error return `None`
    fn alloc(&mut self) -> Option<usize>;
    /// dealloc a item
    fn dealloc(&mut self, index: usize);
}
```
同时还定义了一个可以分配连续帧的` VectorAllocator `trait，这里不多进行讨论：  
```Rust
/// 分配器：固定容量，每次分配 / 回收指定大小的元素
pub trait VectorAllocator {
    /// create allocator with capacity
    fn new(capacity: usize) -> Self;
    /// alloc space with size of `size`. error return `None`
    fn alloc(&mut self, size: usize, align: usize) -> Option<usize>;
    /// dealloc space with `start` and `size`
    fn dealloc(&mut self, start: usize, size: usize, align: usize);
}
```
### 小结
Lab2 主要是实现了操作系统的内存管理模块，通过这次实验，最大的收获是了解了如何在 Rust 中使用 trait 对某个结构体的行为进行抽象。  
<span id="lab3"></span>
## Lab3
### 引言
到目前为止，我们简易的操作系统还只是一个内核在执行，还没有多任务的概念。在现代的操作系统中，为了让其他的程序能方便的运行在操作系统上，需要完成的一个很重要的抽象是「每个程序有自己的地址空间，且地址空间范围是一样的」，这将会减少了上层程序的大量麻烦，否则程序本身要维护自己需要的物理内存，这也会导致极大程度的不安全。  
操作系统为了解决这个问题，使用了虚拟地址。  
### 实验内容
+ 虚拟地址和物理地址的概念和关系
+ 利用页表完成虚拟地址到物理地址的映射
+ 实现内核的重映射
### 实验过程
#### 从虚拟地址到物理地址
虚拟地址和物理地址的概念在操作系统课上已经学过，这里不多加讨论。实现方法参考` os/src/memory/address.rs `  
#### 修改内核
将内核代码放在虚拟地址空间中以 0xffffffff80200000 开头的一段高地址空间中。这意味着原来放在 0x80200000 起始地址的全部内核结构被平移到了 0xffffffff80200000 的地址上。  
` os/src/linker.ld `  
```Rust
/* Arch */
OUTPUT_ARCH(riscv)

/* start entry */
ENTRY(_start)

/* start address of data */
BASE_ADDRESS = 0xffffffff80200000; /* VirtualAddress */

SECTIONS
{
    /* . is location counter */
    . = BASE_ADDRESS;

    /* start of kernel */
    kernel_start = .;

    /* align */
    . = ALIGN(4K);
    text_start = .;

    /* .text field */
    .text : {
        /* 把 entry 函数放在最前面 */
        *(.text.entry)
        /* 要链接的文件的 .text 字段集中放在这里 */
        *(.text .text.*)
    }

    /* align */
    . = ALIGN(4K);
    rodata_start = .;

    /* .rodata field */
    .rodata : {
        /* 要链接的文件的 .rodata 字段集中放在这里 */
        *(.rodata .rodata.*)
    }

    /* align */
    . = ALIGN(4K);
    data_start = .;

    /* .data field */
    .data : {
        /* 要链接的文件的 .data 字段集中放在这里 */
        *(.data .data.*)
    }

    /* align */
    . = ALIGN(4K);
    bss_start = .;

    /* .bss field */
    .bss : {
        /* 要链接的文件的 .bss 字段集中放在这里 */
        *(.sbss .bss .bss.*)
    }

    /* align */
    . = ALIGN(4K);
    /* end of kernel */
    kernel_end = .;
}
```
我们需要告诉 RISC-V CPU 我们做了这些修改，也就是需要在启动时、在进入 rust_main 之前我们要完成一个从物理地址访存模式到虚拟访存模式的转换，同时这也意味着，我们要写一个简单的页表，完成这个线性映射：  
` os/src/entry.asm `  
```Rust
# 操作系统启动时所需的指令以及字段
#
# 我们在 linker.ld 中将程序入口设置为了 _start，因此在这里我们将填充这个标签
# 它将会执行一些必要操作，然后跳转至我们用 rust 编写的入口函数
#
# 关于 RISC-V 下的汇编语言，可以参考 https://github.com/riscv/riscv-asm-manual/blob/master/riscv-asm.md
# %hi 表示取 [12,32) 位，%lo 表示取 [0,12) 位

    .section .text.entry
    .globl _start
# 目前 _start 的功能：将预留的栈空间写入 $sp，然后跳转至 rust_main
_start:
    # 计算 boot_page_table 的物理页号
    lui t0, %hi(boot_page_table)
    li t1, 0xffffffff00000000
    sub t0, t0, t1
    srli t0, t0, 12
    # 8 << 60 是 satp 中使用 Sv39 模式的记号
    li t1, (8 << 60)
    or t0, t0, t1
    # 写入 satp 并更新 TLB
    csrw satp, t0
    sfence.vma

    # 加载栈地址
    lui sp, %hi(boot_stack_top)
    addi sp, sp, %lo(boot_stack_top)
    # 跳转至 rust_main
    # 这里同时伴随 hart 和 dtb_pa 两个指针的传入（是 OpenSBI 帮我们完成的）
    lui t0, %hi(rust_main)
    addi t0, t0, %lo(rust_main)
    jr t0

    # 回忆：bss 段是 ELF 文件中只记录长度，而全部初始化为 0 的一段内存空间
    # 这里声明字段 .bss.stack 作为操作系统启动时的栈
    .section .bss.stack
    .global boot_stack
boot_stack:
    # 16K 启动栈大小
    .space 4096 * 16
    .global boot_stack_top
boot_stack_top:
    # 栈结尾

    # 初始内核映射所用的页表
    .section .data
    .align 12
    .global boot_page_table
boot_page_table:
    .quad 0
    .quad 0
    # 第 2 项：0x8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .quad (0x80000 << 10) | 0xcf
    .zero 505 * 8
    # 第 508 项：0xffff_ffff_0000_0000 -> 0x0000_0000，0xcf 表示 VRWXAD 均为 1
    .quad (0x00000 << 10) | 0xcf
    .quad 0
    # 第 510 项：0xffff_ffff_8000_0000 -> 0x8000_0000，0xcf 表示 VRWXAD 均为 1
    .quad (0x80000 << 10) | 0xcf
    .quad 0
```

#### 实现页表
这里加入两个关于位操作的 crate ：` bitflags `和` bit_field `  
下面是对页表项的封装：  
` os/src/memory/mapping/page_table_entry.rs `
```Rust
use crate::memory::address::*;
/// page table entry for Sv39
use bit_field::BitField;
use bitflags::*;

bitflags! {
    /// 8 flags in page table entry
    #[derive(Default)]
    pub struct Flags: u8 {
        /// valid
        const VALID = 1 << 0;
        /// readable
        const READABLE = 1 << 1;
        /// writable
        const WRITABLE = 1 << 2;
        /// executable
        const EXECUTABLE = 1 << 3;
        /// user
        const USER = 1 << 4;
        /// gloabl
        const GLOBAL = 1 << 5;
        /// accessed
        const ACCESSED = 1 << 6;
        /// dirty
        const DIRTY = 1 << 7;
    }
}

#[derive(Copy, Clone, Default)]
pub struct PageTableEntry(usize);

impl PageTableEntry {
    /// write page number and flags into a page table entry
    pub fn new(page_number: PhysicalPageNumber, flags: Flags) -> Self {
        Self(
            *0usize
                .set_bits(..8, flags.bits() as usize)
                .set_bits(10..54, page_number.into()),
        )
    }

    /// get physcial page number, linear mapping
    pub fn page_number(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.0.get_bits(10..54))
    }

    /// get physcial page address, linear mapping
    pub fn address(&self) -> PhysicalAddress {
        PhysicalAddress::from(self.page_number())
    }

    /// get flags
    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_unchecked(self.0.get_bits(..8) as u8) }
    }

    /// is empty or not
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// clear
    pub fn clear(&mut self) {
        self.0 = 0;
    }

    /// check RWX is 000 or not
    pub fn has_next_level(&self) -> bool {
        let flags = self.flags();
        !(flags.contains(Flags::READABLE)
            || flags.contains(Flags::WRITABLE)
            || flags.contains(Flags::EXECUTABLE))
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("PageTableEntry")
            .field("value", &self.0)
            .field("page_number", &self.page_number())
            .field("flags", &self.flags())
            .finish()
    }
}

macro_rules! implement_flags {
    ($field: ident, $name: ident, $quote: literal) => {
        impl Flags {
            #[doc = "return `Flags::"]
            #[doc = $quote]
            #[doc = "` or `Flags::empty()`"]
            pub fn $name(value: bool) -> Flags {
                if value {
                    Flags::$field
                } else {
                    Flags::empty()
                }
            }
        }
    };
}

implement_flags! {USER, user, "USER"}
implement_flags! {READABLE, readable, "READABLE"}
implement_flags! {WRITABLE, writable, "WRITABLE"}
implement_flags! {EXECUTABLE, executable, "EXECUTABLE"}

```
有了页表项，可以很容易地对页表进行封装：  
` os/src/memory/mapping/page_table.rs `  
```Rust
//! 单一页表页面（4K） [`PageTable`]，以及相应封装 [`FrameTracker`] 的 [`PageTableTracker`]
//!
//! 每个页表中包含 512 条页表项
//!
//! # 页表工作方式
//! 1.  首先从 `satp` 中获取页表根节点的页号，找到根页表
//! 2.  对于虚拟地址中每一级 VPN（9 位），在对应的页表中找到对应的页表项
//! 3.  如果对应项 Valid 位为 0，则发生 Page Fault
//! 4.  如果对应项 Readable / Writable 位为 1，则表示这是一个叶子节点。
//!     页表项中的值便是虚拟地址对应的物理页号
//!     如果此时还没有达到最低级的页表，说明这是一个大页
//! 5.  将页表项中的页号作为下一级查询目标，查询直到达到最低级的页表，最终得到页号

use super::page_table_entry::PageTableEntry;
use crate::memory::{address::*, config::PAGE_SIZE, frame::FrameTracker};

/// 存有 512 个页表项的页表
///
/// 注意我们不会使用常规的 Rust 语法来创建 `PageTable`。相反，我们会分配一个物理页，
/// 其对应了一段物理内存，然后直接把其当做页表进行读写。我们会在操作系统中用一个「指针」
/// [`PageTableTracker`] 来记录这个页表。
#[repr(C)]
pub struct PageTable {
    pub entries: [PageTableEntry; PAGE_SIZE / 8],
}

impl PageTable {
    /// clear the page table
    pub fn zero_init(&mut self) {
        self.entries = [Default::default(); PAGE_SIZE / 8];
    }
}

/// 类似于 [`FrameTracker`]，用于记录某一个内存中页表
///
/// 注意到，「真正的页表」会放在我们分配出来的物理页当中，而不应放在操作系统的运行栈或堆中。
/// 而 `PageTableTracker` 会保存在某个线程的元数据中（也就是在操作系统的堆上），指向其真正的页表。
///
/// 当 `PageTableTracker` 被 drop 时，会自动 drop `FrameTracker`，进而释放帧。
pub struct PageTableTracker(pub FrameTracker);

impl PageTableTracker {
    /// 将一个分配的帧清零，形成空的页表
    pub fn new(frame: FrameTracker) -> Self {
        let mut page_table = Self(frame);
        page_table.zero_init();
        page_table
    }
    /// 获取物理页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        self.0.page_number()
    }
}

// PageTableEntry 和 PageTableTracker 都可以 deref 到对应的 PageTable
// （使用线性映射来访问相应的物理地址）

impl core::ops::Deref for PageTableTracker {
    type Target = PageTable;
    fn deref(&self) -> &Self::Target {
        self.0.address().deref_kernel()
    }
}

impl core::ops::DerefMut for PageTableTracker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.address().deref_kernel()
    }
}

// 因为 PageTableEntry 和具体的 PageTable 之间没有生命周期关联，所以返回 'static 引用方便写代码
impl PageTableEntry {
    pub fn get_next_table(&self) -> &'static mut PageTable {
        self.address().deref_kernel()
    }
}

```
#### 实现内核重映射
为了实现内核重映射，我们需要对内存段进行封装：  
` os/src/memory/mapping/segment.rs `  
```Rust
//! [`MapType`] and [`Segment`]

use crate::memory::{address::*, mapping::Flags, range::Range};

/// Type of mapping
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MapType {
    /// linear mapping
    Linear,
    /// framed mapping
    Framed,
}

/// A mapping segment
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Segment {
    /// mapping type
    pub map_type: MapType,
    /// range of VirtualAddress
    pub range: Range<VirtualAddress>,
    /// flags
    pub flags: Flags,
}

impl Segment {
    /// traverse PhysicalPageNumber if possiable
    pub fn iter_mapped(&self) -> Option<impl Iterator<Item = PhysicalPageNumber>> {
        match self.map_type {
            // linear mapping
            MapType::Linear => Some(self.page_range().into().iter()),
            // framed mapping, need to alloc frames
            MapType::Framed => None,
        }
    }

    /// get range of VirtualPageNumber
    pub fn page_range(&self) -> Range<VirtualPageNumber> {
        Range::from(
            VirtualPageNumber::floor(self.range.start)..VirtualPageNumber::ceil(self.range.end),
        )
    }
}

```
有了页表、内存段，我们对这两个进行组合和封装，借助其中对页表的操作实现对内存段的映射，然后实现对页表的查找，实现一个虚拟页对物理页的映射，最后实现一个联系的段的映射：  
` os/src/memory/mapping/mapping.rs `  
代码太长，这里不贴出来。  
最后对内核进行重映射：  
` os/src/memory/mapping/memory_set.rs `  
```Rust
...
/// create remapping for kernel
    pub fn new_kernel() -> MemoryResult<MemorySet> {
        // 在 linker.ld 里面标记的各个字段的起始点，均为 4K 对齐
        extern "C" {
            fn text_start();
            fn rodata_start();
            fn data_start();
            fn bss_start();
        }

        // create segments
        let segments = vec![
            // DEVICE segment，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(DEVICE_START_ADDRESS..DEVICE_END_ADDRESS),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // .text segment，r-x
            Segment {
                map_type: MapType::Linear,
                range: Range::from((text_start as usize)..(rodata_start as usize)),
                flags: Flags::READABLE | Flags::EXECUTABLE,
            },
            // .rodata segment，r--
            Segment {
                map_type: MapType::Linear,
                range: Range::from((rodata_start as usize)..(data_start as usize)),
                flags: Flags::READABLE,
            },
            // .data segment，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from((data_start as usize)..(bss_start as usize)),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // .bss segment，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(VirtualAddress::from(bss_start as usize)..*KERNEL_END_ADDRESS),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
            // reserved segment，rw-
            Segment {
                map_type: MapType::Linear,
                range: Range::from(*KERNEL_END_ADDRESS..VirtualAddress::from(MEMORY_END_ADDRESS)),
                flags: Flags::READABLE | Flags::WRITABLE,
            },
        ];
        let mut mapping = Mapping::new()?;
        // save all allocated pairs
        let mut allocated_pairs = Vec::new();

        // create mapping for every segments
        for segment in segments.iter() {
            // add the mapping relationships to allocated_pairs
            allocated_pairs.extend(mapping.map(segment, None)?);
        }
        Ok(MemorySet {
            mapping,
            segments,
            allocated_pairs,
        })
    }
    。。。
```
### 小结
本次实验我们利用页表实现了虚拟地址到物理地址的映射和内核空间段的重映射。  
<span id="lab4"></span>
## Lab4
### 引言
进程是资源的分配单位，线程是CPU的基本调度单位。  
出于OS对计算机系统精细管理的目的，我们通常将“正在运行”的动态特性从进程中剥离出来，这样的一个借助 CPU 和栈的执行流，我们称之为线程 (Thread) 。一个进程可以有多个线程，也可以如传统进程一样只有一个线程。  
### 实验内容
+ 线程和进程的概念以及运行状态的表示
+ 线程的切换
+ 对CPU进行抽象在上面完成对线程的调度
### 实验过程
#### 线程和进程的表示
线程表示：  
` os/src/process/thread.rs `  
```Rust
/// TCB
pub struct Thread {
    /// ID
    pub id: ThreadID,
    /// priority
    pub priority: usize,
    /// Stack
    pub stack: Range<VirtualAddress>,
    /// process belonged
    pub process: Arc<RwLock<Process>>,
    /// Some vals
    pub inner: Mutex<ThreadInner>,
}

/// changable part of thread
pub struct ThreadInner {
    /// Context
    pub context: Option<Context>,
    /// is sleep or not
    pub sleeping: bool,
    /// Opening files
    pub descriptors: Vec<Arc<dyn INode>>,
}
```
进程的表示：  
` os/src/procecss/process.rs `   
```Rust
/// PCB
pub struct Process {
    /// is user mode or not
    pub is_user: bool,
    /// page table, memory mapping
    pub memory_set: MemorySet,
}
```
线程调度器：  
` os/src/process/processor.rs `  
```Rust
#[derive(Default)]
pub struct Processor {
    /// current thread
    current_thread: Option<Arc<Thread>>,
    /// thread scheduler
    scheduler: SchedulerImpl<Arc<Thread>>,
    /// save sleeping threads
    sleeping_threads: HashSet<Arc<Thread>>,
}
```
#### 线程的创建
准备工作：  
+ 建立页表映射
+ 设置起始执行的地址
+ 初始化各种寄存器
+ 设置执行参数（可选）

修改` interrupt.asm `中的` __restore `，使得可以调用` __restore `进入新创建的线程：  
` os/src/interrupt/interrupt.asm `  
```Rust
__restore:
    mv      sp, a0  # 加入这一行
    # ...
```
然后设计好` Context `之后，只需要执行` __restore `就可以切换到第一个线程了。  
#### 线程的切换
线程切换的实现：  
` os/src/process/processor.rs `  
```Rust
/// activate `Context` of next thread
    pub fn prepare_next_thread(&mut self) -> *mut Context {
        loop {
            // ask for next thread from scheduler
            if let Some(next_thread) = self.scheduler.get_next() {
                // prepare next thread
                let context = next_thread.prepare();
                self.current_thread = Some(next_thread);
                return context;
            }
            else {
                // have no active threads
                if self.sleeping_threads.is_empty() {
                    // nor the sleeping threads, then panic
                    panic!("all threads terminated, shutting down...");
                }
                else  {
                    // have sleeping threads, waite for interrupt
                    crate::interrupt::wait_for_interrupt();        
                }
            }
        }
    }
```
上下文的保存和取出：  
` os/src/process/thread.rs `  
```Rust
/// stop thread when time interrupt occur, and save Context
    pub fn park(&self, context: Context) {
        // check context of current thread, should be None
        assert!(self.inner().context.is_none());
        // save Context in thread
        self.inner().context.replace(context);
    }
    
    /// prepare a process
    /// 
    /// activate page table and return Context
    pub fn prepare(&self) -> *mut Context {
        // activate page table
        self.process.write().memory_set.activate();
        // get Context
        let parked_frame = self.inner().context.take().unwrap();
        // push Context in kernel stack
        unsafe { KERNEL_STACK.push_context(parked_frame) }
    }
```
#### 线程的结束
内核线程将自己标记为”已结束“，同时触发一个` ebreak `异常。当操作系统观察到线程的标记，便将其终止。  
` os/src/main.rs `  
```Rust
/// 内核线程需要调用这个函数来退出
fn kernel_thread_exit() {
    // 当前线程标记为结束
    PROCESSOR.lock().current_thread().as_ref().inner().dead = true;
    // 制造一个中断来交给操作系统处理
    unsafe { llvm_asm!("ebreak" :::: "volatile") };
}
```
然后将这个函数作为内核线程的` ra `，使得它执行的函数完成后便执行` kernel_thread_exit() `  
` os/src/main.rs `
```Rust
/// 创建一个内核进程
pub fn create_kernel_thread(
    process: Arc<Process>,
    entry_point: usize,
    arguments: Option<&[usize]>,
) -> Arc<Thread> {
    // 创建线程
    let thread = Thread::new(process, entry_point, arguments).unwrap();
    // 设置线程的返回地址为 kernel_thread_exit
    thread.as_ref().inner().context.as_mut().unwrap()
        .set_ra(kernel_thread_exit as usize);
    thread
}
```
#### 内核栈
内核栈的实现方法：  
+ 留出一段空间作为内核栈
+ 运行线程时，在` sscratch `中保存内核栈栈顶指针
+ 如果线程遇到中断，则将` Context `压入` sscratch `指向的栈中，同时用新的栈地址来替换` sp `
+ 从中断返回时，` a0 `应指向在内核栈中的` Context `。出栈` Context `并且将栈顶保存在` sscratch `中。  

下面是内核栈的实现：  
` os/src/process/kernel_stack.rs `
```Rust
/// kernel stack
#[repr(align(16))]
#[repr(C)]
pub struct KernelStack([u8; KERNEL_STACK_SIZE]);

/// public kernel stack
pub static mut KERNEL_STACK: KernelStack = KernelStack([0; KERNEL_STACK_SIZE]);

impl KernelStack {
    /// push Context in stack and return top pointer
    pub fn push_context(&mut self, context: Context) -> *mut Context {
        // top of stack
        let stack_top = &self.0 as *const _ as usize + size_of::<Self>();
        // location of context
        let push_address = (stack_top - size_of::<Context>()) as *mut Context;
        unsafe {
            *push_address = context;
        }
        push_address
    }
}
```
#### 线程调度
和内存分配器同样在` os/src/algorithm/src/scheduler/mod.rs `中定义一个 trait 作为接口：  
` os/src/algorithm/src/scheduler/mod.rs `  
```Rust
/// 线程调度器
///
/// `ThreadType` 应为 `Arc<Thread>`
///
/// ### 使用方法
/// - 在每一个时间片结束后，调用 [`Scheduler::get_next()`] 来获取下一个时间片应当执行的线程。
///   这个线程可能是上一个时间片所执行的线程。
/// - 当一个线程结束时，需要调用 [`Scheduler::remove_thread()`] 来将其移除。这个方法必须在
///   [`Scheduler::get_next()`] 之前调用。
pub trait Scheduler<ThreadType: Clone + Eq>: Default {
    /// 向线程池中添加一个线程
    fn add_thread(&mut self, thread: ThreadType, priority: usize);
    /// 获取下一个时间段应当执行的线程
    fn get_next(&mut self) -> Option<ThreadType>;
    /// 移除一个线程
    fn remove_thread(&mut self, thread: &ThreadType);
    /// 设置线程的优先级
    fn set_priority(&mut self, thread: ThreadType, priority: usize);
}
```
本人在此基础上实现了` Stride Scheduler `，源码如下：  
` os/src/algorithm/src/scheduler/stride_scheduler.rs `  
```Rust
//! [`StrideScheduler`]
//pub const MAX_STRIDE: usize = 2usize.pow(32) - 1;
pub const MAX_STRIDE: usize = 4_294_967_295;
use super::Scheduler;
//use alloc::collections::LinkedList;
use alloc::vec::Vec;

pub struct ThreadBlock <ThreadType: Clone + Eq> {
    thread: ThreadType,
    pub priority: usize,
    pub stride: usize,    
}

impl <ThreadType: Clone + Eq> ThreadBlock <ThreadType> {
    fn new(thread: ThreadType, priority: usize, stride: usize) -> Self {
        Self {
            thread: thread,
            priority: priority,
            stride: stride,
        }
    }
    fn update_stride(&mut self) {
        if self.priority == 0 {
            self.stride = MAX_STRIDE;
        }
        else {
            self.stride += MAX_STRIDE / self.priority;
        }
    }
    fn set_priority(&mut self, priority: usize) {
        self.priority = priority;
    }
}

/// thread scheduler base on stride scheduling
pub struct StrideScheduler <ThreadType: Clone + Eq> {
    pool: Vec<ThreadBlock<ThreadType>>,
}

/// `Default` create a empty scheduler
impl<ThreadType: Clone + Eq> Default for StrideScheduler<ThreadType> {
    fn default() -> Self {
        Self {
            pool: Vec::new(),
        }
    }
}

impl <ThreadType: Clone + Eq> StrideScheduler <ThreadType> {
    fn get_min_stride_thread_index(&mut self) -> Option<usize> {
        if self.pool.is_empty() {
            return None;
        }
        let mut min_stride_thread_index = 0;
        for i in 0..self.pool.len() {
            if self.pool[i].stride < self.pool[min_stride_thread_index].stride {
                min_stride_thread_index = i;
            }
        }
        Some(min_stride_thread_index)
    }
}


impl<ThreadType: Clone + Eq> Scheduler<ThreadType> for StrideScheduler<ThreadType> {
    fn add_thread(&mut self, thread: ThreadType, priority: usize) {
        self.pool.push(
            ThreadBlock::new(thread, priority, 0)
        )
    }

    fn get_next(&mut self) -> Option<ThreadType> {
        if let Some(index) = self.get_min_stride_thread_index() {
            //self.pool[index].update_stride();
            //Some(self.pool[index].thread.clone())
            
            let mut threadblock = self.pool.remove(index);
            threadblock.update_stride();
            let next_thread = threadblock.thread.clone();
            self.pool.push(threadblock);
            Some(next_thread)
            
        }
        else {
            None
        }
    }

    fn remove_thread(&mut self, thread: &ThreadType) {
        let mut removed = self.pool.drain_filter(|t|&(t.thread) == thread);
        assert!(removed.next().is_some() && removed.next().is_none());
    }

    fn set_priority(&mut self, thread: ThreadType, priority: usize) {
        for threadblock in self.pool.iter_mut() {
            if threadblock.thread == thread {
                threadblock.set_priority(priority);
            }
        }
    }
}
```
### 小结
本次实验主要是理清线程和进程的概念，通过设置` Context `构造一个线程的状态抽象描述，实现内核栈和调度器。  
<span id="lab5"></span>
## Lab5
### 引言
文件系统是操作系统用于明确存储设备（常见的是磁盘，也有基于NAND Flash的固态硬盘）或分区上的文件的方法和数据结构；即在存储设备上组织文件的方法。  
### 实验内容
+ 设备树的概念和读取
+ virtio 总线协议
+ 块设备驱动的实现
+ 将块设备托管给文件系统
### 实验过程
#### 设备树
在 RISC-V 中，这接受设备信息一般是由 bootloader，即 OpenSBI 固件完成的。它来完成对于包括物理内存在内的各外设的扫描，将扫描结果以设备树二进制对象（DTB，Device Tree Blob）的格式保存在物理内存中的某个地方。  
我们通过调用学长们写好的` device_tree `库对设备树进行解析：  
` os/src/drivers/device_tree.rs `  
```Rust
/// recursive traverse device tree
fn walk(node: &Node) {
    // check and initialize
    if let Ok(compatible) = node.prop_str("compatible") {
        if compatible == "virtio,mmio" {
            virtio_probe(node);
        }
    }
    // 遍历子树
    for child in node.children.iter() {
        walk(child);
    }
}

/// Headers of Device Tree
struct DtbHeader {
    magic: u32,
    size: u32,
}

/// traverse device tree and initialize device
pub fn init(dtb_va: VirtualAddress) {
    let header = unsafe { &*(dtb_va.0 as *const DtbHeader) };
    // from_be 是大小端序的转换（from big endian）
    let magic = u32::from_be(header.magic);
    if magic == DEVICE_TREE_MAGIC {
        let size = u32::from_be(header.size);
        // 拷贝数据，加载并遍历
        let data = unsafe { slice::from_raw_parts(dtb_va.0 as *const u8, size as usize) };
        if let Ok(dt) = DeviceTree::load(data) {
            walk(&dt.root);
        }
    }
}

```
#### virtio
virtio 起源于 virtio: Towards a De-Facto Standard For Virtual I/O Devices 这篇论文，主要针对于半虚拟化技术中对通用设备的抽象。  
` virtio `节点探测：  
` os/src/drivers/bus/virtio_mmio.rs `  
```Rust
/// 从设备树的某个节点探测 virtio 协议具体类型
pub fn virtio_probe(node: &Node) {
    // reg 属性中包含了描述设备的 Header 的位置
    let reg = match node.prop_raw("reg") {
        Some(reg) => reg,
        _ => return,
    };
    let pa = PhysicalAddress(reg.as_slice().read_be_u64(0).unwrap() as usize);
    let va = VirtualAddress::from(pa);
    let header = unsafe { &mut *(va.0 as *mut VirtIOHeader) };
    // 目前只支持某个特定版本的 virtio 协议
    if !header.verify() {
        return;
    }
    // 判断设备类型
    match header.device_type() {
        DeviceType::Block => virtio_blk::add_driver(header),
        device => println!("unrecognized virtio device: {:?}", device),
    }
}
```
同样我们会使用学长写好的` virtio_drivers `库帮我们通过 MMIO 的方式对设备进行交互。  
` os/src/drivers/bus/virtio_mmio.rs `
```Rust
/// 为 DMA 操作申请连续 pages 个物理页（为 [`virtio_drivers`] 库提供）
///
/// 为什么要求连续的物理内存？设备的 DMA 操作只涉及到内存和对应设备
/// 这个过程不会涉及到 CPU 的 MMU 机制，我们只能给设备传递物理地址
/// 而陷于我们之前每次只能分配一个物理页的设计，这里我们假设我们连续分配的地址是连续的
#[no_mangle]
extern "C" fn virtio_dma_alloc(pages: usize) -> PhysicalAddress {
    let mut pa: PhysicalAddress = Default::default();
    let mut last: PhysicalAddress = Default::default();
    for i in 0..pages {
        let tracker: FrameTracker = FRAME_ALLOCATOR.lock().alloc().unwrap();
        if i == 0 {
            pa = tracker.address();
        } else {
            assert_eq!(last + PAGE_SIZE, tracker.address());
        }
        last = tracker.address();
        TRACKERS.write().insert(last, tracker);
    }
    pa
}

/// 为 DMA 操作释放对应的之前申请的连续的物理页（为 [`virtio_drivers`] 库提供）
#[no_mangle]
extern "C" fn virtio_dma_dealloc(pa: PhysicalAddress, pages: usize) -> i32 {
    for i in 0..pages {
        TRACKERS.write().remove(&(pa + i * PAGE_SIZE));
    }
    0
}

/// 将物理地址转为虚拟地址（为 [`virtio_drivers`] 库提供）
///
/// 需要注意，我们在 0xffffffff80200000 到 0xffffffff88000000 是都有对应的物理地址映射的
/// 因为在内核重映射的时候，我们已经把全部的段放进去了
/// 所以物理地址直接加上 Offset 得到的虚拟地址是可以通过任何内核进程的页表来访问的
#[no_mangle]
extern "C" fn virtio_phys_to_virt(pa: PhysicalAddress) -> VirtualAddress {
    VirtualAddress::from(pa)
}

/// 将虚拟地址转为物理地址（为 [`virtio_drivers`] 库提供）
///
/// 需要注意，实现这个函数的目的是告诉 DMA 具体的请求，请求在实现中会放在栈上面
/// 而在我们的实现中，栈是以 Framed 的形式分配的，并不是高地址的线性映射 Linear
/// 为了得到正确的物理地址并告诉 DMA 设备，我们只能查页表
#[no_mangle]
extern "C" fn virtio_virt_to_phys(va: VirtualAddress) -> PhysicalAddress {
    Mapping::lookup(va).unwrap()
}

```
本身设备是通过直接内存访问DMA（Direct Memory Access）技术来实现数据传输的，CPU 只需要给出要传输哪些内容，放在哪段物理内存上面，把请求告诉设备，设备后面的操作就会利用 DMA 而不经过 CPU 直接传输。  
#### 驱动和块设备驱动
对驱动进行抽象：  
` os/src/drivers/driver.rs `  
```Rust
/// type of device
///
/// 目前只有块设备，可能还有网络、GPU 设备等
#[derive(Debug, Eq, PartialEq)]
pub enum DeviceType {
    Block,
}

/// 驱动的接口
pub trait Driver: Send + Sync {
    /// type of device
    fn device_type(&self) -> DeviceType;

    /// 读取某个块到 buf 中（块设备接口）
    fn read_block(&self, _block_id: usize, _buf: &mut [u8]) -> bool {
        unimplemented!("not a block driver")
    }

    /// 将 buf 中的数据写入块中（块设备接口）
    fn write_block(&self, _block_id: usize, _buf: &[u8]) -> bool {
        unimplemented!("not a block driver")
    }
}
```
对块设备进行抽象：  
` os/src/drivers/block/mod.rs `  
```Rust
pub struct BlockDevice(pub Arc<dyn Driver>);

/// 为 [`BlockDevice`] 实现 [`rcore-fs`] 中 [`BlockDevice`] trait
///
/// 使得文件系统可以通过调用块设备的该接口来读写
impl dev::BlockDevice for BlockDevice {
    /// 每个块的大小（取 2 的对数）
    ///
    /// 这里取 512B 是因为 virtio 驱动对设备的操作粒度为 512B
    const BLOCK_SIZE_LOG2: u8 = 9;

    /// read a block to buf
    fn read_at(&self, block_id: usize, buf: &mut [u8]) -> dev::Result<()> {
        match self.0.read_block(block_id, buf) {
            true => Ok(()),
            false => Err(dev::DevError),
        }        
    }

    /// write data from buf to block
    fn write_at(&self, block_id: usize, buf: &[u8]) -> dev::Result<()> {
        match self.0.write_block(block_id, buf) {
            true => Ok(()),
            false => Err(dev::DevError),
        }
    }

    /// 执行和设备的同步
    ///
    /// 因为我们这里全部为阻塞 I/O 所以不存在同步的问题
    fn sync(&self) -> dev::Result<()> {
        Ok(())
    }
}
```
实现 virtio_blk 驱动  
` os/src/drivers/block/virtio_blk.rs `  
```Rust
/// virtio 协议的块设备驱动
struct VirtIOBlkDriver(Mutex<VirtIOBlk<'static>>);

/// 为 [`VirtIOBlkDriver`] 实现 [`Driver`] trait
///
/// 调用了 [`virtio_drivers`] 库，其中规定的块大小为 512B
impl Driver for VirtIOBlkDriver {
    /// type of device
    fn device_type(&self) -> DeviceType {
        DeviceType::Block
    }

    /// read a block to buf
    fn read_block(&self, block_id: usize, buf: &mut [u8]) -> bool {
        self.0.lock().read_block(block_id, buf).is_ok()
    }

    /// write data in buf to block
    fn write_block(&self, block_id: usize, buf: &[u8]) -> bool {
        self.0.lock().write_block(block_id, buf).is_ok()
    }
}

/// 将从设备树中读取出的设备信息放到 [`static@DRIVERS`] 中
pub fn add_driver(header: &'static mut VirtIOHeader) {
    let virtio_blk = VirtIOBlk::new(header).expect("faild to init block driver");
    let driver = Arc::new(VirtIOBlkDriver(Mutex::new(virtio_blk)));
    DRIVERS.write().push(driver);
}
```
#### 文件系统
我们这里用了 rCore 中的文件系统模块` rcore-fs `，选择其中最简单的 Simple File System。  
` os/src/fs/mod.rs `  
```Rust
lazy_static! {
    /// 根文件系统的根目录的 INode
    pub static ref ROOT_INODE: Arc<dyn INode> = {
        // 选择第一个块设备
        for driver in DRIVERS.read().iter() {
            if driver.device_type() == DeviceType::Block {
                let device = BlockDevice(driver.clone());
                // 动态分配一段内存空间作为设备 Cache
                let device_with_cache = Arc::new(BlockCache::new(device, BLOCK_CACHE_CAPACITY));
                return SimpleFileSystem::open(device_with_cache)
                    .expect("failed to open SFS")
                    .root_inode();
            }
        }
        panic!("failed to load fs")
    };
}
```
### 小结
本次实验主要是在 QEMU 上挂载了存储设备，并实现了 virtio 驱动和一个简单的文件系统。  
<span id="lab6"></span>

## Lab6
### 引言
本次实验将为用户搭建程序开发框架。  
### 实验内容
+ 单独生成 ELF 格式的用户程序，并打包进文件系统中
+ 创建并运行用户进程
+ 使用系统调用为用户程序提供服务
### 实验过程
#### 构建用户程序框架
在` os `的旁边建立一个` user `crate，并移除默认的` main.rs `，在` src `目录下建立` lib.rs `和` bin `子目录。  
目录结构：  
```
rCore-Tutorial
  - os
  - user
    - src
      - bin
        - hello_world.rs
      - lib.rs
    - Cargo.toml
```
为用户程序移除 std 依赖，并补充一些必要的功能：  
+ ` lib.rs `中`#![no_std]`移除标准库
+ ` lib.rs `中`#![feature(...)]`开启一些不稳定的功能
+ ` lib.rs `中`#[global_allocator]`使用库来实现动态内存分配
+ ` lib.rs `中`#[panic_handler] panic`时终止
+ `.cargo/config`设置编译目标为 RISC-V 64
+ `console.rs`实现 print! println! 宏
#### 打包为磁盘镜像
安装` rcore-fs-fuse `工具：  
```
cargo install rcore-fs-fuse --git https://github.com/rcore-os/rcore-fs
```
通过这个工具将一个目录打包成 Simple File System 格式的磁盘镜像。  
下面将编译得到的 ELF 文件单独放在一个导出目录中：  
```Makefile
build: dependency
    # 编译
    @cargo build
    @echo Targets: $(patsubst $(SRC_DIR)/%.rs, %, $(SRC_FILES))
    # 移除原有的所有文件
    @rm -rf $(OUT_DIR)
    @mkdir -p $(OUT_DIR)
    # 复制编译生成的 ELF 至目标目录
    @cp $(BIN_FILES) $(OUT_DIR)
    # 使用 rcore-fs-fuse 工具进行打包
    @rcore-fs-fuse --fs sfs $(IMG_FILE) $(OUT_DIR) zip
    # 将镜像文件的格式转换为 QEMU 使用的高级格式
    @qemu-img convert -f raw $(IMG_FILE) -O qcow2 $(QCOW_FILE)
    # 提升镜像文件的容量（并非实际大小），来允许更多数据写入
    @qemu-img resize $(QCOW_FILE) +1G
```
#### 解析ELF文件并创建线程
我们这里利用了` xmas-elf `解析器  
读取文件内容：  
` os/src/fs/inode_ext.rs `
```Rust
fn readall(&self) -> Result<Vec<u8>> {
        // 从文件头读取长度
        let size = self.metadata()?.size;
        // 构建 Vec 并读取
        let mut buffer = Vec::with_capacity(size);
        unsafe { buffer.set_len(size) };
        self.read_at(0, buffer.as_mut_slice())?;
        Ok(buffer)
    }
```
解析各个字段：  
` os/src/memory/mapping/memory_set.rs `  
```Rust
/// 通过 elf 文件创建内存映射（不包括栈）
    // todo: 有可能不同的字段出现在同一页？
    pub fn from_elf(file: &ElfFile, is_user: bool) -> MemoryResult<MemorySet> {
        // 建立带有内核映射的 MemorySet
        let mut memory_set = MemorySet::new_kernel()?;

        // 遍历 elf 文件的所有部分
        for program_header in file.program_iter() {
            if program_header.get_type() != Ok(Type::Load) {
                continue;
            }
            // 从每个字段读取「起始地址」「大小」和「数据」
            let start = VirtualAddress(program_header.virtual_addr() as usize);
            let size = program_header.mem_size() as usize;
            let data: &[u8] =
                if let SegmentData::Undefined(data) = program_header.get_data(file).unwrap() {
                    data
                } else {
                    return Err("unsupported elf format");
                };

            // 将每一部分作为 Segment 进行映射
            let segment = Segment {
                map_type: MapType::Framed,
                range: Range::from(start..(start + size)),
                flags: Flags::user(is_user)
                    | Flags::readable(program_header.flags().is_read())
                    | Flags::writable(program_header.flags().is_write())
                    | Flags::executable(program_header.flags().is_execute()),
            };

            // 建立映射并复制数据
            memory_set.add_segment(segment, Some(data))?;
        }

        Ok(memory_set)
    }
```
#### 实现系统调用
系统调用总入口：  
` os/src/kernel/syscall.rs `   
```Rust
/// 系统调用的总入口
pub fn syscall_handler(context: &mut Context) -> *mut Context {
    // 无论如何处理，一定会跳过当前的 ecall 指令
    context.sepc += 4;

    let syscall_id = context.x[17];
    let args = [context.x[10], context.x[11], context.x[12]];

    let result = match syscall_id {
        SYS_READ => sys_read(args[0], args[1] as *mut u8, args[2]),
        SYS_WRITE => sys_write(args[0], args[1] as *mut u8, args[2]),
        SYS_EXIT => sys_exit(args[0]),
        _ => unimplemented!(),
    };

    match result {
        SyscallResult::Proceed(ret) => {
            // 将返回值放入 context 中
            context.x[10] = ret as usize;
            context
        }
        SyscallResult::Park(ret) => {
            // 将返回值放入 context 中
            context.x[10] = ret as usize;
            // 保存 context，准备下一个线程
            PROCESSOR.get().park_current_thread(context);
            PROCESSOR.get().prepare_next_thread()
        }
        SyscallResult::Kill => {
            // 终止，跳转到 PROCESSOR 调度的下一个线程
            PROCESSOR.get().kill_current_thread();
            PROCESSOR.get().prepare_next_thread()
        }
    }
}
```
我们会利用文件的统一接口` INode `，使用其中的` read_at() `和` write_at() `接口实现读写系统调用。  
### 小结
这次实验主要是成功单独生成了 ELF 格式的用户程序，并打包进文件系统中，同创建并运行了用户进程。另外，我们还实现了一些系统调用为用户程序提供服务。  