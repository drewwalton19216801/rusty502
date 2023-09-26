/**
 * This represents the system bus of the emulator.
 * 
 * All of the components of the system are connected to this bus,
 * and they use a hook system (implemented here) to interact with the bus.
 */

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Hook {
    // Hooks should be able to read and write address ranges
    pub read: Option<Arc<Mutex<dyn FnMut(u16) -> u8 + Send + Sync>>>,
    pub write: Option<Arc<Mutex<dyn FnMut(u16, u8) + Send + Sync>>>,
}

pub struct Bus {
    // The memory of the system (includes ROM)
    memory: Vec<u8>,

    // The hooks for the system
    hooks: HashMap<u16, Hook>,
}

impl Bus {
    pub fn new() -> Self {
        Self {
            memory: vec![0; 0xFFFF],
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
                return read.lock().unwrap()(address);
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