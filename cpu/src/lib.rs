mod registers;
mod bus;
mod addresses;
mod instructions;

pub mod cpu {
    use crate::{registers::{registers::Registers, self}, bus::bus::Bus, instructions};
    use crate::addresses::addresses;

    pub struct Cpu {
        pub variant: Variant, // CPU variant
        pub state: State, // CPU state
        pub registers: Registers, // Registers
        pub bus: Bus, // Main bus

        pub cycles: u8, // Number of cycles remaining for current instruction
        pub temp: u16, // Temporary storage for various operations
        pub addr_abs: u16, // Absolute address
        pub addr_rel: u16, // Relative address
        pub opcode: u8, // Current opcode
        pub fetched: u8, // Fetched data

        pub enable_illegal_opcodes: bool, // Enable illegal opcodes
    }

    pub enum Variant {
        NMOS, // Original 6502 (with ROR bug)
        CMOS, // Modified 65C02 (no ROR bug)
        NES, // Modified 2A03 (no decimal mode)
    }

    pub enum State {
        Stopped, // CPU is stopped
        Fetching, // CPU is fetching an instruction
        Executing, // CPU is executing an instruction
        Interrupt, // CPU is handling an interrupt
        IllegalOpcode, // CPU encountered an illegal opcode
    }

    impl Cpu {
        pub fn new() -> Self {
            Self {
                registers: Registers::new(),
                bus: Bus::new(),
                variant: Variant::CMOS,
                state: State::Stopped,

                cycles: 0,
                temp: 0,
                addr_abs: 0,
                addr_rel: 0,
                opcode: 0,
                fetched: 0,

                enable_illegal_opcodes: false,
            }
        }

        pub fn change_variant(&mut self, variant: Variant) {
            self.variant = variant;
        }

        pub fn set_illegal_opcodes(&mut self, enable: bool) {
            self.enable_illegal_opcodes = enable;
        }

        pub fn reset(&mut self) {
            // Reset registers to initial state
            self.registers.a = 0x00;
            self.registers.x = 0x00;
            self.registers.y = 0x00;
            self.registers.pc = self.read_word(addresses::RESET_VECTOR);
            self.registers.sp = 0xFD;
            // Set all flags to 0x00, except for the unused flag and the interrupt disable flag
            self.registers.flags = 0x24;
        }

        pub fn read(&self, address: u16) -> u8 {
            self.bus.read(address)
        }

        pub fn read_word(&self, address: u16) -> u16 {
            let lo = self.read(address) as u16;
            let hi = self.read(address + 1) as u16;
            return (hi << 8) | lo;
        }

        pub fn write(&mut self, address: u16, data: u8) {
            self.bus.write(address, data)
        }

        pub fn write_word(&mut self, address: u16, data: u16) {
            let lo = (data & 0x00FF) as u8;
            let hi = ((data & 0xFF00) >> 8) as u8;
            self.write(address, lo);
            self.write(address + 1, hi);
        }

        pub fn fetch(&mut self) -> u8 {
            // Set state to Fetching
            self.state = State::Fetching;
            // Fetch the next byte from memory
            let data = self.read(self.registers.pc);
            self.registers.pc += 1;
            return data;
        }

        pub fn push(&mut self, data: u8) {
            // Push data to the stack
            self.write(0x0100 + self.registers.sp as u16, data);
            self.registers.sp -= 1;
        }

        pub fn push_word(&mut self, data: u16) {
            // Push data to the stack
            self.push(((data & 0xFF00) >> 8) as u8);
            self.push((data & 0x00FF) as u8);
        }

        pub fn pop(&mut self) -> u8 {
            // Pop data from the stack
            self.registers.sp += 1;
            return self.read(0x0100 + self.registers.sp as u16);
        }

        pub fn pop_word(&mut self) -> u16 {
            // Pop data from the stack
            let lo = self.pop() as u16;
            let hi = self.pop() as u16;
            return (hi << 8) | lo;
        }

        pub fn clock(&mut self) {
            // If we have no cycles remaining, fetch the next opcode
            if self.cycles == 0 {
                self.opcode = self.fetch();

                // Get the number of cycles for this opcode
                //self.cycles = self.get_cycles(self.opcode);

                // Execute the opcode
                //self.execute_opcode(self.opcode);
            }

            // Decrement the number of cycles remaining
            self.cycles -= 1;
        }

        pub fn irq(&mut self) {
            // If interrupts are enabled, push the program counter and flags to the stack
            if self.registers.get_flag(registers::registers::Flag::InterruptDisable) == false {
                self.push_word(self.registers.pc);

                // Set the break flag to 0
                self.registers.set_flag(registers::registers::Flag::Break, false);

                // Push the status flags to the stack
                self.registers.set_flag(registers::registers::Flag::Unused, true);
                self.registers.set_flag(registers::registers::Flag::Break, true);
                self.registers.set_flag(registers::registers::Flag::InterruptDisable, true);
                self.push(self.registers.flags);
                self.registers.set_flag(registers::registers::Flag::InterruptDisable, false);

                // Set the program counter to the interrupt vector
                self.registers.pc = self.read_word(addresses::IRQ_VECTOR);

                // Set the state to Interrupt
                self.state = State::Interrupt;

                // Set the number of cycles remaining to 7
                self.cycles = 7;
            }
        }

        pub fn nmi(&mut self) {
            // Push the program counter and flags to the stack
            self.push_word(self.registers.pc);

            // Set the break flag to 0
            self.registers.set_flag(registers::registers::Flag::Break, false);

            // Push the status flags to the stack
            self.registers.set_flag(registers::registers::Flag::Unused, true);
            self.registers.set_flag(registers::registers::Flag::Break, true);
            self.registers.set_flag(registers::registers::Flag::InterruptDisable, true);
            self.push(self.registers.flags);
            self.registers.set_flag(registers::registers::Flag::InterruptDisable, false);

            // Set the program counter to the NMI vector
            self.registers.pc = self.read_word(addresses::NMI_VECTOR);

            // Set the state to Interrupt
            self.state = State::Interrupt;

            // Set the number of cycles remaining to 7
            self.cycles = 7;
        }

        pub fn print_instruction_list(&self) {
            instructions::instructions::print_instruction_list();
        }

        /**
         * CPU instructions
         */

        pub fn adc(&mut self) -> u8 {
            return 0;
        }
        pub fn and(&mut self) -> u8 {
            return 0;
        }
        pub fn asl(&mut self) -> u8 {
            return 0;
        }
        pub fn bcc(&mut self) -> u8 {
            return 0;
        }
        pub fn bcs(&mut self) -> u8 {
            return 0;
        }
        pub fn beq(&mut self) -> u8 {
            return 0;
        }
        pub fn bit(&mut self) -> u8 {
            return 0;
        }
        pub fn bmi(&mut self) -> u8 {
            return 0;
        }
        pub fn bne(&mut self) -> u8 {
            return 0;
        }
        pub fn bpl(&mut self) -> u8 {
            return 0;
        }
        pub fn brk(&mut self) -> u8 {
            return 0;
        }
        pub fn bvc(&mut self) -> u8 {
            return 0;
        }
        pub fn bvs(&mut self) -> u8 {
            return 0;
        }
        pub fn clc(&mut self) -> u8 {
            return 0;
        }
        pub fn cld(&mut self) -> u8 {
            return 0;
        }
        pub fn cli(&mut self) -> u8 {
            return 0;
        }
        pub fn clv(&mut self) -> u8 {
            return 0;
        }
        pub fn cmp(&mut self) -> u8 {
            return 0;
        }
        pub fn cpx(&mut self) -> u8 {
            return 0;
        }
        pub fn cpy(&mut self) -> u8 {
            return 0;
        }
        pub fn dec(&mut self) -> u8 {
            return 0;
        }
        pub fn dex(&mut self) -> u8 {
            return 0;
        }
        pub fn dey(&mut self) -> u8 {
            return 0;
        }
        pub fn eor(&mut self) -> u8 {
            return 0;
        }
        pub fn inc(&mut self) -> u8 {
            return 0;
        }
        pub fn inx(&mut self) -> u8 {
            return 0;
        }
        pub fn iny(&mut self) -> u8 {
            return 0;
        }
        pub fn jmp(&mut self) -> u8 {
            return 0;
        }
        pub fn jsr(&mut self) -> u8 {
            return 0;
        }
        pub fn lda(&mut self) -> u8 {
            return 0;
        }
        pub fn ldx(&mut self) -> u8 {
            return 0;
        }
        pub fn ldy(&mut self) -> u8 {
            return 0;
        }
        pub fn lsr(&mut self) -> u8 {
            return 0;
        }
        pub fn nop(&mut self) -> u8 {
            return 0;
        }
        pub fn ora(&mut self) -> u8 {
            return 0;
        }
        pub fn pha(&mut self) -> u8 {
            return 0;
        }
        pub fn php(&mut self) -> u8 {
            return 0;
        }
        pub fn pla(&mut self) -> u8 {
            return 0;
        }
        pub fn plp(&mut self) -> u8 {
            return 0;
        }
        pub fn rol(&mut self) -> u8 {
            return 0;
        }
        pub fn ror(&mut self) -> u8 {
            return 0;
        }
        pub fn rti(&mut self) -> u8 {
            return 0;
        }
        pub fn rts(&mut self) -> u8 {
            return 0;
        }
        pub fn sbc(&mut self) -> u8 {
            return 0;
        }
        pub fn sec(&mut self) -> u8 {
            return 0;
        }
        pub fn sed(&mut self) -> u8 {
            return 0;
        }
        pub fn sei(&mut self) -> u8 {
            return 0;
        }
        pub fn sta(&mut self) -> u8 {
            return 0;
        }
        pub fn stx(&mut self) -> u8 {
            return 0;
        }
        pub fn sty(&mut self) -> u8 {
            return 0;
        }
        pub fn tax(&mut self) -> u8 {
            return 0;
        }
        pub fn tay(&mut self) -> u8 {
            return 0;
        }
        pub fn tsx(&mut self) -> u8 {
            return 0;
        }
        pub fn txa(&mut self) -> u8 {
            return 0;
        }
        pub fn txs(&mut self) -> u8 {
            return 0;
        }
        pub fn tya(&mut self) -> u8 {
            return 0;
        }

        /**
         * Illegal instructions
         */
        pub fn ahx(&mut self) -> u8 {
            return 0;
        }
        pub fn alr(&mut self) -> u8 {
            return 0;
        }
        pub fn anc(&mut self) -> u8 {
            return 0;
        }
        pub fn arr(&mut self) -> u8 {
            return 0;
        }
        pub fn axs(&mut self) -> u8 {
            return 0;
        }
        pub fn dcp(&mut self) -> u8 {
            return 0;
        }
        pub fn isc(&mut self) -> u8 {
            return 0;
        }
        pub fn kil(&mut self) -> u8 {
            return 0;
        }
        pub fn las(&mut self) -> u8 {
            return 0;
        }
        pub fn lax(&mut self) -> u8 {
            return 0;
        }
        pub fn rla(&mut self) -> u8 {
            return 0;
        }
        pub fn rra(&mut self) -> u8 {
            return 0;
        }
        pub fn sax(&mut self) -> u8 {
            return 0;
        }
        pub fn shx(&mut self) -> u8 {
            return 0;
        }
        pub fn shy(&mut self) -> u8 {
            return 0;
        }
        pub fn slo(&mut self) -> u8 {
            return 0;
        }
        pub fn sre(&mut self) -> u8 {
            return 0;
        }
        pub fn tas(&mut self) -> u8 {
            return 0;
        }
        pub fn xaa(&mut self) -> u8 {
            return 0;
        }
    }
}
