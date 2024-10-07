// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// tests/tests/test_load_elf_file.rs

use std::path::PathBuf;

use sim_lib::loader::Loader;
mod common;

#[test]
fn test_load_elf_file() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = common::creat_sim_for_test();

    // step 2. load ELF file into memory
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let elf_file_path = project_root.join("../tests/tests/data/empty_main.elf");
    println!("ELF file path: {:?}", elf_file_path);
    let loader = Loader::load_elf_file(elf_file_path.as_path(), sim.get_bus_mut())
        .unwrap()
        .unwrap();

    // check ...
    assert!(
        !loader.program_headers().is_empty(),
        "Program headers should not be empty"
    );
    assert_eq!(loader.entry_point(), 0x100d8, "Unexpected entry point");

    // check memory data
    let bus = sim.get_bus();
    assert_eq!(bus.read_word(0x10094), Ok(0xff010113));
    assert_eq!(bus.read_word(0x00012594), Ok(0x00000000));
    assert_eq!(bus.read_word(0x0001254c), Ok(0x10));
}
