// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

use std::path::PathBuf;

use tracing_subscriber::{self, FmtSubscriber};

use cpu_peripherals::{
    bus::{Bus, DevicePointer},
    mem::Mem,
    DeviceAddress, DeviceSize,
};
use sim_lib::simulator::Simulator;

pub(crate) const MEMORY_BASE_ADDRESS: DeviceAddress = 0x1_0000;
const MEMORY_SIZE: DeviceSize = 0x2_0000;

#[allow(dead_code)]
pub(crate) fn creat_sim_for_test() -> Simulator {
    // step 1. create a bus
    let mut bus = Bus::new();

    // step 2. create memory and add it to the bus
    let memory = DevicePointer::new(Mem::new(MEMORY_SIZE));
    let _ = bus.add_device(MEMORY_BASE_ADDRESS, MEMORY_SIZE, memory);

    // step 3. create a simulator
    // let mut sim = Simulator::new(bus);

    Simulator::new(bus)
}

pub(crate) const FLASH_BASE_ADDRESS: DeviceAddress = 0x8000_0000;
const FLASH_SIZE: DeviceSize = 512 * 1024;

const RAM_BASE_ADDRESS: DeviceAddress = 0x8008_0000;
const RAM_SIZE: DeviceSize = 512 * 1024;

#[allow(dead_code)]
pub(crate) fn creat_mcu_sim_for_test() -> Simulator {
    // step 1. create a bus
    let mut bus = Bus::new();

    // step 2. create memory and add it to the bus
    let memory = DevicePointer::new(Mem::new(FLASH_SIZE));
    let _ = bus.add_device(FLASH_BASE_ADDRESS, FLASH_SIZE, memory);
    let memory = DevicePointer::new(Mem::new(RAM_SIZE));
    let _ = bus.add_device(RAM_BASE_ADDRESS, RAM_SIZE, memory);

    // step 3. create a simulator
    Simulator::new(bus)
}

#[allow(dead_code)]
pub(crate) fn setup_tracing() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

#[allow(dead_code)]
pub(crate) fn load_test_bin_program(sim: &mut Simulator, bin_file_name: &str) {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bin_file_path = project_root.join(bin_file_name);
    println!("bin file path: {:?}", bin_file_path);
    let _ = sim.load_bin_file(bin_file_path.as_path(), MEMORY_BASE_ADDRESS);
}
