    .section .text.init,"ax",@progbits
    .globl main                  # Declare the main function
    .globl _start
_start:
  # Initialize global pointer
.option push
.option norelax
1:auipc gp, %pcrel_hi(__global_pointer$)
  addi  gp, gp, %pcrel_lo(1b)
.option pop

    # Initialize stack pointer (SP)
    la sp, _stack_top            # Load the stack top address into SP
    
    # Set the trap vector address
    la t0, freertos_risc_v_trap_handler
    csrw mtvec, t0

    # Clear the BSS section (all variables in .bss should be zeroed)
    la t0, _bss_start            # Load start address of .bss into t0
    la t1, _bss_end              # Load end address of .bss into t1
    li t2, 0                     # Load 0 into t2 (the value to clear)

clear_bss:
    bgeu t0, t1, bss_cleared     # If t0 >= t1, jump to bss_cleared
    sw t2, 0(t0)                 # Store 0 to memory at address t0
    addi t0, t0, 4               # Increment t0 (move to the next word)
    j clear_bss                  # Repeat until the whole .bss section is cleared

bss_cleared:
    call main                    # Call the main function
    
    li a7, 93                    # Load the syscall NO into a7 (93 for exit)
    ecall

    # Infinite loop if main returns (bare-metal systems typically don't return from main)
hang:
    j hang                       # Infinite loop to prevent falling off the edge
