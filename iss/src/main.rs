// Copyright (c) 2024, zhao.shaowei <nsearchf@yeah.net>
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.

// iss/src/main.rs

use std::fs::File;
use std::io::Write;
use std::io::{self, Read};
use std::path::PathBuf;

use clap::{ArgAction, Parser, ValueEnum};

use tracing::{info, trace}; // {debug, error, info, trace, warn};
use tracing_subscriber::fmt::time::UtcTime;
use tracing_subscriber::FmtSubscriber;

use cpu_peripherals::bus::{Bus, DevicePointer};
use cpu_peripherals::{mem::Mem, uart::Uart, DeviceAddress, DeviceSize};
use sim_lib::loader::{self, Loader};
use sim_lib::simulator::Simulator;
use sim_lib::ProgramCounter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn to_tracing_level(&self) -> tracing::Level {
        match self {
            LogLevel::Error => tracing::Level::ERROR,
            LogLevel::Warn => tracing::Level::WARN,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Trace => tracing::Level::TRACE,
        }
    }
}

/// Command line arguments for the RISC-V ISS
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the ELF/bin file
    #[arg(short, long)]
    file_path: String,

    /// Entry point address
    #[arg(short, long, value_parser = parse_hex_address)]
    entry_point: Option<DeviceAddress>,

    /// The path to instruction log file
    #[arg(short, long)]
    instr_file: Option<String>,

    /// The log level for RVV-ISS
    #[arg(short, long, value_enum, default_value_t = LogLevel::Warn)]
    log_level: LogLevel,

    /// If log file of RVV-ISS running with no ansi color
    #[arg(short = 'n', long = "no-ansi", action = ArgAction::SetTrue)]
    no_ansi: bool,

    /// The absolute path to the signature file.
    #[arg(long)]
    rv_arch_test_signature: Option<String>,

    /// Size of each line in signature.
    #[arg(long, default_value_t = 16)]
    rv_arch_test_signature_granularity: u32,

    /// Number of instructions to run.
    #[arg(long)]
    steps: Option<usize>,
}

fn parse_hex_address(s: &str) -> Result<DeviceAddress, std::num::ParseIntError> {
    u64::from_str_radix(s.trim_start_matches("0x"), 16).map(|v| v as DeviceAddress)
}

// const MEMORY_BASE_ADDRESS: DeviceAddress = 0x1_0000;
const FLASH_BASE_ADDRESS: DeviceAddress = 0x8000_0000;
const FLASH_SIZE: DeviceSize = 2 * 1024 * 1024;

const RAM_BASE_ADDRESS: DeviceAddress = 0x8008_0000;
const RAM_SIZE: DeviceSize = 512 * 1024;

// UART0 base address
const UART_BASE_ADDRESS: DeviceAddress = 0x1001_3000;
const UART_SIZE: DeviceSize = 0x1000;

fn init_tracing(args: &Args) {
    let level = args.log_level.to_tracing_level();
    let time_format = time::format_description::parse(
        "[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]",
    )
    .expect("format string should be valid!");
    let timer = UtcTime::new(time_format);

    let subscriber = FmtSubscriber::builder()
        .with_max_level(level)
        .with_ansi(!args.no_ansi)
        .with_timer(timer)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

fn is_elf_file(file_path: &str) -> io::Result<bool> {
    let mut file = File::open(file_path)?;
    let mut buffer = [0u8; 4];

    file.read_exact(&mut buffer)?;

    Ok(&buffer == b"\x7FELF")
}

fn main() {
    let args = Args::parse();

    init_tracing(&args);
    trace!("Starting RISC-V ISS...");

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
    sim.prepare_core_env();
    if let Some(instr_file) = args.instr_file {
        sim.prepare_log_file(&instr_file);
    }

    // step 4. load the ELF/bin program into memory
    let meta_data = load_elf_to_memory(&args.file_path, args.entry_point, &mut sim);

    // step 5. run the simulator
    let start = std::time::Instant::now();
    // None means run until exit, no steps limited
    let r = sim.run(args.steps);
    match r {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Simulation failed: {}", e);
            std::process::exit(1);
        }
    }
    let duration = start.elapsed();
    println!("Target application exit code: {}", sim.get_exit_code());

    // step 6. print the statistics
    let secs = duration.as_secs_f64();
    let instructions = sim.get_run_instrctions();
    // println!("Time elapsed: {:?}, secs {}, instructions {}", duration, secs, instructions);

    println!("Simulation statistics:");
    let ips = instructions as f64 / secs;
    println!(
        "\tIPS(Instructions Per Second): {:.2} KIPS, {:.2} MIPS",
        ips / (1000 as f64),
        ips / ((1000 * 1000) as f64)
    );
    println!("Rust RISC-V ISS has finished running.");

    // step 7. dump signature
    if let Some(file_path) = args.rv_arch_test_signature {
        dump_signature_file(
            args.rv_arch_test_signature_granularity as usize,
            meta_data,
            &sim,
            &file_path,
        );
    }
}

fn load_elf_to_memory(
    file_path_str: &str,
    entry_point: Option<usize>,
    sim: &mut Simulator,
) -> Option<loader::SignatureMetaData> {
    let file_path = PathBuf::from(file_path_str);
    info!("ELF/bin file path: {:?}", file_path);

    if let Ok(is_elf) = is_elf_file(file_path_str) {
        if is_elf {
            let loader = Loader::load_elf_file(file_path.as_path(), sim.get_bus_mut())
                .unwrap()
                .unwrap();

            assert_eq!(loader.entry_point(), 0x8000_0000, "Unexpected entry point");
            let entry_point = loader.entry_point();
            sim.set_reset_vector(entry_point as ProgramCounter);

            loader.signature_meta_data
        } else {
            match entry_point {
                Some(entry_point) => {
                    let mut file = File::open(file_path_str).expect("Failed to open binary file");
                    let mut buffer = Vec::new();
                    file.read_to_end(&mut buffer)
                        .expect("Failed to read binary file");
                    let _ = sim.load_bin_program(&buffer, FLASH_BASE_ADDRESS);
                    sim.set_reset_vector(entry_point as ProgramCounter);
                    None
                }
                None => {
                    eprintln!("Error: For non-ELF files, the entry point must be specified.");
                    std::process::exit(1);
                }
            }
        }
    } else {
        eprintln!("Error: Open or read file({}) failed.", file_path_str);
        std::process::exit(1);
    }
}

fn dump_signature_file(
    signature_granularity: usize,
    meta_data: Option<loader::SignatureMetaData>,
    sim: &Simulator,
    file_path: &str,
) {
    if let Some(meta_data) = meta_data {
        if meta_data.signature_len == 0 {
            return;
        }

        let size_of_each_line: usize = signature_granularity;
        let bus = sim.get_bus();
        let signature_data = bus
            .read(meta_data.signature_addr, meta_data.signature_len)
            .expect("Failed to read signature data from target memory.");

        // Open the file in write mode
        let file = PathBuf::from(&file_path);
        let mut file = File::create(&file).expect("Failed to create signature file");

        // Write the signature data to the file in reverse order
        for chunk in signature_data.chunks(size_of_each_line) {
            let hex_string = chunk
                .iter()
                .rev()
                .map(|byte| format!("{:02x}", byte))
                .collect::<Vec<String>>()
                .join("");
            writeln!(file, "{}", hex_string).expect("Failed to write to file");
        }

        info!("Signature data written to file: {:?}", file_path);
    }
}
