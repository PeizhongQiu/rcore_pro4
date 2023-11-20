arceos/crates/hypercraft/src/arch/x86_64/vmx/region.rs

https://zhuanlan.zhihu.com/p/49400702

对于Intel x86处理器，在打开VMX（Virtual Machine Extension），即执行VMXON指令的时候需要提供一个4KB对齐的内存区间，称作VMXON region，该区域的物理地址作为VMXON指令的操作数。该内存区间用于支持逻辑CPU的VMX功能，该区域在VMXON和VMXOFF之间一直都会被VMX硬件所使用。

VMXON Region内存区域的要求跟VMCS类似，也是需要4KB对齐，并且在使用之前，需要先初始化一下VMCS revision，即从IA32_VMX_BASIC MSR寄存器中获取到的31bit的VMCS revision identifier。对于每个支持VMX功能的逻辑CPU而言，都需要一个相应的VMXON Region。

对于软件而言，除了在执行VMXON之前初始化一下VMCS的版本号之外，不需要再做其他事情，特别是在VMXON和VMXOFF之间，软件不应该访问，甚至是更改VMXON Region这段内存，否则可能会造成不可预知的后果。

在KVM模块中，KVM的模块加载过程vmx_init()中，会调用到alloc_kvm_area()函数，为每个逻辑CPU分配一个VMXON Region。

注意：VMXON Region和VMCS Region是不一样的两个内存区域，VMXON是针对逻辑CPU的，每个逻辑CPU都会有一份，并且在整个VMX功能使用期间硬件都会使用；而VMCS Region则是针对vCPU的，每个vCPU都会有一份VMCS Region，用于辅助硬件对vCPU的模拟。

VMXON Region看样子完全是为了满足VMX功能在硬件上的需求，里面具体有什么数据，Intel并没有公开，软件要做的就是分配一个这样的数据区，标明VMCS revision，让后扔该硬件自个儿玩去，VMM没有必要也没有资格管。

VmxRegion：该数据结构会作为 VMCS Region 和 VMXON Region 两种。
- frame：分配的页面
- 操作：
    - uninit：设置分配的页面地址为 0，表明页面还未初始化
    - new：分配一个页面，且页面全部初始化为 0，并设置 VMCS revision identifier
    - phys_addr：获取分配页面的物理地址


arceos/crates/hypercraft/src/arch/x86_64/vmx/vmcs.rs
- CR0_GUEST_HOST_MASK，CR0_READ_SHADOW，CR4_GUEST_HOST_MASK 和 CR4_READ_SHADOW
    - VM 执行控制字段包括 CR0_GUEST_HOST_MASK，CR0_READ_SHADOW，CR4_GUEST_HOST_MASK 和 CR4_READ_SHADOW。 这些字段控制访问这些寄存器的指令的执行（包括 CLTS、LMSW、MOV CR 和 SMSW）。 它们在支持 Intel 64 架构的处理器上为 64 位，在不支持 Intel 64 架构的处理器上为 32 位。
    - 一般来说，GUEST_HOST_MASK 中设置为 1 的位对应于 HOST “拥有”的位：
        - GUEST 尝试将它们（使用CLTS、LMSW 或MOV 到CR）设置为与相应读取影子中相应位不同的值，从而导致 VM exit。
        - GUEST 读取（使用CR 或SMSW 中的MOV）从相应的读取影子中返回这些位的值。
    - GUEST_HOST_MASK 中设置为 0 的位的位对应于 GUEST “拥有”的位；GUEST尝试修改它们会成功，并且 GUEST 从控制寄存器本身读取这些位的返回值。

```Rust
pub fn set_control( control, capability_msr, old_value, set, clear):
    let cap = capability_msr.read();
    let allowed0 = cap as u32;
    let allowed1 = (cap >> 32) as u32;
    // SDM Vol. 3C, Section 31.5.1, Algorithm 3
    let flexible = !allowed0 & allowed1; // therse bits can be either 0 or 1
    let unknown = flexible & !(set | clear); // hypervisor untouched bits
    let default = unknown & old_value; // these bits keep unchanged in old value
    let fixed1 = allowed0; // these bits are fixed to 1
    control.write(fixed1 | default | set)?;
```