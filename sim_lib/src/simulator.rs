// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// sim_lib/src/simulator.rs

use std::{fs::File, io::Write, path::Path};
use tracing::{error, info, trace, warn};

use cpu_peripherals::{
    bus::{Bus, DevicePointer},
    DeviceAddress, DeviceSize,
};

use rv_core::{
    clint::Clint,
    core::Core,
    decode::{decoder::Decoder, DecodedInstruction, ExecutionReturnData},
    fetch::Fetcher,
    inst_csr_reg::CSR_MIP,
    GprSigned, MachineInstruction, ProgramCounter, RvCoreError,
};

use crate::loader::Loader;
use crate::SimulatorError;

const CLINT_BASE_ADDRESS: DeviceAddress = 0x200_0000;
const CLINT_SIZE: DeviceSize = 0xc000;

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

    pub fn prepare_core_env(&mut self) {
        if self.bus.find_clint_device(CLINT_BASE_ADDRESS).is_ok() {
            warn!("CLINT device already exists, skip creating a new one");
        } else {
            let clint = DevicePointer::new(Clint::new(self.core.clone_csr()));
            let _ = self
                .bus
                .add_clint_device(CLINT_BASE_ADDRESS, CLINT_SIZE, clint);
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

        self.run_instrctions = self.run_instrctions.wrapping_add(1);

        // step 4. check interrupt
        // step 4.1 invoke clint.tick()
        let mip_prev = self.core.read_csr(CSR_MIP).expect("read mip failed");
        let mut mip = mip_prev;
        let clint = self
            .bus
            .find_clint_device_mut(CLINT_BASE_ADDRESS)
            .expect("find clint failed");
        clint.tick(&mut mip);
        if mip != mip_prev {
            self.core
                .write_csr(CSR_MIP, mip)
                .expect("Write to mip failed!");

            if let Some(trap) = self.core.get_interrutp() {
                if let Err(err) = self.core.set_trap(trap, pc) {
                    warn!("Set trap.interrupt failed: {:?}", err)
                }
            }
        }
        // TODO: step 4.2 check external interrupt

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

            self.core.handle_trap(&trap, new_pc, &mut self.log_file)?
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
                                disasm.to_lowercase()
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

#[cfg(test)]
mod tests {
    use super::*;
    use cpu_peripherals::DeviceType;

    #[test]
    fn test_clint_in_sim() {
        let bus = Bus::new();
        let mut sim = Simulator::new(bus);
        sim.prepare_core_env();

        assert_eq!(
            sim.bus
                .find_clint_device(CLINT_BASE_ADDRESS)
                .unwrap()
                .get_type(),
            DeviceType::Clint
        );
    }
}
