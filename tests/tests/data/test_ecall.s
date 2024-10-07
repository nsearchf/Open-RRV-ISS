.section .text
.global _start

# Entry point of the program
_start:
    # Load the address of the ecall handler
    la t0, ecall_handler
    csrw mtvec, t0

    # Set up a value in a register for testing
    li a0, 52       # Load immediate value 42 into a0

    ecall

    li a0, 54
    # Infinite loop to halt the program
    j _start

ecall_handler:
    li t0, 52       # Expected value in a0 for our test
    beq a0, t0, ecall_ret

ecall_error:
    li a0, 1
    j ecall_error

ecall_ret:
    csrr t1, mepc
    addi t1, t1, 4
    csrw mepc, t1
    li a0, 53
    mret
