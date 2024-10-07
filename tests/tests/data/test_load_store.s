.section .data
# Define a data section with some initial values
data1:  .word 0x12345678  # 32-bit word
data2:  .word 0x9abcdef0  # 32-bit word
data3:  .byte 0x12        # 8-bit byte
pad:    .byte 0           # 8-bit byte
data4:  .half 0x3456      # 16-bit halfword

.section .text
.globl _start
_start:
    # Load the address of the data section into register x5
    la x5, data1            # x5 = address of data1

    # Load the value at data1 into register x6
    lw x6, 0(x5)            # x6 = *data1

    # Load the value at data2 into register x7
    lw x7, 4(x5)            # x7 = *data2

    # Load the byte at data3 into register x8
    lb x8, 8(x5)            # x8 = *data3

    # Load the halfword at data4 into register x9
    lh x9, 10(x5)           # x9 = *data4

    # Store the value in register x6 into a new memory location
    sw x6, 12(x5)           # *(data1 + 12) = x6

    # Store the byte in register x8 into a new memory location
    sb x8, 16(x5)           # *(data1 + 16) = x8

    # Store the halfword in register x9 into a new memory location
    sh x9, 18(x5)           # *(data1 + 18) = x9

    # End of the program
    j end                   # Jump to end (an infinite loop)

end:
    j end                   # Infinite loop to stop execution
