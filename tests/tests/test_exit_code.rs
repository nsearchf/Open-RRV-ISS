// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.


use std::path::PathBuf;

use sim_lib::{loader::Loader, ProgramCounter};
mod common;

#[test]
fn test_exit_code_0() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = common::creat_mcu_sim_for_test();

    // step 2. load ELF file into memory
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let elf_file_path = project_root.join("../tests/tests/data/exit_code/exit_code_0.elf");
    println!("ELF file path: {:?}", elf_file_path);
    let loader = Loader::load_elf_file(elf_file_path.as_path(), sim.get_bus_mut())
        .unwrap()
        .unwrap();

    // check loader ...
    assert!(
        !loader.program_headers().is_empty(),
        "Program headers should not be empty"
    );

    let entry_point = loader.entry_point();
    assert_eq!(
        entry_point,
        common::FLASH_BASE_ADDRESS as u64,
        "Unexpected entry point"
    );
    sim.set_reset_vector(entry_point as ProgramCounter);

    // step 3. run the simulator
    sim.run(None).expect("Simulation failed");

    // step 4. check the result
    let exit_code = sim.get_exit_code();
    assert_eq!(exit_code, 0);
}

#[test]
fn test_exit_code_1() {
    // step 1. create a simulator
    let mut sim = common::creat_mcu_sim_for_test();

    // step 2. load ELF file into memory
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let elf_file_path = project_root.join("../tests/tests/data/exit_code/exit_code_1.elf");
    println!("ELF file path: {:?}", elf_file_path);
    let loader = Loader::load_elf_file(elf_file_path.as_path(), sim.get_bus_mut())
        .unwrap()
        .unwrap();

    // check loader ...
    assert!(
        !loader.program_headers().is_empty(),
        "Program headers should not be empty"
    );

    let entry_point = loader.entry_point();
    assert_eq!(
        entry_point,
        common::FLASH_BASE_ADDRESS as u64,
        "Unexpected entry point"
    );
    sim.set_reset_vector(entry_point as ProgramCounter);

    // step 3. run the simulator
    sim.run(None).expect("Simulation failed");

    // step 4. check the result
    let exit_code = sim.get_exit_code();
    assert_eq!(exit_code, 0x1234);
}
