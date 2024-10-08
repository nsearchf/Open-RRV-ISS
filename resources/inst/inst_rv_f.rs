// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.


/* Automatically generated by parse_opcodes */
const MATCH_FADD_S: u32 = 0x53;
const MASK_FADD_S: u32 = 0xfe00007f;
const MATCH_FCLASS_S: u32 = 0xe0001053;
const MASK_FCLASS_S: u32 = 0xfff0707f;
const MATCH_FCVT_S_W: u32 = 0xd0000053;
const MASK_FCVT_S_W: u32 = 0xfff0007f;
const MATCH_FCVT_S_WU: u32 = 0xd0100053;
const MASK_FCVT_S_WU: u32 = 0xfff0007f;
const MATCH_FCVT_W_S: u32 = 0xc0000053;
const MASK_FCVT_W_S: u32 = 0xfff0007f;
const MATCH_FCVT_WU_S: u32 = 0xc0100053;
const MASK_FCVT_WU_S: u32 = 0xfff0007f;
const MATCH_FDIV_S: u32 = 0x18000053;
const MASK_FDIV_S: u32 = 0xfe00007f;
const MATCH_FEQ_S: u32 = 0xa0002053;
const MASK_FEQ_S: u32 = 0xfe00707f;
const MATCH_FLE_S: u32 = 0xa0000053;
const MASK_FLE_S: u32 = 0xfe00707f;
const MATCH_FLT_S: u32 = 0xa0001053;
const MASK_FLT_S: u32 = 0xfe00707f;
const MATCH_FLW: u32 = 0x2007;
const MASK_FLW: u32 = 0x707f;
const MATCH_FMADD_S: u32 = 0x43;
const MASK_FMADD_S: u32 = 0x600007f;
const MATCH_FMAX_S: u32 = 0x28001053;
const MASK_FMAX_S: u32 = 0xfe00707f;
const MATCH_FMIN_S: u32 = 0x28000053;
const MASK_FMIN_S: u32 = 0xfe00707f;
const MATCH_FMSUB_S: u32 = 0x47;
const MASK_FMSUB_S: u32 = 0x600007f;
const MATCH_FMUL_S: u32 = 0x10000053;
const MASK_FMUL_S: u32 = 0xfe00007f;
const MATCH_FMV_W_X: u32 = 0xf0000053;
const MASK_FMV_W_X: u32 = 0xfff0707f;
const MATCH_FMV_X_W: u32 = 0xe0000053;
const MASK_FMV_X_W: u32 = 0xfff0707f;
const MATCH_FNMADD_S: u32 = 0x4f;
const MASK_FNMADD_S: u32 = 0x600007f;
const MATCH_FNMSUB_S: u32 = 0x4b;
const MASK_FNMSUB_S: u32 = 0x600007f;
const MATCH_FSGNJ_S: u32 = 0x20000053;
const MASK_FSGNJ_S: u32 = 0xfe00707f;
const MATCH_FSGNJN_S: u32 = 0x20001053;
const MASK_FSGNJN_S: u32 = 0xfe00707f;
const MATCH_FSGNJX_S: u32 = 0x20002053;
const MASK_FSGNJX_S: u32 = 0xfe00707f;
const MATCH_FSQRT_S: u32 = 0x58000053;
const MASK_FSQRT_S: u32 = 0xfff0007f;
const MATCH_FSUB_S: u32 = 0x8000053;
const MASK_FSUB_S: u32 = 0xfe00007f;
const MATCH_FSW: u32 = 0x2027;
const MASK_FSW: u32 = 0x707f;
