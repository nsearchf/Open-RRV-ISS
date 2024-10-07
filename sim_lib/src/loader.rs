// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// sim_lib/src/loader.rs

use std::fs::File;
use std::io::Read;
use std::path::Path;

use goblin::elf::Elf;

use tracing::{info, trace};

use cpu_peripherals::{bus::Bus, DeviceAddress};

use crate::SimulatorError;

pub struct Loader {
    entry_point: u64,
    program_headers: Vec<goblin::elf::ProgramHeader>,
}

impl Loader {
    pub fn load_bin_program(
        bin_program: &[u8],
        bus: &mut Box<Bus>,
        base_addr: DeviceAddress,
    ) -> Result<(), SimulatorError> {
        info!("Loading binary program");
        let mem = bus.find_device_mut(base_addr);
        let _ = mem.expect("cann't find mem").write(base_addr, bin_program);
        Ok(())
    }

    pub fn load_bin_file(
        bin_file: &Path,
        bus: &mut Box<Bus>,
        base_addr: DeviceAddress,
    ) -> Result<(), SimulatorError> {
        info!("Loading bin file: {:?}", bin_file);
        let mut file = File::open(bin_file)?;
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer)?;

        let mem = bus.find_device_mut(base_addr);
        let _ = mem.expect("cann't find mem").write(base_addr, &buffer);

        Ok(())
    }

    pub fn load_elf_file(
        elf_file: &Path,
        bus: &mut Box<Bus>,
    ) -> Result<Option<Self>, SimulatorError> {
        info!("Loading ELF file: {:?}", elf_file);
        let mut file = File::open(elf_file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let elf = Elf::parse(&buffer)?;

        let entry_point = elf.entry;
        let program_headers = elf.program_headers;

        for ph in &program_headers {
            trace!("Loading program header: {:?}", ph);
            if ph.p_type == goblin::elf::program_header::PT_LOAD {
                let offset = ph.p_offset as usize;
                let file_size = ph.p_filesz as usize;
                let mem_size = ph.p_memsz as usize;
                let vaddr = ph.p_vaddr as usize;

                // assert_eq!(mem_size, file_size);

                for i in 0..file_size {
                    bus.write_byte(vaddr + i, buffer[offset + i])?;
                }

                if mem_size > file_size {
                    for i in file_size..mem_size {
                        bus.write_byte(vaddr + i, 0)?;
                    }
                }
            }
        }

        Ok(Some(Loader {
            entry_point,
            program_headers,
        }))
    }

    pub fn entry_point(&self) -> u64 {
        self.entry_point
    }

    pub fn program_headers(&self) -> &[goblin::elf::ProgramHeader] {
        &self.program_headers
    }
}
