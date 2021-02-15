leaf:
    addi sp, sp, -8; # sp = sp + -8
    sw s1, 4(sp) # 
    sw s0, 0(sp) # 
    

    lw s0, 0(sp) # 
    lw s1, 4(sp) # 
    addi sp, sp, 8; # sp = sp + 8
    ret