.global asm_trap_vector
# This will be our trap vector when we start
# handling interrupts.
asm_trap_vector:
	csrr	a0, mtval
	wfi
	mret
