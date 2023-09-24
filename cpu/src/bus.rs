pub mod bus {
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    pub struct Bus {
        // Store 64K of RAM on the heap
        pub ram: Box<[u8; 64 * 1024]>,

        pub hooks: Vec<Option<Hook>>,
    }

    #[derive(Clone)]
    pub struct Hook {
        pub read: Option<Arc<Mutex<dyn FnMut(u16) -> u8 + Send>>>,
        pub write: Option<Arc<Mutex<dyn FnMut(u16, u8) + Send>>>,
    }

    impl Bus {
        pub fn new() -> Self {
            Self {
                ram: Box::new([0; 64 * 1024]),
                hooks: vec![None; 64 * 1024],
            }
        }

        pub fn load_rom_at(&mut self, rom: &[u8], address: u16) {
            for (i, &byte) in rom.iter().enumerate() {
                self.ram[address as usize + i] = byte;
            }
        }

        pub fn add_hook(&mut self, address: u16, hook: Hook) {
            self.hooks[address as usize] = Some(hook);
        }

        pub fn add_hook_range(&mut self, start_address: u16, end_address: u16, hook: Hook) {
            for address in start_address..=end_address {
                self.hooks[address as usize] = Some(hook.clone());
            }
        }

        pub fn read_byte(&self, address: u16) -> u8 {
            if let Some(hook) = &self.hooks[address as usize] {
                if let Some(read_hook) = &hook.read {
                    read_hook.lock().unwrap()(address)
                } else {
                    self.ram[address as usize]
                }
            } else {
                self.ram[address as usize]
            }
        }

        pub fn write_byte(&mut self, address: u16, value: u8) {
            if let Some(hook) = &self.hooks[address as usize] {
                if let Some(write_hook) = &hook.write {
                    write_hook.lock().unwrap()(address, value)
                } else {
                    self.ram[address as usize] = value;
                }
            } else {
                self.ram[address as usize] = value;
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_load_rom_at() {
            let mut bus = Bus::new();
            let rom = [0x01, 0x02, 0x03];
            bus.load_rom_at(&rom, 0x8000);
            assert_eq!(bus.read_byte(0x8000), 0x01);
            assert_eq!(bus.read_byte(0x8001), 0x02);
            assert_eq!(bus.read_byte(0x8002), 0x03);
        }

        #[test]
        fn test_add_hook() {
            let mut bus = Bus::new();
            let hook = Arc::new(Mutex::new(|address| {
                if address == 0x1234 {
                    0x42
                } else {
                    0x00
                }
            }));
            bus.add_hook(0x1234, Hook {
                read: Some(hook.clone()),
                write: None,
            });
            assert_eq!(bus.read_byte(0x1234), 0x42);

            let hook = Arc::new(Mutex::new(|address| {
                if address == 0x1234 {
                    0x42
                } else {
                    0x00
                }
            }));
            bus.add_hook(0x1234, Hook {
                read: Some(hook.clone()),
                write: None,
            });
            assert_eq!(bus.read_byte(0x1234), 0x42);
        }

        #[test]
        fn test_read_write() {
            let mut bus = Bus::new();
            bus.write_byte(0x1234, 0x42);
            assert_eq!(bus.read_byte(0x1234), 0x42);
        }
    }
}
