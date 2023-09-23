pub mod registers {

    pub struct Registers {
        pub a: u8, // Accumulator
        pub x: u8, // X register
        pub y: u8, // Y register
        pub pc: u16, // Program counter
        pub sp: u8, // Stack pointer
        pub flags: u8, // Status flags
    }

    pub enum Flag {
        Carry = 0b00000001, // 1 << 0
        Zero = 0b00000010, // 1 << 1
        InterruptDisable = 0b00000100, // 1 << 2
        DecimalMode = 0b00001000, // 1 << 3
        Break = 0b00010000, // 1 << 4
        Unused = 0b00100000, // 1 << 5
        Overflow = 0b01000000, // 1 << 6
        Negative = 0b10000000, // 1 << 7
    }

    impl Registers {
        pub fn new() -> Self {
            Self {
                a: 0x00,
                x: 0x00,
                y: 0x00,
                pc: 0x0000,
                sp: 0x00,
                flags: 0x00,
            }
        }

        pub fn set_flag(&mut self, flag: Flag, value: bool) {
            if value {
                self.flags |= flag as u8;
            } else {
                self.flags &= !(flag as u8);
            }
        }

        pub fn get_flag(&self, flag: Flag) -> bool {
            (self.flags & (flag as u8)) > 0
        }

        pub fn dump_registers(&self) {
            // Dump registers to stdout, using NVUBDIZC format for flags
            println!("A: {:02X} X: {:02X} Y: {:02X} P: {:02X} SP: {:02X} PC: {:04X}",
                     self.a, self.x, self.y, self.flags, self.sp, self.pc);
            
            let mut flag_string = String::new();
            flag_string.push_str(if self.get_flag(Flag::Negative) { "N" } else { "n" });
            flag_string.push_str(if self.get_flag(Flag::Overflow) { "V" } else { "v" });
            flag_string.push_str(if self.get_flag(Flag::Unused) { "U" } else { "u" });
            flag_string.push_str(if self.get_flag(Flag::Break) { "B" } else { "b" });
            flag_string.push_str(if self.get_flag(Flag::DecimalMode) { "D" } else { "d" });
            flag_string.push_str(if self.get_flag(Flag::InterruptDisable) { "I" } else { "i" });
            flag_string.push_str(if self.get_flag(Flag::Zero) { "Z" } else { "z" });
            flag_string.push_str(if self.get_flag(Flag::Carry) { "C" } else { "c" });
            println!("Flags: {}", flag_string);
        }
    }
}