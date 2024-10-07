.section .text
.global _start

_start:
    beq  x8, x9, label
    addi x4, x4, 1
    addi x5, x5, 1

label:
    addi x6, x6, 1
    addi x7, x7, 1

#
# for Unit test of rv_core/src/decode/mod.rs
#
temp_test:
    # R, I, S, B, U, J
    add x12, x13, x14 # add rd, rs1, rs2
beq_target:
    addi x15, x16, -2 # addi rd, rs1, imm
    sb x5, -3(x6)     # sb rs2, offset(rs1)
jal_target:
    beq x18, x19, beq_target # beq rs1, rs2, offset
    auipc x20, 0x1234        # auipc rd, imm
    jal x1, jal_target       # jal rd, offset
