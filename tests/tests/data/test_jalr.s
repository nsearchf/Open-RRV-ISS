.section .text
.global _start

_start:
    addi x5, x0, 10         # x5 = 10
    addi x6, x0, 20         # x6 = 20
    addi x7, x0, 0          # x7 = 0

    jal x1, label_jal
    addi x5, x5, 1

label_jal:
    addi x6, x6, 1
    jalr x1, x1, 0
