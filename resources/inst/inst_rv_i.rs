// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.


/* Automatically generated by parse_opcodes */
const MATCH_ADD: u32 = 0x33;
const MASK_ADD: u32 = 0xfe00707f;
const MATCH_ADDI: u32 = 0x13;
const MASK_ADDI: u32 = 0x707f;
const MATCH_AND: u32 = 0x7033;
const MASK_AND: u32 = 0xfe00707f;
const MATCH_ANDI: u32 = 0x7013;
const MASK_ANDI: u32 = 0x707f;
const MATCH_AUIPC: u32 = 0x17;
const MASK_AUIPC: u32 = 0x7f;
const MATCH_BEQ: u32 = 0x63;
const MASK_BEQ: u32 = 0x707f;
const MATCH_BGE: u32 = 0x5063;
const MASK_BGE: u32 = 0x707f;
const MATCH_BGEU: u32 = 0x7063;
const MASK_BGEU: u32 = 0x707f;
const MATCH_BLT: u32 = 0x4063;
const MASK_BLT: u32 = 0x707f;
const MATCH_BLTU: u32 = 0x6063;
const MASK_BLTU: u32 = 0x707f;
const MATCH_BNE: u32 = 0x1063;
const MASK_BNE: u32 = 0x707f;
const MATCH_EBREAK: u32 = 0x100073;
const MASK_EBREAK: u32 = 0xffffffff;
const MATCH_ECALL: u32 = 0x73;
const MASK_ECALL: u32 = 0xffffffff;
const MATCH_FENCE: u32 = 0xf;
const MASK_FENCE: u32 = 0x707f;
const MATCH_JAL: u32 = 0x6f;
const MASK_JAL: u32 = 0x7f;
const MATCH_JALR: u32 = 0x67;
const MASK_JALR: u32 = 0x707f;
const MATCH_LB: u32 = 0x3;
const MASK_LB: u32 = 0x707f;
const MATCH_LBU: u32 = 0x4003;
const MASK_LBU: u32 = 0x707f;
const MATCH_LH: u32 = 0x1003;
const MASK_LH: u32 = 0x707f;
const MATCH_LHU: u32 = 0x5003;
const MASK_LHU: u32 = 0x707f;
const MATCH_LUI: u32 = 0x37;
const MASK_LUI: u32 = 0x7f;
const MATCH_LW: u32 = 0x2003;
const MASK_LW: u32 = 0x707f;
const MATCH_OR: u32 = 0x6033;
const MASK_OR: u32 = 0xfe00707f;
const MATCH_ORI: u32 = 0x6013;
const MASK_ORI: u32 = 0x707f;
const MATCH_SB: u32 = 0x23;
const MASK_SB: u32 = 0x707f;
const MATCH_SH: u32 = 0x1023;
const MASK_SH: u32 = 0x707f;
const MATCH_SLL: u32 = 0x1033;
const MASK_SLL: u32 = 0xfe00707f;
const MATCH_SLT: u32 = 0x2033;
const MASK_SLT: u32 = 0xfe00707f;
const MATCH_SLTI: u32 = 0x2013;
const MASK_SLTI: u32 = 0x707f;
const MATCH_SLTIU: u32 = 0x3013;
const MASK_SLTIU: u32 = 0x707f;
const MATCH_SLTU: u32 = 0x3033;
const MASK_SLTU: u32 = 0xfe00707f;
const MATCH_SRA: u32 = 0x40005033;
const MASK_SRA: u32 = 0xfe00707f;
const MATCH_SRL: u32 = 0x5033;
const MASK_SRL: u32 = 0xfe00707f;
const MATCH_SUB: u32 = 0x40000033;
const MASK_SUB: u32 = 0xfe00707f;
const MATCH_SW: u32 = 0x2023;
const MASK_SW: u32 = 0x707f;
const MATCH_XOR: u32 = 0x4033;
const MASK_XOR: u32 = 0xfe00707f;
const MATCH_XORI: u32 = 0x4013;
const MASK_XORI: u32 = 0x707f;
