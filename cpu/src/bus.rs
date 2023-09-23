pub mod bus {
    use std::{cell::RefCell, rc::Rc};

    pub struct Bus {
        pub ram: [u8; 64 * 1024], // 64 KB of RAM

        pub read_hooks: Vec<Option<Rc<RefCell<dyn FnMut(u16) -> u8>>>>,
        pub write_hooks: Vec<Option<Rc<RefCell<dyn FnMut(u16, u8)>>>>,
    }

    impl Bus {
        pub fn new() -> Self {
            Self {
                ram: [0; 64 * 1024],

                read_hooks: vec![None; 64 * 1024],
                write_hooks: vec![None; 64 * 1024],
            }
        }

        pub fn load_rom_at(&mut self, rom: &[u8], address: u16) {
            for (i, &byte) in rom.iter().enumerate() {
                self.ram[address as usize + i] = byte;
            }
        }

        pub fn add_read_hook(&mut self, address: u16, hook: Rc<RefCell<dyn FnMut(u16) -> u8>>) {
            self.read_hooks[address as usize] = Some(hook);
        }

        pub fn add_write_hook(&mut self, address: u16, hook: Rc<RefCell<dyn FnMut(u16, u8)>>) {
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
}