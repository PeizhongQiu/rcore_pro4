阅读代码，描述在使能 VMX 的过程中 vmx_region 是如何分配和初始化的。（使能 VMX 的过程在 crates/hypercraft/src/arch/x86_64/vmx/percpu.rs 中的 VmxPerCpuState::<H>::hardware_enable 函数）

1、首先访问 IA32_VMX_BASIC MSR 获取 vmcs_revision_id；
2、分配一个物理页面，前面 4 个字节为 vmcs_revision_id，其余部分为0；
3、将分配页面的物理地址作为参数传递给 vmxon。