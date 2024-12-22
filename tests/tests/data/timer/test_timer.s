    .section .text
    .globl _start
    .globl timer_handler

_start:
    # Step 1: Initialize mtvec to point to the timer_handler
    la t0, timer_handler  # Load the address of timer_handler
    csrw mtvec, t0        # Set mtvec to the handler address

    # Step 2: Load base addresses for mtime and mtimecmp
    li t0, 0x200bff8      # Load mtime address into t0
    li t1, 0x2004000      # Load mtimecmp address into t1

    # Step 3: Read the current time from mtime
    lw a0, 0(t0)          # Load lower 4 bytes of mtime into a0
    lw a1, 4(t0)          # Load upper 4 bytes of mtime into a1

    # Step 4: Set mtimecmp = mtime + OFFSET
    li t2, 20         # Set timer offset (adjust as needed)
    add a0, a0, t2        # Add offset to lower 32 bits
    sw a0, 0(t1)          # Store lower 4 bytes to mtimecmp
    sw a1, 4(t1)          # Store upper 4 bytes to mtimecmp

    # Step 5: Enable timer interrupt in mie
    csrr t0, mie          # Read current mie value into t0
    li t1, 0x80           # Load bit mask for MTIE (0x80)
    or t0, t0, t1         # Set the 7th bit (MTIE)
    csrw mie, t0          # Write back the updated value to mie

    # Step 6: Enable global interrupts in mstatus
    csrr t0, mstatus      # Read current mstatus value into t0
    li t1, 0x8            # Load bit mask for MIE (0x8)
    or t0, t0, t1         # Set the 3rd bit (MIE)
    csrw mstatus, t0      # Write back the updated value to mstatus

    # Initialize the interrupt flag
    la t3, interrupt_flag
    li t4, 0              # Set flag to 0 (not triggered)
    sw t4, 0(t3)

wait_loop:
    # Check if interrupt_flag is set
    lw t4, 0(t3)
    beqz t4, wait_loop    # If flag is 0, continue waiting

    # Exit the program with success
    li a0, 0              # Exit code 0 (success)
    li a7, 93             # ecall number for exit
    ecall

error_exit:
    # Exit the program with error
    li a0, 1              # Exit code 1 (error)
    li a7, 93             # ecall number for exit
    ecall

# Timer interrupt handler
timer_handler:
    # Step 7: Set interrupt_flag to 1
    la t3, interrupt_flag
    li t4, 1              # Set flag to 1 (triggered)
    sw t4, 0(t3)

    # Step 8: Acknowledge the interrupt (clear mtimecmp to avoid retriggering)
    li t0, 0
    li t1, 0x2004000      # Load mtimecmp address into t1
    sw t0, 0(t1)          # Write max value to lower 4 bytes of mtimecmp
    sw t0, 4(t1)          # Write max value to upper 4 bytes of mtimecmp

    mret                  # Return from the interrupt

# Memory-mapped addresses
    .section .data
interrupt_flag:
    .word 0              # Initially 0, set to 1 when interrupt occurs
