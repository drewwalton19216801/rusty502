use crate::emulator::Emulator;
use std::env;

mod emulator;

/**
 * This is the main function for the emulator. It parses the command line arguments and
 * runs the emulator.
 *
 * The command line arguments are as follows:
 *  -r, --rom: The path to the ROM file to load
 *  -a, --address: The address to load the ROM at (default: 0xC000)
 *  -v, --variant: The variant of the CPU to use
 *     - NMOS: The NMOS 6502 CPU
 *     - CMOS: The CMOS 65C02 CPU (default)
 *     - NES: The NES CPU (Ricoh 2A03)
 *  -s, --speed: The speed of the CPU in MHz (default: 0.000100 (100 Hz))
 *  -b, --benchmark: Runs demos/blink.bin for 1000000 cycles and prints the results"
 *  -h, --help: Prints the help message
 */

fn main() {
    // Parse the command line arguments
    let args: Vec<String> = env::args().collect();
    let (rom_path, address, variant, speed, benchmark_mode) = parse_args(args);

    // Create the emulator
    let mut emulator = Emulator::new();
    emulator.init();

    // If benchmark mode is enabled, run the benchmark
    if benchmark_mode {
        emulator.benchmark();
        std::process::exit(0);
    }

    // Load the ROM file
    emulator.load_rom_from_path(&rom_path, address);

    // Change the variant of the CPU
    emulator.change_variant(variant);

    // Run the emulator
    emulator.run(speed, None, false);

    println!();
}

fn parse_args(args: Vec<String>) -> (String, u16, String, f64, bool) {
    // Set the default values
    let mut rom_path = String::from("demos/blink.bin");
    let mut address = 0xC000;
    let mut variant = String::from("CMOS");
    let mut speed: f64 = 0.000100; // 100 Hz
    let mut benchmark_mode = false;

    // Parse the arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-r" | "--rom" => {
                rom_path = args[i + 1].clone();
                i += 1;
            }
            "-a" | "--address" => {
                address = u16::from_str_radix(&args[i + 1], 16).unwrap();
                i += 1;
            }
            "-v" | "--variant" => {
                variant = args[i + 1].clone();
                i += 1;
            }
            "-s" | "--speed" => {
                speed = args[i + 1].parse::<f64>().unwrap();
                i += 1;
            }
            "-b" | "--benchmark" => {
                benchmark_mode = true;
            }
            "-h" | "--help" => {
                print_help();
                std::process::exit(0);
            }
            _ => {
                println!("Invalid argument: {}", args[i]);
                print_help();
                std::process::exit(1);
            }
        }
        i += 1;
    }

    // Return the parsed arguments
    (rom_path, address, variant, speed, benchmark_mode)
}

fn print_help() {
    println!("Usage: emulator [OPTIONS]");
    println!("Options:");
    println!("  -r, --rom: The path to the ROM file to load");
    println!("  -a, --address: The address to load the ROM at (default: 0xC000)");
    println!("  -v, --variant: The variant of the CPU to use");
    println!("     - NMOS: The MMOS 6502 CPU");
    println!("     - CMOS: The CMOS 65C02 CPU (default)");
    println!("     - NES: The NES CPU (Ricoh 2A03)");
    println!("  -s, --speed: The speed of the CPU in MHz (default: 0.000100 (100 Hz))");
    println!(
        "  -b, --benchmark: Runs demos/blink.bin for 200,000,000 cycles and prints the results"
    );
    println!("  -h, --help: Prints the help message");
}
