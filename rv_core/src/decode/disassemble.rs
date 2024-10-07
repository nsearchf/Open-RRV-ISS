// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use crate::{
    core::Core,
    decode::{FormatB, FormatI, FormatJ, FormatR, FormatS, FormatU},
};

pub(crate) fn disasm_format_r(
    name: &str,
    operands: &FormatR,
    core: &Core,
    use_reg_name: bool,
) -> String {
    if use_reg_name {
        let rd = core.get_reg_name_by_index(operands.rd).unwrap();
        let rs1 = core.get_reg_name_by_index(operands.rs1).unwrap();
        let rs2 = core.get_reg_name_by_index(operands.rs2).unwrap();
        format!("{} {}, {}, {}", name, rd, rs1, rs2)
    } else {
        format!(
            "{} x{}, x{}, x{}",
            name, operands.rd, operands.rs1, operands.rs2
        )
    }
}

pub(crate) fn disasm_format_i(
    name: &str,
    operands: &FormatI,
    core: &Core,
    use_reg_name: bool,
) -> String {
    if use_reg_name {
        let rd = core.get_reg_name_by_index(operands.rd).unwrap();
        let rs1 = core.get_reg_name_by_index(operands.rs1).unwrap();
        format!("{} {}, {}, {}", name, rd, rs1, operands.imm)
    } else {
        format!(
            "{} x{}, x{}, {}",
            name, operands.rd, operands.rs1, operands.imm
        )
    }
}

pub(crate) fn disasm_format_s(
    name: &str,
    operands: &FormatS,
    core: &Core,
    use_reg_name: bool,
) -> String {
    if use_reg_name {
        let rs1 = core.get_reg_name_by_index(operands.rs1).unwrap();
        let rs2 = core.get_reg_name_by_index(operands.rs2).unwrap();
        format!("{} {}, {}, {}", name, rs1, rs2, operands.imm)
    } else {
        format!(
            "{} x{}, x{}, {}",
            name, operands.rs1, operands.rs2, operands.imm
        )
    }
}

pub(crate) fn disasm_format_b(
    name: &str,
    operands: &FormatB,
    core: &Core,
    use_reg_name: bool,
) -> String {
    if use_reg_name {
        let rs1 = core.get_reg_name_by_index(operands.rs1).unwrap();
        let rs2 = core.get_reg_name_by_index(operands.rs2).unwrap();
        format!("{} {}, {}, {}", name, rs1, rs2, operands.imm)
    } else {
        format!(
            "{} x{}, x{}, {}",
            name, operands.rs1, operands.rs2, operands.imm
        )
    }
}

pub(crate) fn disasm_format_u(
    name: &str,
    operands: &FormatU,
    core: &Core,
    use_reg_name: bool,
) -> String {
    if use_reg_name {
        let rd = core.get_reg_name_by_index(operands.rd).unwrap();
        format!("{} {}, {}", name, rd, operands.imm)
    } else {
        format!("{} x{}, {}", name, operands.rd, operands.imm)
    }
}

pub(crate) fn disasm_format_j(
    name: &str,
    operands: &FormatJ,
    core: &Core,
    use_reg_name: bool,
) -> String {
    if use_reg_name {
        let rd = core.get_reg_name_by_index(operands.rd).unwrap();
        format!("{} {}, {}", name, rd, operands.imm)
    } else {
        format!("{} x{}, {}", name, operands.rd, operands.imm)
    }
}
