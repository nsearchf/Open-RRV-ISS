// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.


use std::path::PathBuf;

use cpu_peripherals::bus::{Bus, DevicePointer};
use cpu_peripherals::{mem::Mem, uart::Uart, DeviceAddress, DeviceSize};
// use cpu_peripherals::{clint::Clint, mem::Mem, uart::Uart, DeviceAddress, DeviceSize};
use sim_lib::loader::Loader;
use sim_lib::simulator::Simulator;
use sim_lib::ProgramCounter;

mod common;

const FLASH_BASE_ADDRESS: DeviceAddress = 0x8000_0000;
const FLASH_SIZE: DeviceSize = 512 * 1024;

const RAM_BASE_ADDRESS: DeviceAddress = 0x8008_0000;
const RAM_SIZE: DeviceSize = 512 * 1024;

const UART_BASE_ADDRESS: DeviceAddress = 0x1001_3000;
const UART_SIZE: DeviceSize = 0x1000;

#[test]
fn test_load_elf_file() {
    // common::setup_tracing();

    // step 1. create a bus
    let mut bus = Bus::new();

    // step 2. create deivces and add them to the bus
    let memory = DevicePointer::new(Mem::new(FLASH_SIZE));
    let _ = bus.add_device(FLASH_BASE_ADDRESS, FLASH_SIZE, memory);
    let memory = DevicePointer::new(Mem::new(RAM_SIZE));
    let _ = bus.add_device(RAM_BASE_ADDRESS, RAM_SIZE, memory);

    let uart = DevicePointer::new(Uart::new("UART0"));
    let _ = bus.add_device(UART_BASE_ADDRESS, UART_SIZE, uart);

    // step 3. create a simulator
    let mut sim = Simulator::new(bus);

    // step 2. load ELF file into memory
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let elf_file_path = project_root.join("../tests/tests/data/md5/md5.elf");
    println!("ELF file path: {:?}", elf_file_path);
    let loader = Loader::load_elf_file(elf_file_path.as_path(), sim.get_bus_mut())
        .unwrap()
        .unwrap();

    // check ...
    assert!(
        !loader.program_headers().is_empty(),
        "Program headers should not be empty"
    );
    assert_eq!(loader.entry_point(), 0x8000_0000, "Unexpected entry point");
    let entry_point = loader.entry_point();
    sim.set_reset_vector(entry_point as ProgramCounter);

    // check memory data
    {
        let bus = sim.get_bus();
        let addr: DeviceAddress = 0x80080000_u32 as DeviceAddress;
        assert_eq!(bus.read_byte(addr + 0), Ok(0x6d));
        assert_eq!(bus.read_byte(addr + 1), Ok(0x8d));
        assert_eq!(bus.read_byte(addr + 2), Ok(0x0b));
        assert_eq!(bus.read_byte(addr + 3), Ok(0x2c));
    }

    sim.run(None).expect("Simulation failed");

    let exit_code = sim.get_exit_code();
    assert_eq!(exit_code, 0);
}
