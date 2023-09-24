pub mod emulator {
    use std::io::Write;
    use std::sync::{Arc, Mutex};
    use std::thread;

    use cpu::{self, cpu::Cpu};
    use cpu::bus::bus::Hook;

    // Global variable for the LED strip
    static mut LED_STRIP: [bool; 8] = [false; 8];

    pub struct Emulator {
        pub cpu: Cpu,
        cpu_speed_hz: f64,
    }

    impl Emulator {
        pub fn new() -> Self {
            Self {
                cpu: Cpu::new(),
                cpu_speed_hz: 0_000_000.0,
            }
        }

        #[allow(dead_code)]
        pub fn change_speed(&mut self, speed: f64) {
            self.cpu_speed_hz = speed;
        }

        #[allow(dead_code)]
        pub fn change_speed_mhz(&mut self, speed: f64) {
            self.cpu_speed_hz = speed * 1_000_000.0;
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

        // Run the emulator for a certain number of cycles (optional)
        pub fn run(&mut self, speed_mhz: f64, num_cycles: Option<u64>) {
            let shared_cpu = Arc::new(Mutex::new(self.cpu.clone()));

            let mut cycles_left = num_cycles.unwrap_or(u64::MAX);

            shared_cpu
                .lock()
                .unwrap()
                .bus
                .add_hook_range(
                    0x6000,
                    0x6002,
                    Hook {
                        read: None,
                        write: Some(Arc::new(Mutex::new(Emulator::blink_led))),
                    },
                );

            // Change the variant to CMOS
            shared_cpu
                .lock()
                .unwrap()
                .change_variant(cpu::cpu::Variant::CMOS);

            shared_cpu.lock().unwrap().reset();

            // Calculate the number of cycles to run per second
            let cycles_per_second = speed_mhz * 1_000_000.0;

            // Spawn a new thread to run the CPU
            let cpu_thread = thread::spawn({
                let shared_cpu = shared_cpu.clone();
                move || while cycles_left > 0 {
                    shared_cpu.lock().unwrap().clock();
                    cycles_left -= 1;
                    std::thread::sleep(std::time::Duration::from_secs_f64(1.0 / cycles_per_second));
                }
            });

            // Wait for the CPU thread to finish
            cpu_thread.join().unwrap();
        }

        pub fn benchmark(&mut self) {
            self.load_rom_from_path("demos/blink.bin", 0xC000);
            self.change_variant(String::from("CMOS"));
            self.cpu.reset();

            let num_cycles = 200000000;

            // Spawn a new thread to run the CPU
            let cpu_thread = thread::spawn({
                let mut shared_cpu = self.cpu.clone();
                move || {
                    for _ in 0..num_cycles {
                        shared_cpu.clock();
                    }
                }
            });

            // Start a timer
            let start = std::time::Instant::now();

            // Wait for the CPU thread to finish
            cpu_thread.join().unwrap();

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

            // Clear the line and print the LED strip
            print!("\x1B[K");
            print!("\rLED STRIP: ");
            for i in 0..8 {
                if unsafe { LED_STRIP[i] } {
                    print!("█");
                } else {
                    print!("░");
                }
            }

            // Flush stdout
            std::io::stdout().flush().unwrap();
        }
    }
}
