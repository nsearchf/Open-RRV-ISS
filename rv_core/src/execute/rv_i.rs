// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use tracing::trace;

use crate::trap::{Exception, Trap};
use crate::GprSigned;
use crate::{core::Core, GprUnsigned, MachineInstruction, ProgramCounter, RvCoreError};
use cpu_peripherals::{bus::Bus, CpuPeripheralsError, DeviceAddress};

use crate::decode::{
    self,
    disassemble::{
        disasm_format_b, disasm_format_i, disasm_format_j, disasm_format_r, disasm_format_s,
        disasm_format_u,
    },
    ExecutionReturnData,
};

pub(crate) fn execute_add(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_shift_arithmetic_logical_compare_r(
        raw,
        core,
        disasm,
        |rs1, rs2| rs1.wrapping_add(rs2) as GprUnsigned,
        "ADD",
    )
}

pub(crate) fn execute_addi(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_arithmetic_logical_compare_i(
        raw,
        core,
        disasm,
        |rs1, imm| rs1.wrapping_add(imm) as GprUnsigned,
        "ADDI",
    )
}

pub(crate) fn execute_and(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_shift_arithmetic_logical_compare_r(
        raw,
        core,
        disasm,
        |rs1, rs2| (rs1 & rs2) as GprUnsigned,
        "AND",
    )
}
pub(crate) fn execute_andi(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_arithmetic_logical_compare_i(
        raw,
        core,
        disasm,
        |rs1, imm| (rs1 & imm) as GprUnsigned,
        "ANDI",
    )
}
pub(crate) fn execute_auipc(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    let operands = decode::parse_u_type(raw);
    trace!("Executing AUIPC with operands: {:?}", operands);
    let rd_val = core.get_pc() + operands.imm as ProgramCounter;
    core.write_register(operands.rd, rd_val as GprUnsigned)?;

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: None,
            disasm: Some(disasm_format_u("AUIPC", &operands, core, false)),
        }))
    } else {
        Ok(None)
    }
}

pub(crate) fn execute_beq(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_branch_instruction("BEQ", raw, core, disasm, |a, b| a == b)
}

pub(crate) fn execute_bge(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_branch_instruction("BGE", raw, core, disasm, |a, b| a >= b)
}
pub(crate) fn execute_bgeu(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_branch_instruction("BGEU", raw, core, disasm, |a, b| {
        a as GprUnsigned >= b as GprUnsigned
    })
}
pub(crate) fn execute_blt(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_branch_instruction("BLT", raw, core, disasm, |a, b| a < b)
}
pub(crate) fn execute_bltu(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_branch_instruction("BLTU", raw, core, disasm, |a, b| {
        (a as GprUnsigned) < (b as GprUnsigned)
    })
}
pub(crate) fn execute_bne(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_branch_instruction("BNE", raw, core, disasm, |a, b| a != b)
}
pub(crate) fn execute_ebreak(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    trace!("Executing EBREAK");

    // trigger trap
    core.set_trap(Trap::Exception(Exception::Breakpoint), raw)?;

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: None,
            disasm: Some("EBREAK".to_string()),
        }))
    } else {
        Ok(None)
    }
}

pub(crate) fn execute_ecall(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    trace!("Executing ECALL");

    // trigger trap
    core.set_trap(Trap::Exception(Exception::ECallFromMMode), raw)?;

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: None,
            disasm: Some("ECALL".to_string()),
        }))
    } else {
        Ok(None)
    }
}

pub(crate) fn execute_fence(
    _raw: MachineInstruction,
    _core: &mut Core,
    _bus: &mut Bus,
    _disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    todo!("Not implemented")
}
pub(crate) fn execute_jal(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    let operands = decode::parse_j_type(raw);
    trace!("Executing JAL with operands: {:?}", operands);
    let pc = core.get_pc();
    let next_pc = pc.wrapping_add(4);
    let new_pc = pc.wrapping_add(operands.imm as GprUnsigned as ProgramCounter);
    core.write_register(operands.rd, next_pc as GprUnsigned)?;

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: Some(new_pc as ProgramCounter),
            disasm: Some(disasm_format_j("JAL", &operands, core, false)),
        }))
    } else {
        Ok(Some(ExecutionReturnData {
            pc: Some(new_pc as ProgramCounter),
            disasm: None,
        }))
    }
}

pub(crate) fn execute_jalr(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    let operands = decode::parse_i_type(raw);
    trace!("Executing JALR with operands: {:?}", operands);
    let next_pc = core.get_pc().wrapping_add(4);

    let rs1 = core.read_register(operands.rs1).unwrap();
    let new_pc = rs1.wrapping_add(operands.imm as GprUnsigned);
    core.write_register(operands.rd, next_pc as GprUnsigned)?;

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: Some(new_pc as ProgramCounter),
            disasm: Some(disasm_format_i("JALR", &operands, core, false)),
        }))
    } else {
        Ok(Some(ExecutionReturnData {
            pc: Some(new_pc as ProgramCounter),
            disasm: None,
        }))
    }
}

pub(crate) fn execute_lb(
    raw: MachineInstruction,
    core: &mut Core,
    bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_load_i_type(
        |addr| bus.read_byte(addr),
        |val| (val as GprSigned) as GprUnsigned,
        "LB",
        raw,
        core,
        disasm,
    )
}

pub(crate) fn execute_lbu(
    raw: MachineInstruction,
    core: &mut Core,
    bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_load_i_type(
        |addr| bus.read_byte(addr),
        |val| val as GprUnsigned,
        "LBU",
        raw,
        core,
        disasm,
    )
}
pub(crate) fn execute_lh(
    raw: MachineInstruction,
    core: &mut Core,
    bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_load_i_type(
        |addr| bus.read_halfword(addr),
        |val| (val as GprSigned) as GprUnsigned,
        "LH",
        raw,
        core,
        disasm,
    )
}

pub(crate) fn execute_lhu(
    raw: MachineInstruction,
    core: &mut Core,
    bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_load_i_type(
        |addr| bus.read_halfword(addr),
        |val| val as GprUnsigned,
        "LHU",
        raw,
        core,
        disasm,
    )
}
pub(crate) fn execute_lui(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    let operands = decode::parse_u_type(raw);
    trace!("Executing LUI with operands: {:?}", operands);
    core.write_register(operands.rd, operands.imm as GprUnsigned)?;

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: None,
            disasm: Some(disasm_format_u("LUI", &operands, core, false)),
        }))
    } else {
        Ok(None)
    }
}
pub(crate) fn execute_lw(
    raw: MachineInstruction,
    core: &mut Core,
    bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_load_i_type(
        |addr| bus.read_word(addr),
        |val| (val as GprSigned) as GprUnsigned,
        "LW",
        raw,
        core,
        disasm,
    )
}

pub(crate) fn execute_or(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_shift_arithmetic_logical_compare_r(
        raw,
        core,
        disasm,
        |rs1, rs2| (rs1 | rs2) as GprUnsigned,
        "OR",
    )
}
pub(crate) fn execute_ori(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_arithmetic_logical_compare_i(
        raw,
        core,
        disasm,
        |rs1, imm| (rs1 | imm) as GprUnsigned,
        "ORI",
    )
}
pub(crate) fn execute_sb(
    raw: MachineInstruction,
    core: &mut Core,
    bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    let operands = decode::parse_s_type(raw);
    trace!("Executing SB with operands: {:?}", operands);
    let rs1 = core.read_register(operands.rs1).unwrap();
    let mem_addr = rs1.wrapping_add(operands.imm as GprUnsigned);

    let rs2 = core.read_register(operands.rs2).unwrap();

    bus.write_byte(mem_addr as DeviceAddress, rs2 as u8)?;

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: None,
            disasm: Some(disasm_format_s("SB", &operands, core, false)),
        }))
    } else {
        Ok(None)
    }
}

pub(crate) fn execute_sh(
    raw: MachineInstruction,
    core: &mut Core,
    bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    let operands = decode::parse_s_type(raw);
    trace!("Executing SH with operands: {:?}", operands);
    let rs1 = core.read_register(operands.rs1).unwrap();
    let mem_addr = rs1.wrapping_add(operands.imm as GprUnsigned);

    let rs2 = core.read_register(operands.rs2).unwrap();

    bus.write_halfword(mem_addr as DeviceAddress, rs2 as u16)?;

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: None,
            disasm: Some(disasm_format_s("SH", &operands, core, false)),
        }))
    } else {
        Ok(None)
    }
}

pub(crate) fn execute_sll(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_shift_arithmetic_logical_compare_r(
        raw,
        core,
        disasm,
        |rs1, rs2| (rs1 << rs2) as GprUnsigned,
        "SLL",
    )
}

pub(crate) fn execute_slt(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_shift_arithmetic_logical_compare_r(
        raw,
        core,
        disasm,
        |rs1, rs2| (rs1 < rs2) as GprUnsigned,
        "SLT",
    )
}

pub(crate) fn execute_slti(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_arithmetic_logical_compare_i(
        raw,
        core,
        disasm,
        |rs1, imm| (rs1 < imm) as GprUnsigned,
        "SLTI",
    )
}

pub(crate) fn execute_sltiu(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_arithmetic_logical_compare_i(
        raw,
        core,
        disasm,
        |rs1, imm| ((rs1 as GprUnsigned) < (imm as GprUnsigned)) as GprUnsigned,
        "SLTIU",
    )
}

pub(crate) fn execute_sltu(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_shift_arithmetic_logical_compare_r(
        raw,
        core,
        disasm,
        |rs1, rs2| ((rs1 as GprUnsigned) < (rs2 as GprUnsigned)) as GprUnsigned,
        "SLTU",
    )
}

pub(crate) fn execute_sra(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_shift_arithmetic_logical_compare_r(
        raw,
        core,
        disasm,
        |rs1, rs2| ((rs1 as GprSigned) >> rs2) as GprUnsigned,
        "SRA",
    )
}

pub(crate) fn execute_srl(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_shift_arithmetic_logical_compare_r(
        raw,
        core,
        disasm,
        |rs1, rs2| ((rs1 as GprUnsigned) >> rs2) as GprUnsigned,
        "SRL",
    )
}

pub(crate) fn execute_sub(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_shift_arithmetic_logical_compare_r(
        raw,
        core,
        disasm,
        |rs1, rs2| (rs1 - rs2) as GprUnsigned,
        "SUB",
    )
}
pub(crate) fn execute_sw(
    raw: MachineInstruction,
    core: &mut Core,
    bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    let operands = decode::parse_s_type(raw);
    trace!("Executing SW with operands: {:?}", operands);
    let rs1 = core.read_register(operands.rs1).unwrap();
    let mem_addr = rs1.wrapping_add(operands.imm as GprUnsigned);
    trace!(">>>>rs1, mem_addr: {:#x}, {:#x}", rs1, mem_addr);

    let rs2 = core.read_register(operands.rs2).unwrap();

    bus.write_word(mem_addr as DeviceAddress, rs2 as u32)?;

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: None,
            disasm: Some(disasm_format_s("SW", &operands, core, false)),
        }))
    } else {
        Ok(None)
    }
}

pub(crate) fn execute_xor(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_shift_arithmetic_logical_compare_r(
        raw,
        core,
        disasm,
        |rs1, rs2| (rs1 ^ rs2) as GprUnsigned,
        "XOR",
    )
}
pub(crate) fn execute_xori(
    raw: MachineInstruction,
    core: &mut Core,
    _bus: &mut Bus,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    execute_arithmetic_logical_compare_i(
        raw,
        core,
        disasm,
        |rs1, imm| (rs1 ^ imm) as GprUnsigned,
        "XORI",
    )
}

fn execute_shift_arithmetic_logical_compare_r(
    raw: MachineInstruction,
    core: &mut Core,
    disasm: bool,
    operation: fn(GprSigned, GprSigned) -> GprUnsigned,
    mnemonic: &'static str,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    let operands = decode::parse_r_type(raw);
    trace!("Executing {} with operands: {:?}", mnemonic, operands);
    let rs1 = core.read_register(operands.rs1).unwrap() as GprSigned;
    let rs2 = core.read_register(operands.rs2).unwrap() as GprSigned;
    let value = operation(rs1, rs2);
    let _ = core.write_register(operands.rd, value).unwrap();

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: None,
            disasm: Some(disasm_format_r(mnemonic, &operands, core, false)),
        }))
    } else {
        Ok(None)
    }
}

fn execute_arithmetic_logical_compare_i(
    raw: MachineInstruction,
    core: &mut Core,
    disasm: bool,
    operation: fn(GprSigned, GprSigned) -> GprUnsigned,
    mnemonic: &'static str,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    let operands = decode::parse_i_type(raw);
    trace!("Executing {} with operands: {:?}", mnemonic, operands);
    let rs1 = core.read_register(operands.rs1).unwrap() as GprSigned;
    let imm = operands.imm;
    let value = operation(rs1, imm);
    let _ = core.write_register(operands.rd, value).unwrap();

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: None,
            disasm: Some(disasm_format_i(mnemonic, &operands, core, false)),
        }))
    } else {
        Ok(None)
    }
}

fn execute_branch_instruction(
    mnemonic: &'static str,
    raw: MachineInstruction,
    core: &mut Core,
    disasm: bool,
    compare: fn(GprSigned, GprSigned) -> bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError> {
    let operands = decode::parse_b_type(raw);
    trace!("Executing {} with operands: {:?}", mnemonic, operands);

    let rs1 = core.read_register(operands.rs1).unwrap() as GprSigned;
    let rs2 = core.read_register(operands.rs2).unwrap() as GprSigned;

    let pc = if compare(rs1, rs2) {
        let new_pc = core
            .get_pc()
            .wrapping_add(operands.imm as GprUnsigned as ProgramCounter);
        Some(new_pc as ProgramCounter)
    } else {
        None
    };

    let disasm = if disasm {
        Some(disasm_format_b(mnemonic, &operands, core, false))
    } else {
        None
    };

    if pc.is_none() && disasm.is_none() {
        Ok(None)
    } else {
        Ok(Some(ExecutionReturnData { pc, disasm }))
    }
}

trait BusAccessWidth {}

impl BusAccessWidth for u8 {}
impl BusAccessWidth for u16 {}
impl BusAccessWidth for u32 {}
impl BusAccessWidth for i8 {}
impl BusAccessWidth for i16 {}
impl BusAccessWidth for i32 {}

fn execute_load_i_type<T>(
    load_fn: impl Fn(DeviceAddress) -> Result<T, CpuPeripheralsError>,
    convert_fn: impl Fn(T) -> GprUnsigned,
    mnemonic: &'static str,
    raw: MachineInstruction,
    core: &mut Core,
    disasm: bool,
) -> Result<Option<ExecutionReturnData>, RvCoreError>
where
    T: BusAccessWidth,
{
    let operands = decode::parse_i_type(raw);
    trace!("Executing {} with operands: {:?}", mnemonic, operands);
    let rs1 = core.read_register(operands.rs1).unwrap();
    let mem_addr = rs1.wrapping_add(operands.imm as GprUnsigned);

    let val = load_fn(mem_addr as DeviceAddress)?;
    let _ = core.write_register(operands.rd, convert_fn(val)).unwrap();

    if disasm {
        Ok(Some(ExecutionReturnData {
            pc: None,
            disasm: Some(disasm_format_i(mnemonic, &operands, core, false)),
        }))
    } else {
        Ok(None)
    }
}
