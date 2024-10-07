// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// tests/tests/exec_load_store_instr.rs

mod common;

#[test]
fn test_load_store_instruction() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = common::creat_sim_for_test();

    // step 2. load the program into memory
    common::load_test_bin_program(&mut sim, "tests/data/test_load_store.bin");

    // step 3. prepare the environment
    sim.set_reset_vector(common::MEMORY_BASE_ADDRESS.try_into().unwrap());

    // step 4. run the simulator
    sim.run(Some(6 + 3)).expect("Simulation failed");

    // step 5. check the result
    let core = sim.get_core();
    assert_eq!(core.read_register(6), Ok(0x12345678));
    assert_eq!(core.read_register(7), Ok(0x9abcdef0));
    assert_eq!(core.read_register(8), Ok(0x12));
    assert_eq!(core.read_register(9), Ok(0x3456));

    let data_base_addr = 0x1102c;
    let bus = sim.get_bus();
    assert_eq!(
        bus.read_word(data_base_addr + 12)
            .expect("Failed to read memory"),
        0x12345678
    );
    assert_eq!(
        bus.read_byte(data_base_addr + 16)
            .expect("Failed to read memory"),
        0x12
    );
    assert_eq!(
        bus.read_halfword(data_base_addr + 18)
            .expect("Failed to read memory"),
        0x3456
    );
}
