// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// sim_lib/src/simulator.rs

use std::{fs::File, io::Write, path::Path};
use tracing::{error, info, trace};

use cpu_peripherals::{bus::Bus, DeviceAddress};
use rv_core::{
    core::Core,
    decode::{decoder::Decoder, DecodedInstruction, ExecutionReturnData},
    fetch::Fetcher,
    GprSigned, MachineInstruction, ProgramCounter, RvCoreError,
};

use crate::loader::Loader;
use crate::SimulatorError;

pub struct Simulator {
    core: Core,
    decoder: Decoder,
    bus: Box<Bus>,
    exit_code: GprSigned,
    log_file: Option<File>,
    run_instrctions: u64,
}

impl Simulator {
    pub fn new(bus: Bus) -> Self {
        info!("Creating a new simulator");
        Self {
            core: Core::new(),
            decoder: Decoder::new(),
            bus: Box::new(bus),
            exit_code: 0,
            log_file: None,
            run_instrctions: 0,
        }
    }

    pub fn prepare_log_file(&mut self, file_path: &str) {
        let err_str = format!("Failed to open log file({})", file_path);
        let file = File::create(file_path).expect(&err_str);
        // let file = File::open(file_path).expect(&err_str);
        self.log_file = Some(file);
    }

    pub fn load_elf_file(&mut self, elf_file: &Path) -> Result<(), SimulatorError> {
        let _ = Loader::load_elf_file(elf_file, &mut self.bus);
        Ok(())
    }

    pub fn load_bin_file(
        &mut self,
        bin_file: &Path,
        base_addr: DeviceAddress,
    ) -> Result<(), SimulatorError> {
        let _ = Loader::load_bin_file(bin_file, &mut self.bus, base_addr);
        Ok(())
    }

    pub fn load_bin_program(
        &mut self,
        bin_program: &[u8],
        base_addr: DeviceAddress,
    ) -> Result<(), SimulatorError> {
        let _ = Loader::load_bin_program(bin_program, &mut self.bus, base_addr);
        Ok(())
    }

    pub fn set_reset_vector(&mut self, pc: ProgramCounter) {
        self.core.set_pc(pc);
    }
    pub fn run(&mut self, steps: Option<usize>) -> Result<(), SimulatorError> {
        if steps.is_none() {
            loop {
                if self.exit_code != 0 {
                    info!(
                        "Target APP exit with code: {}({:#x})",
                        self.get_exit_code(),
                        self.get_exit_code()
                    );
                    break Ok(());
                }
                self.step()?
            }
        } else {
            for _ in 0..steps.unwrap() {
                if self.exit_code != 0 {
                    info!(
                        "Target APP exit with code: {}({:#x})",
                        self.get_exit_code(),
                        self.get_exit_code()
                    );
                    break;
                }
                self.step()?
            }
            Ok(())
        }
    }

    // just for test
    pub fn get_core(&self) -> &Core {
        &self.core
    }

    pub fn get_core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    pub fn get_bus(&self) -> &Box<Bus> {
        &self.bus
    }

    pub fn get_bus_mut(&mut self) -> &mut Box<Bus> {
        &mut self.bus
    }

    pub fn get_run_instrctions(&self) -> u64 {
        self.run_instrctions
    }
    
    fn step(&mut self) -> Result<(), SimulatorError> {
        let pc = self.core.get_pc();
        trace!("PC: {:#010x}", pc);
        let mem = self.bus.find_device(pc.try_into().unwrap())?;

        // step 1. Fetch instruction
        let instruction = Fetcher::fetch(pc, mem)?;
        trace!("Instruction: {:#010x}", instruction);

        // step 2. Decode instruction
        let decoded_instruction = self.decoder.decode(instruction)?;

        // step 3. Execute instruction
        let mut ret_data = self.execute(&decoded_instruction, instruction)?;

        self.run_instrctions += 1;

        // step 4. check interrupt TODO: // mie, mip

        // step 5. process trap
        ret_data = if let Some(trap) = self.core.take_trap() {
            let new_pc = self.calc_new_pc(ret_data);

            if Core::is_ecall(&trap) {
                let a7 = self.core.read_reg_by_name("a7")?;
                if a7 == 93 {
                    let a0 = self.core.read_reg_by_name("a0")?;
                    self.set_exit_code(a0 as GprSigned);
                }
            }

            self.core.handle_trap(&trap, new_pc)?
        } else {
            ret_data
        };

        // step 6. update PC
        self.update_pc(ret_data);

        Ok(())
    }

    fn set_exit_code(&mut self, code: GprSigned) {
        self.exit_code = code << 1 | 1;
    }

    pub fn get_exit_code(&self) -> GprSigned {
        self.exit_code >> 1
    }

    fn execute(
        &mut self,
        instr: &DecodedInstruction,
        instruction: MachineInstruction,
    ) -> Result<Option<ExecutionReturnData>, RvCoreError> {
        let disasm = self.log_file.is_some();
        let rdata = (instr.execute)(instruction, &mut self.core, &mut self.bus, disasm);

        if disasm {
            if let Ok(Some(rdata)) = rdata.as_ref() {
                if let Some(disasm) = rdata.disasm.as_ref() {
                    if let Some(log_file) = self.log_file.as_mut() {
                        log_file
                            .write_fmt(format_args!(
                                "{:#010x} ({:#010x}) {}\n",
                                self.core.get_pc(),
                                instruction,
                                disasm
                            ))
                            .unwrap_or_else(|e| {
                                error!("Failed to write log: {}", e);
                            });
                    }
                }
            }
        }

        rdata
    }

    fn calc_new_pc(&self, ret_data: Option<ExecutionReturnData>) -> ProgramCounter {
        if let Some(rdata) = ret_data {
            if let Some(pc) = rdata.pc {
                pc
            } else {
                self.core.get_pc().wrapping_add(4)
            }
        } else {
            self.core.get_pc().wrapping_add(4)
        }
    }
    fn update_pc(&mut self, ret_data: Option<ExecutionReturnData>) {
        let new_pc = self.calc_new_pc(ret_data);
        trace!("New PC: {:#010x}", new_pc);
        self.core.set_pc(new_pc);
    }
}
