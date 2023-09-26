use std::sync::{Arc, Mutex, RwLock};
use std::thread;

use cpu::{self, cpu::Cpu};

use self::bus::Bus;

pub mod bus;
pub mod devices;

pub struct Emulator {
    pub cpu: Cpu,
    cpu_speed_hz: f64,
    pub bus: bus::Bus,
}
impl Emulator {
    pub fn new() -> Self {
        Self {
            cpu: Cpu::new(),
            cpu_speed_hz: 0_000_000.0,
            bus: Bus::new(),
        }
    }

    pub fn init(&mut self) {
        self.bus = Bus::new();
        self.cpu_speed_hz = 0_000_000.0;
        self.cpu = Cpu::new();
        self.init_bus();
    }

    pub fn init_bus(&mut self) {
        let read_byte_fn = Arc::new(Mutex::new({
            let mut bus = self.bus.clone();
            move |address: u16| -> u8 { bus.read_byte(address) }
        }));

        let write_byte_fn = Arc::new(Mutex::new({
            let mut bus = self.bus.clone();
            move |address: u16, value: u8| bus.write_byte(address, value)
        }));

        self.cpu.connect_read_byte(read_byte_fn);
        self.cpu.connect_write_byte(write_byte_fn);
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
        self.bus.load_rom_at(&rom, address);
    }

    pub fn change_variant(&mut self, variant: String) {
        // Change the variant of the CPU
        self.cpu
            .change_variant(cpu::cpu::Variant::from_string(variant));
    }

    // Run the emulator for a certain number of cycles (optional)
    pub fn run(&mut self, speed_mhz: f64, num_cycles: Option<u64>, benchmark_mode: bool) {
        let mut cycles_left = num_cycles.unwrap_or(u64::MAX);

        // Change the variant to CMOS
        self.change_variant(String::from("CMOS"));

        self.cpu.reset();

        // Calculate the number of cycles to run per second
        let cycles_per_second = speed_mhz * 1_000_000.0;

        if (benchmark_mode) {
            // Start a timer
            let start = std::time::Instant::now();

            // Run the CPU for the specified number of cycles
            for _ in 0..cycles_left {
                self.cpu.clock();
            }

            // Stop the timer
            let end = std::time::Instant::now();

            // Calculate the time elapsed
            let time_elapsed = end.duration_since(start);

            // Calculate the number of cycles per second
            let cycles_per_second = num_cycles.unwrap() as f64 / start.elapsed().as_secs_f64();

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
        } else {
            // Run the CPU in this thread
            while cycles_left > 0 {
                println!("cycles_left: {}", cycles_left);
                self.cpu.clock();
                cycles_left -= 1;
                std::thread::sleep(std::time::Duration::from_secs_f64(1.0 / cycles_per_second));
            }
        }
        /*
        let cpu_thread = thread::spawn({
            let shared_cpu = shared_cpu.clone();
            move || {
                while cycles_left > 0 {
                    shared_cpu.lock().unwrap().clock();
                    cycles_left -= 1;
                    std::thread::sleep(std::time::Duration::from_secs_f64(1.0 / cycles_per_second));
                }
            }
        });
        */
    }

    pub fn benchmark(&mut self) {
        println!("Running benchmark...");
        self.load_rom_from_path("demos/blink.bin", 0xC000);
        self.change_variant(String::from("CMOS"));
        self.cpu.reset();

        let num_cycles = 100;

        println!("Running for {} cycles...", num_cycles);

        // Run the CPU
        self.run(1.0, Some(num_cycles), true);
    }
}
