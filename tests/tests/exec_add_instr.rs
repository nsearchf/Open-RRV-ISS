// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// tests/tests/exec_add_instr.rs

use rv_core::{GprSigned, GprUnsigned};

mod common;

#[test]
fn test_add_instruction_execution() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = common::creat_sim_for_test();

    // step 2. load the program into memory
    // add a1, a0, a1 ---- add x11, x10, x11 ; add rd, rs1, rs2
    let add_instr = [0xb3, 0x05, 0xb5, 0x00];
    let _ = sim.load_bin_program(&add_instr, common::MEMORY_BASE_ADDRESS);

    // step 3. prepare the environment
    sim.set_reset_vector(common::MEMORY_BASE_ADDRESS.try_into().unwrap());
    let core = sim.get_core_mut();
    let _ = core.write_register(10, 10).unwrap();
    let _ = core.write_register(11, 11).unwrap();

    // step 4. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 5. check the result
    let core = sim.get_core();
    assert_eq!(core.read_register(11), Ok(21));
}

// addi t1, t1, 0
// 001_30313  addi	t1,t1,1 ---- addi x6, x6, 1 ; add rd, rs1, rs2
// let ADD_INSTR = [0x13, 0x03, 0x13, 0x00];
const ADDI_0_INSTR: [u8; 4] = [0x13, 0x03, 0x03, 0x00];

fn test_addi_instruction_execution_x(initial_val: GprSigned, imm: GprSigned) {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = common::creat_sim_for_test();

    // step 2. load the program into memory
    let mut addi_instr = u32::from_le_bytes(ADDI_0_INSTR);
    addi_instr |= (imm as GprUnsigned) << 20;
    let addi_instr = addi_instr.to_le_bytes();
    let _ = sim.load_bin_program(&addi_instr, common::MEMORY_BASE_ADDRESS);

    // step 3. prepare the environment
    sim.set_reset_vector(common::MEMORY_BASE_ADDRESS.try_into().unwrap());
    let core = sim.get_core_mut();
    let _ = core
        .write_reg_by_name("t1", initial_val as GprUnsigned)
        .unwrap();

    // step 4. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 5. check the result
    let core = sim.get_core();
    assert_eq!(
        core.read_register(6),
        Ok((initial_val + imm) as GprUnsigned)
    );
}

// positive + positive
#[test]
fn test_addi_instruction_execution_0() {
    test_addi_instruction_execution_x(1, 1);
}

// positive + negitive
#[test]
fn test_addi_instruction_execution_1() {
    test_addi_instruction_execution_x(1, -2);
}

// negitive + positive
#[test]
fn test_addi_instruction_execution_2() {
    test_addi_instruction_execution_x(-1, 2);
}

// negitive + negitive
#[test]
fn test_addi_instruction_execution_3() {
    test_addi_instruction_execution_x(-1, -5);
}

// 0 + positive
#[test]
fn test_addi_instruction_execution_4() {
    test_addi_instruction_execution_x(0, 10);
}

// 0 + negitive
#[test]
fn test_addi_instruction_execution_5() {
    test_addi_instruction_execution_x(0, -11);
}

// 0 + 0
#[test]
fn test_addi_instruction_execution_6() {
    test_addi_instruction_execution_x(0, 0);
}
