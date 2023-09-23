use std::env;

mod emulator;

/**
 * This is the main function for the emulator. It parses the command line arguments and
 * runs the emulator.
 * 
 * The command line arguments are as follows:
 *  -r, --rom: The path to the ROM file to load
 *  -a, --address: The address to load the ROM at (default: 0x8000)
 *  -v, --variant: The variant of the CPU to use
 *     - NMOS: The CMOS 6502 CPU
 *     - CMOS: The NMOS 65C02 CPU (default)
 *     - NES: The NES CPU (Ricoh 2A03)
 *  -h, --help: Prints the help message
 */

fn main() {
    // Parse the command line arguments
    let args: Vec<String> = env::args().collect();
    let (rom_path, address, variant) = parse_args(args);

    // Create the emulator
    let mut emulator = emulator::emulator::Emulator::new();

    // Load the ROM file
    emulator.load_rom_from_path(&rom_path, address);

    // Change the variant of the CPU
    emulator.change_variant(variant);

    // Run the emulator
    emulator.run();
    
    /*
    let mut emulator = emulator::emulator::Emulator::new();
    emulator.load_file_from_path("demos/blink.bin");
    emulator.run();
    */
}

fn parse_args(args: Vec<String>) -> (String, u16, String) {
    // Set the default values
    let mut rom_path = String::from("demos/blink.bin");
    let mut address = 0x8000;
    let mut variant = String::from("CMOS");

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
    (rom_path, address, variant)
}

fn print_help() {
    println!("Usage: emulator [OPTIONS]");
    println!("Options:");
    println!("  -r, --rom: The path to the ROM file to load");
    println!("  -a, --address: The address to load the ROM at (default: 0x8000)");
    println!("  -v, --variant: The variant of the CPU to use");
    println!("     - NMOS: The CMOS 6502 CPU");
    println!("     - CMOS: The NMOS 65C02 CPU (default)");
    println!("     - NES: The NES CPU (Ricoh 2A03)");
    println!("  -h, --help: Prints the help message");
}