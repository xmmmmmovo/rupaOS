# 使用宏来循环保存寄存器，必要设置
.altmacro

# 寄存器宽度对应字节数
.set    REG_SIZE, 8
# Context 大小
.set    CONTEXT_SIZE, 34

# 宏：将寄存器保存到栈上
.macro SAVE reg, offset
    sd \reg, \offset*8(sp)
.endm

.macro SAVE_N n
    SAVE  x\n, \n
.endm


# 宏：将寄存器从栈中取出
.macro LOAD reg, offset
    ld  \reg, \offset*8(sp)
.endm

.macro LOAD_N n
    LOAD  x\n, \n
.endm

    .section .text
    .global __interrupt

# 进入中断
# 保存Context并且进入Rust中的中断处理函数
__interrupt:
    # 在栈上开辟Context需要的空间
    addi sp, sp, -34*8; # sp = sp + -34*8
    
    # 保存通用寄存器
    SAVE x1, 1

    # 将原来的sp写入2的位置
    addi x1, sp, 34*8; # x1 = sp + 34*8
    SAVE x1, 2

    # 保存x3-x31
    .set n, 3
    .rept 29
        SAVE_N %n
        .set n, n+1
    .endr

    # 调用handle_interrupt
    # context: &mut Context
    mv  a0, sp # a0 = sp
    # scause: Scause
    csrr a1, scause
    # stval: usize
    csrr a2, stval
    jal  handle_interrupt

    .globl __restore
# 离开中断
# 从 Context 中恢复所有寄存器，并跳转至 Context 中 sepc 的位置
__restore:
    # 恢复CSR
    LOAD s1, 32
    LOAD s2, 33
    csrw sstatus, s1
    csrw sepc, s2

    # 恢复通用寄存器
    LOAD x1, 1
    # 恢复 x3 至 x31
    .set n, 3
    .rept 29
        LOAD_N %n
        .set n, n + 1
    .endr

    # 恢复 sp（又名 x2）这里最后恢复是为了上面可以正常使用 LOAD 宏
    LOAD x2, 2
    sret