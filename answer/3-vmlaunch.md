阅读代码，详细阐述：
- 在 VM-entry 和 VM-exit 的过程中，Host 的 RSP 寄存器的值是如何变化的？包括：哪些指令
- 在 VM-entry 和 VM-exit 的过程中，Guest 的通用寄存器的值是如何保存又如何恢复的？（提示：与RSP的修改有关）
wm_launch:
```
"mov    [rdi + {host_stack_top}], rsp", // 将当前 RSP 保存到 Vcpu::host_stack_top
"mov    rsp, rdi",                      // 将 RSP 设置为 guest regs area
restore_regs_from_stack!(),             // 加载 guest 寄存器内容
"vmlaunch",
```

vmexit:
```
save_regs_to_stack!(),                  // 保存寄存器内容
"mov    r15, rsp",                      // save temporary RSP to r15
"mov    rdi, rsp",                      // set the first arg to &Vcpu
"mov    rsp, [rsp + {host_stack_top}]", // set RSP to Vcpu::host_stack_top
"call   {vmexit_handler}",              // call vmexit_handler
"mov    rsp, r15",                      // load temporary RSP from r15
restore_regs_from_stack!(),             // 加载寄存器内容
"vmresume",
"jmp    {failed}",
```

- VM-exit 过程中是如何确保调用 vmexit_handler 函数时栈是可用的？

栈地址保存在 Vcpu::host_stack_top