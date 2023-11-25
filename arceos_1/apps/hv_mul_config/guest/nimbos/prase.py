# coding:utf-8
import yaml
import os
import struct

# 获取当前脚本所在文件夹路径
curPath = os.path.dirname(os.path.realpath(__file__))
# 获取yaml文件路径
yamlPath = os.path.join(curPath, "config.yml")

# open方法打开直接读出来
f = open(yamlPath, 'r', encoding='utf-8')
cfg = f.read()
print(type(cfg))  # 读出来是字符串
print(cfg)

d = yaml.load(cfg)  # 用load方法转字典
print(d)
print(type(d))

merged_file = open("config.bin", "wb")
merged_file.truncate(4*1024)


merged_file.seek(0, 0)
merged_file.write(struct.pack('<Q', d['vm_count']))

for i in range(0,d['vm_count']):
    merged_file.write(struct.pack('<Q', d['vm'+str(i)]['id']))
    merged_file.write(struct.pack('<Q', d['vm'+str(i)]['memory']))
    merged_file.write(struct.pack('<Q', d['vm'+str(i)]['vcpu_count']))
    merged_file.write(struct.pack('<Q', d['vm'+str(i)]['io_apic']))
    merged_file.write(struct.pack('<Q', d['vm'+str(i)]['HPET']))
    merged_file.write(struct.pack('<Q', d['vm'+str(i)]['local_apic']))

merged_file.close()