add x9, x8, x0
add x10, x0, x0
add x11, x0, x0
addi x13, x0, 20

loop:
    bge x11, x13, break # if x11 >= x13 then break
    lw x12, 0(x9)
    add x10, x10, x12
    addi x9, x9, 4
    addi x11, x11, 1
    j loop  # jump to loop
break: