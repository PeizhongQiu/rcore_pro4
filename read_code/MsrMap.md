arceos/crates/hypercraft/src/arch/x86_64/vmx/region.rs

MsrBitmap：https://www.cnblogs.com/haiyonghao/p/14440954.html

在支持“use MSR bitmaps” 的 VM 执行控制设置为 1 的处理器上，VM 执行控制字段包括四个连续 MSR 位图的 64 位物理地址，每个位图大小为 1 KB。 该字段在不支持该控件的 1 设置的处理器上不存在。 这四个位图是：
- 读取低MSR 的位图（位于MSR 位图地址）。 这对于 00000000H 到 00001FFFH 范围内的每个 MSR 地址包含一位。 该位确定应用于该 MSR 的 RDMSR 的执行是否会导致 VM 退出。
- 读取高MSR 的位图（位于MSR 位图地址加1024 处）。 这对于 C0000000H 到 C0001FFFH 范围内的每个 MSR 地址包含一位。 该位确定应用于该 MSR 的 RDMSR 的执行是否会导致 VM 退出。
- 写入低MSR 位图（位于MSR 位图地址加2048 处）。 这对于 00000000H 到 00001FFFH 范围内的每个 MSR 地址包含一位。 该位确定应用于该 MSR 的 WRMSR 执行是否会导致 VM 退出。
- 写入高MSR 位图（位于MSR 位图地址加3072 处）。 这对于 C0000000H 到 C0001FFFH 范围内的每个 MSR 地址包含一位。 该位确定应用于该 MSR 的 WRMSR 执行是否会导致 VM 退出。
当且仅当“use MSR bitmaps”控制为 1 时，逻辑处理器才使用这些位图。如果使用位图，并且 RCX 的值不在该位图覆盖的范围内，则执行 RDMSR 或 WRMSR 会导致 VM 退出或 MSR 位图中的相应位（对应于指令和 RCX 值）是否为 1。如果使用位图，则它们的地址必须按 4 KB 对齐。

MsrBitmap
- frame：分配的页面
- 操作
    - passthrough_all：页面初始化为 0，即通过所有，不会引发 VMexit
    - intercept_all：页面初始化为 1，即拦截所有，所有访问和写 msr 引发 VMexit
    - set_intercept(msr, is_write, intercept)：根据 msr 和 is_write 确定修改的 msr 位置，然后设置为 intercept。
    - set_read_intercept：调用 set_intercept(msr, false, intercept); 设置 msr 读的位置为 intercept。
    - set_write_intercept：调用 set_intercept(msr, true, intercept); 设置 msr 写的位置为 intercept。