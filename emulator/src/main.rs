use std::{cell::RefCell, rc::Rc};

use cpu::{self, cpu::Cpu};

fn main() {
    let mut cpu = Cpu::new();
    // Register our test read hook
    cpu.bus.add_read_hook(0x8000, Rc::new(RefCell::new(test_read_hook)));
    // Register our test write hook
    cpu.bus.add_write_hook(0x8000, Rc::new(RefCell::new(test_write_hook)));

    // Change the variant to NES
    cpu.change_variant(cpu::cpu::Variant::NES);

    cpu.reset();
    let value = cpu.bus.read(0x8000);
    cpu.bus.write(0x8001, value+1);
    let data = cpu.bus.read(0x8001);
    println!("Data: {:02X}", data);
    cpu.registers.dump_registers();
}

fn test_read_hook(address: u16) -> u8 {
    let data = 0x69;
    println!("Read hook called for address {:04X}", address);
    return data;
}

fn test_write_hook(address: u16, data: u8) {
    println!("Write hook called for address {:04X} with data {:02X}", address, data);
}