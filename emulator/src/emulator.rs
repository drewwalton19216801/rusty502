pub mod emulator {
    use std::{cell::RefCell, rc::Rc};

    use cpu::{self, cpu::Cpu};

    // Global variable for the LED strip
    static mut LED_STRIP: [bool; 8] = [false; 8];

    pub struct Emulator {
        pub cpu: Cpu,
    }

    impl Emulator {
        pub fn new() -> Self {
            Self { cpu: Cpu::new() }
        }

        pub fn load_rom_from_path(&mut self, path: &str, address: u16) {
            // Load the rom file into a vector
            let rom = std::fs::read(path).unwrap();

            // Load the rom file into memory
            self.cpu.bus.load_rom_at(&rom, address);
        }

        pub fn change_variant(&mut self, variant: String) {
            // Change the variant of the CPU
            self.cpu
                .change_variant(cpu::cpu::Variant::from_string(variant));
        }

        pub fn run(&mut self) {
            // Register our test write hook
            self.cpu
                .bus
                .add_write_hook(0x6000, Rc::new(RefCell::new(Self::blink_led)));
            self.cpu
                .bus
                .add_write_hook(0x6002, Rc::new(RefCell::new(Self::blink_led)));

            // Change the variant to CMOS
            self.cpu.change_variant(cpu::cpu::Variant::CMOS);

            self.cpu.reset();

            // Run the CPU for 100 cycles
            for _ in 0..1000 {
                self.cpu.clock();
            }
        }

        pub fn benchmark(&mut self) {
            self.load_rom_from_path("demos/blink.bin", 0xC000);
            self.change_variant(String::from("CMOS"));
            self.cpu.reset();

            let num_cycles = 200000000;

            // Start a timer
            let start = std::time::Instant::now();
            // Run the CPU for num_cycles cycles
            for _ in 0..num_cycles {
                self.cpu.clock();
            }
            // Stop the timer
            let end = std::time::Instant::now();

            // Calculate the time elapsed
            let time_elapsed = end.duration_since(start);

            // Calculate the number of cycles per second
            let cycles_per_second = num_cycles as f64 / start.elapsed().as_secs_f64();

            // Calculate the number of instructions per second
            let instructions_per_second = cycles_per_second / 6.0;

            // Convert the number of cycles per second to MHz
            let mhz = cycles_per_second / 1000000.0;

            // Print the results
            println!("Cycles per second: {}", cycles_per_second);
            println!(
                "Average instructions per second*: {}",
                instructions_per_second
            );
            println!("Time elapsed: {:?}", time_elapsed);
            println!("MHz: {}", mhz);
            println!("");
            println!("* This is the average number of instructions per second, as not all instructions take the same number of cycles.");
        }

        pub fn blink_led(address: u16, data: u8) {
            // If the address is 0x6002 and the data is 0xFF, then we want to turn on the LED
            if address == 0x6002 && data == 0xFF {
                unsafe {
                    LED_STRIP[0] = true;
                }
            }

            // If the address is 0x6000, we want to enable the LED bits according to the data
            if address == 0x6000 {
                unsafe {
                    for i in 0..8 {
                        LED_STRIP[i] = (data & (1 << i)) != 0;
                    }
                }
            }

            // Print the LED strip
            unsafe {
                for i in 0..8 {
                    print!("{}", if LED_STRIP[i] { "█" } else { " " });
                }
                println!();
            }
        }
    }
}
