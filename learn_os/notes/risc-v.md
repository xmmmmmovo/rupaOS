```assembly
add x1, x2, x3; # x1 = x2 + x3
sub x1, x2, x3 # x1 = x2 - x3
addi x3, x4, 10; # x3 = x4 + 10

lw x10, 12(x15) # load word
add x11, x12, x10; # x11 = x12 + x10
sw x10, 40(x15) # save word

lb t1, 0(s1) # load bit
sb t1, 0(s1) # save bit
lbu t1, 0(s1); # save bit with zero


beq t0, t1, target; # if t0 == t1 then target
bne t0, t1, target # if t0 != t1 then target
blt t0, t1, target # if t0 < t1 then target
bltu t0, t1, target # if t0 < t1 then target
bge t0, t1, target # if t0 >= t1 then target

j target  # jump to target

and x5, x6, x7
andi x5, x6, 3
andi with 0000 00FF
or
xor
sll # 左移
srl # 右移

# xor 0x1111_1111 逻辑非

sra # 右移n位后空出的高位由原数最高比特位的符号扩展得到

mv rd, rd # rd = rd
li rd, 13 # rd = 13

# a0 - a7 -> x10 - x17 用来向调用的函数传递参数 
# a0和a1寄存器常用来传递返回值
# ra 即x1寄存器 用来保存返回时的返回地址值
# s0 - s11 对应编号x8 - x9, x18 - x27的寄存器用来作为保存寄存器
# 保存原进程中的关键数据避免在函数调用过程中被破坏

jal sum  # jump to sum and save position to ra
jr ra   # jump to ra 寄存器值所对应的地址空间
ret     # = jr ra

# sp寄存器 栈指针寄存器

# 调用的函数---caller 被调用的函数---callee

# 函数调用时保留的寄存器
# 被调用函数不会使用这些寄存器，使用时也会保存好原值
# sp gp tp寄存器和s0-s11寄存器

# 不保存的
# a0-a7 ra t0-t6
```

![image-20210209005610865](https://cdn.jsdelivr.net/gh/xmmmmmovo/ResourcesBackup/blog/image-20210209005610865.png)

调用时候保存的寄存器值：原返回地址，参数寄存器值，保留寄存器值，局部变量值。

静态区，堆区，栈区

