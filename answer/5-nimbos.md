1、结合之前学习到的知识，解释 Guest BIOS 核心代码中 prot_gdt 和 prot_gdt_desc 都是什么内容。

全局描述符表和段描述符寄存器

2、修改代码，使分配给 Guest OS 的内存容量从 16 MB 增加到 32 MB。

修改 arceos/apps/hv/src/x64.rs 中 pub const GUEST_PHYS_MEMORY_SIZE: usize = 0x100_0000; // 16M 为 0x1000_0000

3、简述：如果要使 NimbOS OS 被加载的地址从0x20_0000更改到其他地址，需要做哪些修改？

修改 arceos/apps/hv/src/x64.rs 中 pub const GUEST_ENTRY: GuestPhysAddr = 0x20_0000;
修改 bios arceos/apps/hv/guest/nimbos/bios/boot16.S 中 mov     ecx, 0x200000       # kernel entry