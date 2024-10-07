// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// tests/tests/exec_beq_instr.rs

use rv_core::GprUnsigned;

mod common;

#[test]
fn test_beq_instruction_execution_0() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = common::creat_sim_for_test();

    // step 2. load the program into memory
    common::load_test_bin_program(&mut sim, "tests/data/test_beq.bin");

    // step 3. prepare the environment
    sim.set_reset_vector(common::MEMORY_BASE_ADDRESS.try_into().unwrap());
    let core = sim.get_core_mut();
    let _ = core.write_register(8, 0).unwrap();
    let _ = core.write_register(9, 0).unwrap();

    // step 4. run the simulator
    sim.run(Some(3)).expect("Simulation failed");

    // step 5. check the result
    let core = sim.get_core();
    assert_eq!(core.read_register(4), Ok(0));
    assert_eq!(core.read_register(5), Ok(0));
    assert_eq!(core.read_register(6), Ok(1));
    assert_eq!(core.read_register(7), Ok(1));
}

#[test]
fn test_beq_instruction_execution_1() {
    // step 1. create a simulator
    let mut sim = common::creat_sim_for_test();

    // step 2. load the program into memory
    // load_test_program(&mut sim);
    common::load_test_bin_program(&mut sim, "tests/data/test_beq.bin");

    // step 3. prepare the environment
    sim.set_reset_vector(common::MEMORY_BASE_ADDRESS.try_into().unwrap());
    let core = sim.get_core_mut();
    let _ = core.write_register(8, 0).unwrap();
    let _ = core.write_register(9, 1).unwrap();

    // step 4. run the simulator
    sim.run(Some(3)).expect("Simulation failed");

    // step 5. check the result
    let core = sim.get_core();
    assert_eq!(core.read_register(4), Ok(1));
    assert_eq!(core.read_register(5), Ok(1));
    assert_eq!(core.read_register(6), Ok(0));
    assert_eq!(core.read_register(7), Ok(0));
}

#[test]
fn test_jal_instruction_execution() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = common::creat_sim_for_test();

    // step 2. load the program into memory
    common::load_test_bin_program(&mut sim, "tests/data/test_jal.bin");

    // step 3. prepare the environment
    sim.set_reset_vector(common::MEMORY_BASE_ADDRESS.try_into().unwrap());
    // let core = sim.get_core_mut();
    // let _ = core.write_register(8, 0).unwrap();
    // let _ = core.write_register(9, 0).unwrap();

    // step 4. run the simulator
    sim.run(Some(6)).expect("Simulation failed");

    // step 5. check the result
    let core = sim.get_core();
    assert_eq!(core.read_register(5), Ok(10));
    assert_eq!(core.read_register(6), Ok(20 + 1));
    assert_eq!(core.read_register(7), Ok(0 + 1));

    let next_pc = common::MEMORY_BASE_ADDRESS as GprUnsigned + 4 * 4;
    println!("next_pc: {:#x}", next_pc);
    assert_eq!(core.read_register(1), Ok(next_pc));
}

#[test]
fn test_jalr_instruction_execution() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = common::creat_sim_for_test();

    // step 2. load the program into memory
    common::load_test_bin_program(&mut sim, "tests/data/test_jalr.bin");

    // step 3. prepare the environment
    sim.set_reset_vector(common::MEMORY_BASE_ADDRESS.try_into().unwrap());
    // let core = sim.get_core_mut();
    // let _ = core.write_register(8, 0).unwrap();
    // let _ = core.write_register(9, 0).unwrap();

    // step 4. run the simulator
    sim.run(Some(7)).expect("Simulation failed");

    // step 5. check the result
    let core = sim.get_core();
    assert_eq!(core.read_register(5), Ok(10 + 1));
    assert_eq!(core.read_register(6), Ok(20 + 1));
    assert_eq!(core.read_register(7), Ok(0));

    let next_pc = common::MEMORY_BASE_ADDRESS as GprUnsigned + 7 * 4;
    println!("next_pc: {:#x}", next_pc);
    assert_eq!(core.read_register(1), Ok(next_pc));
}
