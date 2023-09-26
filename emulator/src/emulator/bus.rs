/**
 * This represents the system bus of the emulator.
 *
 * All of the components of the system are connected to this bus,
 * and they use a hook system (implemented here) to interact with the bus.
 */
use std::{collections::HashMap, sync::{Mutex, Arc}};

#[derive(Clone)]
pub struct Hook {
    pub read: Option<Arc<Mutex<dyn FnMut(u16) -> u8 + Send>>>,
    pub write: Option<Arc<Mutex<dyn FnMut(u16, u8) + Send>>>,
}

#[derive(Clone)]
pub struct Bus {
    // The memory of the system (includes ROM)
    pub memory: [u8; 0xFFFF + 1],

    // The hooks for the system, must be able to implement Copy
    hooks: HashMap<u16, Hook>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: [0; 0xFFFF + 1],
            hooks: HashMap::new(),
        }
    }

    pub fn add_hook(&mut self, address: u16, hook: Hook) {
        self.hooks.insert(address, hook);
    }

    pub fn add_hook_range(&mut self, start: u16, end: u16, hook: Hook) {
        for address in start..=end {
            self.add_hook(address, hook.clone());
        }
    }

    pub fn read_byte(&mut self, address: u16) -> u8 {
        // Check if there is a hook for this address
        if let Some(hook) = self.hooks.get_mut(&address) {
            // Check if the hook has a read function
            if let Some(read) = &mut hook.read {
                // Call the read function
                let data = read.lock().unwrap()(address);
                println!("Read byte from address 0x{:04X}: 0x{:02X}", address, data);
                return data;
            }
        }

        // Return the value at the address
        self.memory[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        // Check if there is a hook for this address
        if let Some(hook) = self.hooks.get_mut(&address) {
            // Check if the hook has a write function
            if let Some(write) = &mut hook.write {
                // Call the write function
                write.lock().unwrap()(address, value);
                return;
            }
        }

        // Write the value to the address
        self.memory[address as usize] = value;
    }

    pub fn load_rom_at(&mut self, rom: &[u8], address: u16) {
        // Load the rom into memory
        for (i, byte) in rom.iter().enumerate() {
            self.memory[address as usize + i] = *byte;
        }
    }
}
