// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// sim_lib/src/loader.rs

use std::fs::File;
use std::io::Read;
use std::path::Path;

use goblin::elf::{Elf, ProgramHeader, Symtab};
use goblin::strtab::Strtab;

use tracing::{info, trace};

use cpu_peripherals::{bus::Bus, DeviceAddress};

use crate::SimulatorError;

pub struct SignatureMetaData {
    pub signature_addr: DeviceAddress,
    pub signature_len: usize,
}

pub struct Loader {
    entry_point: u64,
    program_headers: Vec<goblin::elf::ProgramHeader>,
    pub signature_meta_data: Option<SignatureMetaData>,
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

        Self::do_load_elf_file(&program_headers, &buffer, bus)?;

        let signature_meta_data = Self::get_signature_meta_data(&elf.syms, &elf.strtab);
        Ok(Some(Loader {
            entry_point,
            program_headers,
            signature_meta_data,
        }))
    }

    pub fn entry_point(&self) -> u64 {
        self.entry_point
    }

    pub fn program_headers(&self) -> &[goblin::elf::ProgramHeader] {
        &self.program_headers
    }

    fn do_load_elf_file(
        program_headers: &Vec<ProgramHeader>,
        buffer: &[u8],
        bus: &mut Box<Bus>,
    ) -> Result<(), SimulatorError> {
        for ph in program_headers {
            trace!("Loading program header: {:?}", ph);
            if ph.p_type == goblin::elf::program_header::PT_LOAD {
                let offset = ph.p_offset as usize;
                let file_size = ph.p_filesz as usize;
                let mem_size = ph.p_memsz as usize;
                let vaddr = ph.p_vaddr as usize;

                // assert_eq!(mem_size, file_size);
                trace!(
                    "Loading {} bytes from offset {} to address {:#x}",
                    file_size,
                    offset,
                    vaddr
                );

                for i in 0..file_size {
                    bus.write_byte(vaddr + i, buffer[offset + i])?;
                }

                if mem_size > file_size {
                    for i in file_size..mem_size {
                        bus.write_byte(vaddr + i, 0)?;
                    }
                }
                trace!("Loaded {} bytes to address {:#x} Done", mem_size, vaddr);
            }
        }

        Ok(())
    }

    fn get_signature_meta_data(
        symbol_table: &Symtab,
        strtab: &Strtab,
    ) -> Option<SignatureMetaData> {
        // Try to get "begin_signature" and "end_signature" symbols
        let mut begin_signature: u64 = 0;
        let mut end_signature: u64 = 0;

        // find symbols
        for (_, symbol) in symbol_table.iter().enumerate() {
            let name = strtab.get_at(symbol.st_name).unwrap_or("invalid name");
            if name == "begin_signature" {
                info!("begin_signature value: 0x{:x}", symbol.st_value);
                begin_signature = symbol.st_value;
            } else if name == "end_signature" {
                info!("end_signature value: 0x{:x}", symbol.st_value);
                end_signature = symbol.st_value;
            }
        }

        if begin_signature != 0 && end_signature != 0 {
            Some(SignatureMetaData {
                signature_addr: begin_signature as DeviceAddress,
                signature_len: (end_signature - begin_signature) as usize,
            })
        } else {
            None
        }
    }
}
