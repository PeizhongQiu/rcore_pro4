arceos/crates/hypercraft/src/hal.rs

HyperCraftHal 特征操作：分配和回收页面，host 物理地址和虚拟地址转换，获取当前时间，vmexit_handler。

实现该特征的数据结构：HyperCraftHalImpl

arceos/modules/axruntime/src/hv/mod.rs

操作：使用 axalloc::global_allocator 完成分配和回收页面，物理地址和虚拟地址转换是线性转换，vmexit_handler 调用 vmx::vmexit_handler，获取时间调用 axhal::time::current_time_nanos()。
