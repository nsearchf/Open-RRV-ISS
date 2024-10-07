// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

pub const CAUSE_MISALIGNED_FETCH: u8 = 0x0;
pub const CAUSE_FETCH_ACCESS: u8 = 0x1;
pub const CAUSE_ILLEGAL_INSTRUCTION: u8 = 0x2;
pub const CAUSE_BREAKPOINT: u8 = 0x3;
pub const CAUSE_MISALIGNED_LOAD: u8 = 0x4;
pub const CAUSE_LOAD_ACCESS: u8 = 0x5;
pub const CAUSE_MISALIGNED_STORE: u8 = 0x6;
pub const CAUSE_STORE_ACCESS: u8 = 0x7;
pub const CAUSE_USER_ECALL: u8 = 0x8;
pub const CAUSE_SUPERVISOR_ECALL: u8 = 0x9;
pub const CAUSE_VIRTUAL_SUPERVISOR_ECALL: u8 = 0xa;
pub const CAUSE_MACHINE_ECALL: u8 = 0xb;
pub const CAUSE_FETCH_PAGE_FAULT: u8 = 0xc;
pub const CAUSE_LOAD_PAGE_FAULT: u8 = 0xd;
pub const CAUSE_STORE_PAGE_FAULT: u8 = 0xf;
pub const CAUSE_DOUBLE_TRAP: u8 = 0x10;
pub const CAUSE_SOFTWARE_CHECK_FAULT: u8 = 0x12;
pub const CAUSE_HARDWARE_ERROR_FAULT: u8 = 0x13;
pub const CAUSE_FETCH_GUEST_PAGE_FAULT: u8 = 0x14;
pub const CAUSE_LOAD_GUEST_PAGE_FAULT: u8 = 0x15;
pub const CAUSE_VIRTUAL_INSTRUCTION: u8 = 0x16;
pub const CAUSE_STORE_GUEST_PAGE_FAULT: u8 = 0x17;
