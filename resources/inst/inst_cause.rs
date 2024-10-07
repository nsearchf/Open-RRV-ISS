// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.


const CAUSE_MISALIGNED_FETCH: u8 = 0x0;
const CAUSE_FETCH_ACCESS: u8 = 0x1;
const CAUSE_ILLEGAL_INSTRUCTION: u8 = 0x2;
const CAUSE_BREAKPOINT: u8 = 0x3;
const CAUSE_MISALIGNED_LOAD: u8 = 0x4;
const CAUSE_LOAD_ACCESS: u8 = 0x5;
const CAUSE_MISALIGNED_STORE: u8 = 0x6;
const CAUSE_STORE_ACCESS: u8 = 0x7;
const CAUSE_USER_ECALL: u8 = 0x8;
const CAUSE_SUPERVISOR_ECALL: u8 = 0x9;
const CAUSE_VIRTUAL_SUPERVISOR_ECALL: u8 = 0xa;
const CAUSE_MACHINE_ECALL: u8 = 0xb;
const CAUSE_FETCH_PAGE_FAULT: u8 = 0xc;
const CAUSE_LOAD_PAGE_FAULT: u8 = 0xd;
const CAUSE_STORE_PAGE_FAULT: u8 = 0xf;
const CAUSE_DOUBLE_TRAP: u8 = 0x10;
const CAUSE_SOFTWARE_CHECK_FAULT: u8 = 0x12;
const CAUSE_HARDWARE_ERROR_FAULT: u8 = 0x13;
const CAUSE_FETCH_GUEST_PAGE_FAULT: u8 = 0x14;
const CAUSE_LOAD_GUEST_PAGE_FAULT: u8 = 0x15;
const CAUSE_VIRTUAL_INSTRUCTION: u8 = 0x16;
const CAUSE_STORE_GUEST_PAGE_FAULT: u8 = 0x17;
