// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// tests/tests/exec_asm_instr_of_md5.rs

use rv_core::GprSigned;
use rv_core::GprUnsigned;
// use rv_core::{GprSigned, GprUnsigned};
use rv_core::inst_csr_reg::*;
use rv_core::ProgramCounter;

mod common;

use cpu_peripherals::{
    bus::{Bus, DevicePointer},
    mem::Mem,
    DeviceAddress, DeviceSize,
};
use sim_lib::simulator::Simulator;

const MEMORY_BASE_ADDRESS: DeviceAddress = 0x1_0000;
const MEMORY_SIZE: DeviceSize = 0x100;

fn creat_sim_for_test(instr: u32) -> Simulator {
    // step 1. create a bus
    let mut bus = Bus::new();

    // step 2. create memory and add it to the bus
    let memory = DevicePointer::new(Mem::new(MEMORY_SIZE));
    let _ = bus.add_device(MEMORY_BASE_ADDRESS, MEMORY_SIZE, memory);

    let mut sim = Simulator::new(bus);

    // step 3. load one instrucion into memory
    let add_instr = instr.to_le_bytes();
    let _ = sim.load_bin_program(&add_instr, MEMORY_BASE_ADDRESS);

    // step 3. set the reset vector
    sim.set_reset_vector(MEMORY_BASE_ADDRESS.try_into().unwrap());

    sim
}

// 80000000:	00100117          	auipc	sp,0x100
// 80000004:	00010113          	mv	sp,sp
// 8000000c:	04028293          	addi	t0,t0,64 # 80000048 <trap_vector>
// 80000010:	30529073          	csrw	mtvec,t0
// 80000024:	00000393          	li	t2,0
// 80000028:	0062f863          	bgeu	t0,t1,80000038 <bss_cleared>
// 8000002c:	0072a023          	sw	t2,0(t0)
// 80000034:	ff5ff06f          	j	80000028 <clear_bss>
// 80000038:	0e9000ef          	jal	80000920 <main>
// 80000040:	00000073          	ecall
// 80000060:	fec42783          	lw	a5,-20(s0)
// 80000068:	00e79733          	sll	a4,a5,a4
// 80000070:	40d006b3          	neg	a3,a3
// 80000074:	01f6f693          	andi	a3,a3,31
// 80000078:	00d7d7b3          	srl	a5,a5,a3
// 8000007c:	00e7e7b3          	or	a5,a5,a4
// 8000008c:	00008067          	ret
// 800000b8:	67452737          	lui	a4,0x67452
// 800000f4:	00000013          	nop
// 80000150:	01070633          	add	a2,a4,a6
// 80000158:	00e5b5b3          	sltu	a1,a1,a4
// 8000019c:	00074703          	lbu	a4,0(a4)
// 800001a8:	00e78c23          	sb	a4,24(a5)
// 800001b4:	0c079463          	bnez	a5,8000027c <md5Update+0x178>
// 800001c4:	00279793          	slli	a5,a5,0x2
// 80000290:	eef76ae3          	bltu	a4,a5,80000184 <md5Update+0x80>
// 80000304:	40f707b3          	sub	a5,a4,a5
// 80000440:	01d75693          	srli	a3,a4,0x1d
// 800005ec:	08e78863          	beq	a5,a4,8000067c <md5Step+0xec>
// 800005f8:	00078863          	beqz	a5,80000608 <md5Step+0x78>
// 80000610:	00f77733          	and	a4,a4,a5
// 80000618:	fff7c693          	not	a3,a5
// 80000684:	00f747b3          	xor	a5,a4,a5
// 800009a0:	08d7ce63          	blt	a5,a3,80000a3c <memcpy+0xcc>
// 800009fc:	fee898e3          	bne	a7,a4,800009ec <memcpy+0x7c>
// 80000b60:	00a03533          	snez	a0,a0
// 800800c8:	0000000f          	fence	unknown,unknown

//     0x00100117,      // auipc	sp,0x100
#[test]
fn test_execution_auipc() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00100117);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    assert_eq!(core.get_pc(), MEMORY_BASE_ADDRESS as ProgramCounter);
    // let _ = core.write_reg_by_name("sp", 10).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    let exepect_val = MEMORY_BASE_ADDRESS as ProgramCounter + (0x100 << 12);
    assert_eq!(core.read_reg_by_name("sp"), Ok(exepect_val));
    assert_eq!(core.get_pc(), MEMORY_BASE_ADDRESS as ProgramCounter + 4)
}

//     0x00010113,      // mv	sp,sp
#[test]
fn test_execution_mv() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00010113);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    assert_eq!(core.get_pc(), MEMORY_BASE_ADDRESS as ProgramCounter);

    let exepect_val = 10;
    let _ = core.write_reg_by_name("sp", exepect_val).unwrap();
    assert_eq!(core.read_reg_by_name("sp"), Ok(exepect_val));

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("sp"), Ok(exepect_val));
}

//     0x04028293,      // addi	t0,t0,64 # 80000048 <trap_v
#[test]
fn test_execution_addi() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x04028293);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("t0", 10).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("t0"), Ok(74));
}

// `csrw csr, rs1` Expands to csrrw x0, csr, rs1.
//     0x30529073,      // csrw	mtvec,t0
#[test]
fn test_execution_csrw() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x30529073);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("t0", 0x5678).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_csr(CSR_MTVEC), Ok(0x5678));
}

//     0x00000393,      // li	t2,0
#[test]
fn test_execution_li() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00000393);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("t0", 10).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("t2"), Ok(0));
}

// Branch if Greater Than or Equal, Unsigned. B-type,
//     0x0062f863,      // bgeu	t0,t1,80000038 <bss_cleared
#[test]
fn test_execution_bgeu() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x0062f863);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("t0", 10).unwrap();
    let _ = core.write_reg_by_name("t1", 10).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.get_pc(), MEMORY_BASE_ADDRESS as ProgramCounter + 16);

    // t0 > t1
    {
        sim.set_reset_vector(MEMORY_BASE_ADDRESS.try_into().unwrap());
        let core = sim.get_core_mut();
        let _ = core.write_reg_by_name("t1", 9).unwrap();
        sim.run(Some(1)).expect("Simulation failed");
        let core = sim.get_core();
        assert_eq!(core.get_pc(), MEMORY_BASE_ADDRESS as ProgramCounter + 16);
    }

    // t0 < t1
    {
        sim.set_reset_vector(MEMORY_BASE_ADDRESS.try_into().unwrap());
        let core = sim.get_core_mut();
        let _ = core.write_reg_by_name("t1", 11).unwrap();
        sim.run(Some(1)).expect("Simulation failed");
        let core = sim.get_core();
        assert_eq!(core.get_pc(), MEMORY_BASE_ADDRESS as ProgramCounter + 4);
    }
}

//     0x0072a023,      // sw	t2,0(t0)
#[test]
fn test_execution_sw() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x0072a023);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("t2", 0x1234abcd).unwrap();
    let mem_addr = MEMORY_BASE_ADDRESS as ProgramCounter + 8;
    let _ = core.write_reg_by_name("t0", mem_addr).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let bus = sim.get_bus();
    assert_eq!(bus.read_word(mem_addr as DeviceAddress), Ok(0x1234abcd));
}

// Expands to jal x0, offset.
//     0xff5ff06f,      // j	80000028 <clear_bss>
#[test]
fn test_execution_j() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0xff5ff06f);

    // step 2. prepare the environment
    // let core = sim.get_core_mut();
    // let _ = core.write_reg_by_name("t0", 10).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    let mut pc = MEMORY_BASE_ADDRESS as ProgramCounter;
    pc = pc.wrapping_add(-12i32 as ProgramCounter);

    assert_eq!(core.get_pc(), pc);
}

//     0x0e9000ef,      // jal	80000920 <main>
#[test]
fn test_execution_jal() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x0e9000ef);

    // step 2. prepare the environment
    // let core = sim.get_core_mut();
    // let _ = core.write_reg_by_name("t0", 10).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    let mut pc = MEMORY_BASE_ADDRESS as ProgramCounter;
    pc = pc.wrapping_add(2280 as ProgramCounter);

    assert_eq!(core.get_pc(), pc);
    assert_eq!(
        core.read_reg_by_name("ra"),
        Ok(MEMORY_BASE_ADDRESS as ProgramCounter + 4)
    );
}

//     0x00000073,      // ecall
// #[test]
// fn test_execution_5() {
//     // common::setup_tracing();

//     // step 1. create a simulator
//     let mut sim = creat_sim_for_test(0x04028293);

//     // step 2. prepare the environment
//     let core = sim.get_core_mut();
//     let _ = core.write_reg_by_name("t0", 10).unwrap();

//     // step 3. run the simulator
//     sim.run(Some(1)).expect("Simulation failed");

//     // step 4. check the result
//     let core = sim.get_core();
//     assert_eq!(core.read_reg_by_name("t0"), Ok(74));
// }

//     0xfec42783,      // lw	a5,-20(s0)
#[test]
fn test_execution_lw() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0xfec42783);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let s0_val = MEMORY_BASE_ADDRESS as ProgramCounter + 40;
    let mem_addr = s0_val - 20;
    let _ = core.write_reg_by_name("s0", s0_val).unwrap();
    let bus = sim.get_bus_mut();
    _ = bus
        .write_word(mem_addr.try_into().unwrap(), 0xabcd_1234)
        .unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a5"), Ok(0xabcd_1234));
}

// Shift Left Logical. R-type, RV32I and RV64I.
//     0x00e79733,      // sll	a4,a5,a4
#[test]
fn test_execution_sll() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00e79733);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a5", 0x1234).unwrap();
    let _ = core.write_reg_by_name("a4", 16).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a4"), Ok(0x1234_0000));
}

//     0x40d006b3,      // neg	a3,a3
#[test]
fn test_execution_neg() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x40d006b3);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let val = -10_i32;
    let _ = core.write_reg_by_name("a3", val as GprUnsigned).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a3"), Ok((-val) as GprUnsigned));
}

//     0x01f6f693,      // andi	a3,a3,31
#[test]
fn test_execution_andi() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x01f6f693);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a3", 0x1f1f).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a3"), Ok(0x1f));
}

//     0x00d7d7b3,      // srl	a5,a5,a3
#[test]
fn test_execution_srl() {
    common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00d7d7b3);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a5", 0x1234_0000).unwrap();
    let _ = core.write_reg_by_name("a3", 16).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a5"), Ok(0x1234));
}

//     0x00e7e7b3,      // or	a5,a5,a4
#[test]
fn test_execution_or() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00e7e7b3);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a5", 0x12345a5a).unwrap();
    let _ = core.write_reg_by_name("a4", 0x1234a5a5).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a5"), Ok(0x1234ffff));
}

//     0x00008067,      // ret
// #[test]
// fn test_execution_c() {
//     // common::setup_tracing();

//     // step 1. create a simulator
//     let mut sim = creat_sim_for_test(0x04028293);

//     // step 2. prepare the environment
//     let core = sim.get_core_mut();
//     let _ = core.write_reg_by_name("t0", 10).unwrap();

//     // step 3. run the simulator
//     sim.run(Some(1)).expect("Simulation failed");

//     // step 4. check the result
//     let core = sim.get_core();
//     assert_eq!(core.read_reg_by_name("t0"), Ok(74));
// }

// Load Upper Immediate.
//     0x67452737,      // lui	a4,0x67452
#[test]
fn test_execution_lui() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x67452737);

    // step 2. prepare the environment
    // let core = sim.get_core_mut();
    // let _ = core.write_reg_by_name("t0", 10).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a4"), Ok(0x67452 << 12));
}

//     0x00000013,      // nop
// #[test]
// fn test_execution_nop() {
//     common::setup_tracing();

//     // step 1. create a simulator
//     let mut sim = creat_sim_for_test(0x00000013);

//     // step 2. prepare the environment
//     let core = sim.get_core_mut();
//     let _ = core.write_reg_by_name("t0", 10).unwrap();

//     // step 3. run the simulator
//     sim.run(Some(1)).expect("Simulation failed");

//     // step 4. check the result
//     let core = sim.get_core();
//     assert_eq!(core.read_reg_by_name("t0"), Ok(74));
// }

//     0x01070633,      // add	a2,a4,a6
#[test]
fn test_execution_add() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x01070633);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a4", 4).unwrap();
    let _ = core.write_reg_by_name("a6", 6).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a2"), Ok(10));
}

// Set if Less Than, Unsigned.
//     0x00e5b5b3,      // sltu	a1,a1,a4
#[test]
fn test_execution_sltu() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00e5b5b3);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a1", 1).unwrap();
    let _ = core.write_reg_by_name("a4", 4).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a1"), Ok(1));

    // a1 == a4
    {
        sim.set_reset_vector(MEMORY_BASE_ADDRESS.try_into().unwrap());
        let core = sim.get_core_mut();
        let _ = core.write_reg_by_name("a1", 4).unwrap();
        sim.run(Some(1)).expect("Simulation failed");
        let core = sim.get_core();
        assert_eq!(core.read_reg_by_name("a1"), Ok(0));
    }

    // a1 > a4
    {
        sim.set_reset_vector(MEMORY_BASE_ADDRESS.try_into().unwrap());
        let core = sim.get_core_mut();
        let _ = core.write_reg_by_name("a1", 5).unwrap();
        sim.run(Some(1)).expect("Simulation failed");
        let core = sim.get_core();
        assert_eq!(core.read_reg_by_name("a1"), Ok(0));
    }
}

// Load Byte, Unsigned. I-type, RV32I and RV64I.
//     0x00074703,      // lbu	a4,0(a4)
#[test]
fn test_execution_lbu() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00074703);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let mem_addr = MEMORY_BASE_ADDRESS as ProgramCounter + 40;
    let _ = core.write_reg_by_name("a4", mem_addr).unwrap();
    let bus = sim.get_bus_mut();
    let _ = bus.write_byte(mem_addr as DeviceAddress, 0xab).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    // let core = sim.get_core();
    let bus = sim.get_bus();
    assert_eq!(bus.read_byte(mem_addr as DeviceAddress).unwrap(), 0xab);
}

//     0x00e78c23,      // sb	a4,24(a5)
#[test]
fn test_execution_sb() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00e78c23);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let a5_val = MEMORY_BASE_ADDRESS as ProgramCounter + 40;
    let mem_addr = a5_val + 24;
    let _ = core.write_reg_by_name("a4", 0x12ab).unwrap();
    let _ = core.write_reg_by_name("a5", a5_val).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let bus = sim.get_bus();
    assert_eq!(bus.read_byte(mem_addr as DeviceAddress).unwrap(), 0xab);
}

// Branch if Not Equal to Zero. Expands to bne rs1, x0, offset.
//     0x0c079463,      // bnez	a5,8000027c <md5Update+0x17
#[test]
fn test_execution_bnez() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x0c079463);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a5", 10).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(
        core.get_pc(),
        (MEMORY_BASE_ADDRESS as ProgramCounter + 0xc8)
    );

    {
        sim.set_reset_vector(MEMORY_BASE_ADDRESS.try_into().unwrap());
        let core = sim.get_core_mut();
        let _ = core.write_reg_by_name("a5", 0).unwrap();
        sim.run(Some(1)).expect("Simulation failed");
        let core = sim.get_core();
        assert_eq!(core.get_pc(), (MEMORY_BASE_ADDRESS as ProgramCounter + 4));
    }
}

// Shift Left Logical Immediate. I-type, RV32I and RV64I.
//     0x00279793,      // slli	a5,a5,0x2
#[test]
fn test_execution_slli() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00279793);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a5", 10).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a5"), Ok(10 << 2));
}

// Branch if Less Than, Unsigned. B-type, RV32I and RV64I.
//     0xeef76ae3,      // bltu	a4,a5,80000184 <md5Update+0
#[test]
fn test_execution_bltu() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0xeef76ae3);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a4", 4).unwrap();
    let _ = core.write_reg_by_name("a5", 5).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    let offset = -268_i32;
    let mut new_pc = MEMORY_BASE_ADDRESS as ProgramCounter;
    new_pc = new_pc.wrapping_add(offset as u32);
    assert_eq!(core.get_pc(), new_pc);
}

//     0x40f707b3,      // sub	a5,a4,a5
#[test]
fn test_execution_sub() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x40f707b3);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a4", 4).unwrap();
    let _ = core.write_reg_by_name("a5", 5).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(
        core.read_reg_by_name("a5"),
        Ok(-1 as GprSigned as GprUnsigned)
    );
}

// Shift Right Logical Immediate. I-type, RV32I and RV64I.
//     0x01d75693,      // srli	a3,a4,0x1d
#[test]
fn test_execution_srli() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x01d75693);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    // let _ = core.write_reg_by_name("a3", 10).unwrap();
    let _ = core.write_reg_by_name("a4", 0x8000_0000).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a3"), Ok(0x8000_0000 >> 0x1d));
}

//     0x08e78863,      // beq	a5,a4,8000067c <md5Step+0xec>
#[test]
fn test_execution_beq() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x08e78863);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a4", 10).unwrap();
    let _ = core.write_reg_by_name("a5", 10).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.get_pc(), MEMORY_BASE_ADDRESS as ProgramCounter + 0x90);
}

// Branch if Equal to Zero.
//     0x00078863,      // beqz	a5,80000608 <md5Step+0x78>
#[test]
fn test_execution_beqz() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00078863);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a5", 0).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.get_pc(), MEMORY_BASE_ADDRESS as ProgramCounter + 16);
}

//     0x00f77733,      // and	a4,a4,a5
#[test]
fn test_execution_and() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00f77733);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a4", 0x1234a5a5).unwrap();
    let _ = core.write_reg_by_name("a5", 0x12345a5a).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a4"), Ok(0x1234_0000));
}

// NOT. Pseudoinstruction, RV32I and RV64I.
// Writes the ones’ complement of x[rs1] to x[rd]. Expands to xori rd, rs1, -1.
//     0xfff7c693,      // not	a3,a5
#[test]
fn test_execution_not() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0xfff7c693);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a5", 0xa5a55a5a).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a3"), Ok(0x5a5aa5a5));
}

//     0x00f747b3,      // xor	a5,a4,a5
#[test]
fn test_execution_xor() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00f747b3);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a4", 0xa5a55a5a).unwrap();
    let _ = core.write_reg_by_name("a5", 0x5a5aa5a5).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a5"), Ok(0xffff_ffff));
}

//     0x08d7ce63,      // blt	a5,a3,80000a3c <memcpy+0xcc>
#[test]
fn test_execution_blt() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x08d7ce63);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a5", 10).unwrap();
    let _ = core.write_reg_by_name("a3", 13).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.get_pc(), MEMORY_BASE_ADDRESS as ProgramCounter + 156);
}

//     0xfee898e3,      // bne	a7,a4,800009ec <memcpy+0x7c>
#[test]
fn test_execution_bne() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0xfee898e3);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a7", 7).unwrap();
    let _ = core.write_reg_by_name("a4", 4).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    let mut new_pc = MEMORY_BASE_ADDRESS as ProgramCounter;
    new_pc = new_pc.wrapping_add(-16_i32 as u32 as ProgramCounter);
    assert_eq!(core.get_pc(), new_pc);
}

// Set if Not Equal to Zero.
//     0x00a03533,      // snez	a0,a0
#[test]
fn test_execution_snez() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x00a03533);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("a0", 10).unwrap();

    // step 3. run the simulator
    sim.run(Some(1)).expect("Simulation failed");

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("a0"), Ok(1));
}

//     0x0000000f,      // fence	unknown,unknown
#[test]
fn test_execution_fence() {
    // common::setup_tracing();

    // step 1. create a simulator
    let mut sim = creat_sim_for_test(0x0000000f);

    // step 2. prepare the environment
    let core = sim.get_core_mut();
    let _ = core.write_reg_by_name("t0", 10).unwrap();

    // step 3. run the simulator
    // sim.run(Some(1)).expect("Simulation failed"); // TODO

    // step 4. check the result
    let core = sim.get_core();
    assert_eq!(core.read_reg_by_name("t0"), Ok(10));
}
