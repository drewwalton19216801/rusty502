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
            Self {
                cpu: Cpu::new(),
            }
        }

        pub fn load_file_from_path(&mut self, path: &str) {
            // Load the file into a vector
            let rom = std::fs::read(path).unwrap();

            // Load the file into memory
            self.cpu.bus.load_rom_at(&rom, 0xC000);
        }

        pub fn run(&mut self) {
            // Register our test write hook
            self.cpu.bus
                .add_write_hook(0x6000, Rc::new(RefCell::new(Self::blink_led)));
            self.cpu.bus
                .add_write_hook(0x6002, Rc::new(RefCell::new(Self::blink_led)));

            // Change the variant to CMOS
            self.cpu.change_variant(cpu::cpu::Variant::CMOS);

            self.cpu.reset();

            // Run the CPU for 100 cycles
            for _ in 0..100 {
                self.cpu.clock();
            }
        }

        pub fn blink_led(address: u16, data: u8) {
            // If the address is 0x6000 or 0x6002, we want to print the data
            if address == 0x6000 || address == 0x6002 {
                println!("Write to address {:04X}: {:02X}", address, data);
            }

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
                print!("LED strip: ");
                for i in 0..8 {
                    print!("{}", if LED_STRIP[i] { "X" } else { " " });
                }
                println!();
            }
        }
    }
}
