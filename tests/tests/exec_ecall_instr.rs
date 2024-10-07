// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// tests/tests/exec_ecall_instr.rs

mod common;

#[test]
fn test_ecall_instruction_execution_0() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = common::creat_sim_for_test();

    // step 2. load the program into memory
    common::load_test_bin_program(&mut sim, "tests/data/test_ecall.bin");

    // step 3. prepare the environment
    sim.set_reset_vector(common::MEMORY_BASE_ADDRESS.try_into().unwrap());
    assert_eq!(sim.get_core().read_reg_by_name("a0"), Ok(0));

    // step 4. run the simulator
    sim.run(Some(12)).expect("Simulation failed");

    // step 5. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a0"), Ok(53));
}

#[test]
fn test_ecall_instruction_execution_1() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = common::creat_sim_for_test();

    // step 2. load the program into memory
    common::load_test_bin_program(&mut sim, "tests/data/test_ecall.bin");

    // step 3. prepare the environment
    sim.set_reset_vector(common::MEMORY_BASE_ADDRESS.try_into().unwrap());
    assert_eq!(sim.get_core().read_reg_by_name("a0"), Ok(0));

    // step 4. run the simulator
    sim.run(Some(13)).expect("Simulation failed");

    // step 5. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a0"), Ok(54));
}
