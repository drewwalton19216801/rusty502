pub mod bus {
    use std::{cell::RefCell, sync::Arc};

    pub struct Bus {
        // Store 64K of RAM on the heap
        pub ram: Box<[u8; 64 * 1024]>,

        pub read_hooks: Vec<Option<Arc<RefCell<dyn FnMut(u16) -> u8>>>>,
        pub write_hooks: Vec<Option<Arc<RefCell<dyn FnMut(u16, u8)>>>>,
    }

    impl Bus {
        pub fn new() -> Self {
            Self {
                ram: Box::new([0; 64 * 1024]),
                read_hooks: vec![None; 64 * 1024],
                write_hooks: vec![None; 64 * 1024],
            }
        }

        pub fn load_rom_at(&mut self, rom: &[u8], address: u16) {
            for (i, &byte) in rom.iter().enumerate() {
                self.ram[address as usize + i] = byte;
            }
        }

        pub fn add_read_hook(&mut self, address: u16, hook: Arc<RefCell<dyn FnMut(u16) -> u8>>) {
            self.read_hooks[address as usize] = Some(hook);
        }

        pub fn add_write_hook(&mut self, address: u16, hook: Arc<RefCell<dyn FnMut(u16, u8)>>) {
            self.write_hooks[address as usize] = Some(hook);
        }

        pub fn read(&self, address: u16) -> u8 {
            // Check if there is a read hook for this address,
            // and if so, call it
            if let Some(hook) = &self.read_hooks[address as usize] {
                let mut hook = hook.borrow_mut();
                return hook(address);
            } else {
                // Otherwise, just read from RAM
                return self.ram[address as usize];
            }
        }

        pub fn write(&mut self, address: u16, data: u8) {
            // For debugging, we want to write to RAM anyway
            self.ram[address as usize] = data;

            // Check if there is a write hook for this address,
            // and if so, call it
            if let Some(hook) = &self.write_hooks[address as usize] {
                let mut hook = hook.borrow_mut();
                hook(address, data);
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
            assert_eq!(bus.read(0x8000), 0x01);
            assert_eq!(bus.read(0x8001), 0x02);
            assert_eq!(bus.read(0x8002), 0x03);
        }

        #[test]
        fn test_add_read_hook() {
            let mut bus = Bus::new();
            let hook = Arc::new(RefCell::new(|address| {
                if address == 0x1234 {
                    return 0x42;
                } else {
                    return 0x00;
                }
            }));
            bus.add_read_hook(0x1234, hook.clone());
            assert_eq!(bus.read(0x1234), 0x42);
            assert_eq!(bus.read(0x5678), 0x00);
        }

        #[test]
        fn test_add_write_hook() {
            let mut bus = Bus::new();
            let hook = Arc::new(RefCell::new(|address, data| {
                if address == 0x1234 {
                    assert_eq!(data, 0x42);
                } else {
                    assert_eq!(data, 0x00);
                }
            }));
            bus.add_write_hook(0x1234, hook.clone());
            bus.write(0x1234, 0x42);
            bus.write(0x5678, 0x00);
        }

        #[test]
        fn test_read_write() {
            let mut bus = Bus::new();
            bus.write(0x1234, 0x42);
            assert_eq!(bus.read(0x1234), 0x42);
        }
    }
}
