// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// tests/tests/test_load_elf_file.rs

use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use goblin::elf::{header, Elf};

use sim_lib::loader::Loader;
mod common;

fn get_elf_info<P: AsRef<Path>>(path: P) -> Result<(usize, usize, usize, usize), &'static str> {
    let mut file = File::open(path).map_err(|_| "Failed to open file")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|_| "Failed to read file")?;

    // Parse the ELF file
    let elf = Elf::parse(&buffer).map_err(|_| "Failed to parse ELF")?;

    // Check if it's a RISC-V ELF
    if elf.header.e_machine != header::EM_RISCV {
        return Err("Not a RISC-V ELF file");
    }

    // Get the entry point
    let entry_point = elf.header.e_entry as usize;

    // Get the symbol addresses
    let exit_sym = elf
        .syms
        .iter()
        .find(|sym| elf.strtab.get_at(sym.st_name) == Some("exit"))
        .map(|sym| sym.st_value as usize)
        .ok_or("Failed to find 'exit' symbol")?;

    let cnt1_sym = elf
        .syms
        .iter()
        .find(|sym| elf.strtab.get_at(sym.st_name) == Some("cnt1"))
        .map(|sym| sym.st_value as usize)
        .ok_or("Failed to find 'cnt1' symbol")?;

    let cnt2_sym = elf
        .syms
        .iter()
        .find(|sym| elf.strtab.get_at(sym.st_name) == Some("cnt2"))
        .map(|sym| sym.st_value as usize)
        .ok_or("Failed to find 'cnt2' symbol")?;

    Ok((entry_point, exit_sym, cnt1_sym, cnt2_sym))
}

#[test]
fn test_load_elf_file() {
    // common::setup_tracing();
    let elf_file = "../tests/tests/data/empty_main.elf";

    // step 1. create a simulator
    let mut sim = common::creat_sim_for_test();

    // step 2. load ELF file into memory
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let elf_file_path = project_root.join(elf_file);
    println!("ELF file path: {:?}", elf_file_path);

    let loader = Loader::load_elf_file(elf_file_path.as_path(), sim.get_bus_mut())
        .unwrap()
        .unwrap();

    // check ...
    assert!(
        !loader.program_headers().is_empty(),
        "Program headers should not be empty"
    );

    // check memory data
    match get_elf_info(elf_file) {
        Ok((entry, exit, cnt1, cnt2)) => {
            // println!("Entry point: {:#x}", entry);
            // println!("Address of 'exit': {:#x}", exit);
            // println!("Address of 'cnt1': {:#x}", cnt1);
            // println!("Address of 'cnt2': {:#x}", cnt2);

            assert_eq!(loader.entry_point(), entry as u64, "Unexpected entry point");

            let bus = sim.get_bus();
            assert_eq!(bus.read_word(exit), Ok(0xff010113));
            assert_eq!(bus.read_word(cnt1), Ok(0x00000000));
            assert_eq!(bus.read_word(cnt2), Ok(0x10));
        }
        Err(e) => {
            println!("Error: {}", e);
            assert!(false, "Failed to get ELF info");
        }
    }
}
