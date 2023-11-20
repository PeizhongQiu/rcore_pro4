阅读 Intel SDM Vol. 3C, Chapter 25: Virtual-Machine Control Structures 相关小节，回答以下问题：

如果让要 hypervisor 实现以下功能，应该如何配置 VMCS？
- 拦截 Guest HLT 指令

“HLT exiting” VM-execution control is 1.

- 拦截 Guest PAUSE 指令

the “PAUSE exiting” VM-execution control is 1,

- 拦截外部设备产生的中断，而不是直通给 Guest

External-interrupt exiting 置 1

- 打开或关闭 Guest 的中断

Guest-state area中的RFLAGS选项

- 拦截 Guest 缺页异常 (#PF)

Whether a page fault (exception with vector 14) causes a VM exit is determined by bit 14 in the exception bitmap as well as the error code produced by the page fault and two 32-bit fields in the VMCS (the page-fault error-code mask and page-fault error-code match).

- 拦截所有 Guest I/O 指令 (x86 IN/OUT/INS/OUTS 等)

Unconditional I/O exiting is 1

- *只拦截 Guest 对串口的 I/O 读写 (I/O 端口为 0x3f8)

“use I/O bitmaps” control is 1

I/O bitmaps A I/O 端口为 0x3f8 为 1

- 拦截所有 Guest MSR 读写

1-setting of the “use MSR bitmaps” VM-execution control

MSR bitmaps 全部置1

- *只拦截 Guest 对 IA32_EFER MSR 的写入

1-setting of the “use MSR bitmaps” VM-execution control

MSR bitmaps 中 Write bitmap 中 IA32_EFER 对应位为 1

- *只拦截 Guest 对 CR0 控制寄存器 PG 位 (31 位) 的写入

CR0_GUEST_HOST_MASK 设置为 0x80000000

*如果要在单核 hypervisor 中交替运行两个 vCPU，应该如何操作 VMCS？
```
vmclear A
vmclear B

vmptrld A
vmlaunch A 
...
vmptrld B
vmlaunch B
```
