pub mod instructions {
    use crate::cpu::Cpu;

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum AddressingMode {
        Absolute,
        AbsoluteX,
        AbsoluteY,
        Immediate,
        Implied,
        IndexedIndirect,
        Indirect,
        IndirectIndexed,
        Relative,
        ZeroPage,
        ZeroPageX,
        ZeroPageY,
    }

    pub struct Instruction {
        pub opcode: u8,
        pub name: &'static str,
        pub mode: AddressingMode,
        pub cycles: u8,
        // Function pointer to the instruction's implementation in the Cpu struct
        pub function: fn(&mut Cpu) -> u8,
    }

    pub const INSTRUCTION_LIST: [Instruction; 256] = [
        Instruction { opcode: 0x00, name: "BRK", mode: AddressingMode::Immediate, cycles: 7, function: crate::cpu::Cpu::brk },
        Instruction { opcode: 0x01, name: "ORA", mode: AddressingMode::IndexedIndirect, cycles: 6, function: crate::cpu::Cpu::ora },
        Instruction { opcode: 0x02, name: "KIL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::kil },
        Instruction { opcode: 0x03, name: "SLO", mode: AddressingMode::IndexedIndirect, cycles: 8, function: crate::cpu::Cpu::slo },
        Instruction { opcode: 0x04, name: "NOP", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x05, name: "ORA", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::ora },
        Instruction { opcode: 0x06, name: "ASL", mode: AddressingMode::ZeroPage, cycles: 5, function: crate::cpu::Cpu::asl },
        Instruction { opcode: 0x07, name: "SLO", mode: AddressingMode::ZeroPage, cycles: 5, function: crate::cpu::Cpu::slo },
        Instruction { opcode: 0x08, name: "PHP", mode: AddressingMode::Implied, cycles: 3, function: crate::cpu::Cpu::php },
        Instruction { opcode: 0x09, name: "ORA", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::ora },
        Instruction { opcode: 0x0A, name: "ASL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::asl },
        Instruction { opcode: 0x0B, name: "ANC", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::anc },
        Instruction { opcode: 0x0C, name: "NOP", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x0D, name: "ORA", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::ora },
        Instruction { opcode: 0x0E, name: "ASL", mode: AddressingMode::Absolute, cycles: 6, function: crate::cpu::Cpu::asl },
        Instruction { opcode: 0x0F, name: "SLO", mode: AddressingMode::Absolute, cycles: 6, function: crate::cpu::Cpu::slo },
        Instruction { opcode: 0x10, name: "BPL", mode: AddressingMode::Relative, cycles: 2, function: crate::cpu::Cpu::bpl },
        Instruction { opcode: 0x11, name: "ORA", mode: AddressingMode::IndirectIndexed, cycles: 5, function: crate::cpu::Cpu::ora },
        Instruction { opcode: 0x12, name: "KIL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::kil },
        Instruction { opcode: 0x13, name: "SLO", mode: AddressingMode::IndirectIndexed, cycles: 8, function: crate::cpu::Cpu::slo },
        Instruction { opcode: 0x14, name: "NOP", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x15, name: "ORA", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::ora },
        Instruction { opcode: 0x16, name: "ASL", mode: AddressingMode::ZeroPageX, cycles: 6, function: crate::cpu::Cpu::asl },
        Instruction { opcode: 0x17, name: "SLO", mode: AddressingMode::ZeroPageX, cycles: 6, function: crate::cpu::Cpu::slo },
        Instruction { opcode: 0x18, name: "CLC", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::clc },
        Instruction { opcode: 0x19, name: "ORA", mode: AddressingMode::AbsoluteY, cycles: 4, function: crate::cpu::Cpu::ora },
        Instruction { opcode: 0x1A, name: "NOP", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x1B, name: "SLO", mode: AddressingMode::AbsoluteY, cycles: 7, function: crate::cpu::Cpu::slo },
        Instruction { opcode: 0x1C, name: "NOP", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x1D, name: "ORA", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::ora },
        Instruction { opcode: 0x1E, name: "ASL", mode: AddressingMode::AbsoluteX, cycles: 7, function: crate::cpu::Cpu::asl },
        Instruction { opcode: 0x1F, name: "SLO", mode: AddressingMode::AbsoluteX, cycles: 7, function: crate::cpu::Cpu::slo },
        Instruction { opcode: 0x20, name: "JSR", mode: AddressingMode::Absolute, cycles: 6, function: crate::cpu::Cpu::jsr },
        Instruction { opcode: 0x21, name: "AND", mode: AddressingMode::IndexedIndirect, cycles: 6, function: crate::cpu::Cpu::and },
        Instruction { opcode: 0x22, name: "KIL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::kil },
        Instruction { opcode: 0x23, name: "RLA", mode: AddressingMode::IndexedIndirect, cycles: 8, function: crate::cpu::Cpu::rla },
        Instruction { opcode: 0x24, name: "BIT", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::bit },
        Instruction { opcode: 0x25, name: "AND", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::and },
        Instruction { opcode: 0x26, name: "ROL", mode: AddressingMode::ZeroPage, cycles: 5, function: crate::cpu::Cpu::rol },
        Instruction { opcode: 0x27, name: "RLA", mode: AddressingMode::ZeroPage, cycles: 5, function: crate::cpu::Cpu::rla },
        Instruction { opcode: 0x28, name: "PLP", mode: AddressingMode::Implied, cycles: 4, function: crate::cpu::Cpu::plp },
        Instruction { opcode: 0x29, name: "AND", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::and },
        Instruction { opcode: 0x2A, name: "ROL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::rol },
        Instruction { opcode: 0x2B, name: "ANC", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::anc },
        Instruction { opcode: 0x2C, name: "BIT", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::bit },
        Instruction { opcode: 0x2D, name: "AND", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::and },
        Instruction { opcode: 0x2E, name: "ROL", mode: AddressingMode::Absolute, cycles: 6, function: crate::cpu::Cpu::rol },
        Instruction { opcode: 0x2F, name: "RLA", mode: AddressingMode::Absolute, cycles: 6, function: crate::cpu::Cpu::rla },
        Instruction { opcode: 0x30, name: "BMI", mode: AddressingMode::Relative, cycles: 2, function: crate::cpu::Cpu::bmi },
        Instruction { opcode: 0x31, name: "AND", mode: AddressingMode::IndirectIndexed, cycles: 5, function: crate::cpu::Cpu::and },
        Instruction { opcode: 0x32, name: "KIL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::kil },
        Instruction { opcode: 0x33, name: "RLA", mode: AddressingMode::IndirectIndexed, cycles: 8, function: crate::cpu::Cpu::rla },
        Instruction { opcode: 0x34, name: "NOP", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x35, name: "AND", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::and },
        Instruction { opcode: 0x36, name: "ROL", mode: AddressingMode::ZeroPageX, cycles: 6, function: crate::cpu::Cpu::rol },
        Instruction { opcode: 0x37, name: "RLA", mode: AddressingMode::ZeroPageX, cycles: 6, function: crate::cpu::Cpu::rla },
        Instruction { opcode: 0x38, name: "SEC", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::sec },
        Instruction { opcode: 0x39, name: "AND", mode: AddressingMode::AbsoluteY, cycles: 4, function: crate::cpu::Cpu::and },
        Instruction { opcode: 0x3A, name: "NOP", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x3B, name: "RLA", mode: AddressingMode::AbsoluteY, cycles: 7, function: crate::cpu::Cpu::rla },
        Instruction { opcode: 0x3C, name: "NOP", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x3D, name: "AND", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::and },
        Instruction { opcode: 0x3E, name: "ROL", mode: AddressingMode::AbsoluteX, cycles: 7, function: crate::cpu::Cpu::rol },
        Instruction { opcode: 0x3F, name: "RLA", mode: AddressingMode::AbsoluteX, cycles: 7, function: crate::cpu::Cpu::rla },
        Instruction { opcode: 0x40, name: "RTI", mode: AddressingMode::Implied, cycles: 6, function: crate::cpu::Cpu::rti },
        Instruction { opcode: 0x41, name: "EOR", mode: AddressingMode::IndexedIndirect, cycles: 6, function: crate::cpu::Cpu::eor },
        Instruction { opcode: 0x42, name: "KIL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::kil },
        Instruction { opcode: 0x43, name: "SRE", mode: AddressingMode::IndexedIndirect, cycles: 8, function: crate::cpu::Cpu::sre },
        Instruction { opcode: 0x44, name: "NOP", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x45, name: "EOR", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::eor },
        Instruction { opcode: 0x46, name: "LSR", mode: AddressingMode::ZeroPage, cycles: 5, function: crate::cpu::Cpu::lsr },
        Instruction { opcode: 0x47, name: "SRE", mode: AddressingMode::ZeroPage, cycles: 5, function: crate::cpu::Cpu::sre },
        Instruction { opcode: 0x48, name: "PHA", mode: AddressingMode::Implied, cycles: 3, function: crate::cpu::Cpu::pha },
        Instruction { opcode: 0x49, name: "EOR", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::eor },
        Instruction { opcode: 0x4A, name: "LSR", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::lsr },
        Instruction { opcode: 0x4B, name: "ALR", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::alr },
        Instruction { opcode: 0x4C, name: "JMP", mode: AddressingMode::Absolute, cycles: 3, function: crate::cpu::Cpu::jmp },
        Instruction { opcode: 0x4D, name: "EOR", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::eor },
        Instruction { opcode: 0x4E, name: "LSR", mode: AddressingMode::Absolute, cycles: 6, function: crate::cpu::Cpu::lsr },
        Instruction { opcode: 0x4F, name: "SRE", mode: AddressingMode::Absolute, cycles: 6, function: crate::cpu::Cpu::sre },
        Instruction { opcode: 0x50, name: "BVC", mode: AddressingMode::Relative, cycles: 2, function: crate::cpu::Cpu::bvc },
        Instruction { opcode: 0x51, name: "EOR", mode: AddressingMode::IndirectIndexed, cycles: 5, function: crate::cpu::Cpu::eor },
        Instruction { opcode: 0x52, name: "KIL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::kil },
        Instruction { opcode: 0x53, name: "SRE", mode: AddressingMode::IndirectIndexed, cycles: 8, function: crate::cpu::Cpu::sre },
        Instruction { opcode: 0x54, name: "NOP", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x55, name: "EOR", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::eor },
        Instruction { opcode: 0x56, name: "LSR", mode: AddressingMode::ZeroPageX, cycles: 6, function: crate::cpu::Cpu::lsr },
        Instruction { opcode: 0x57, name: "SRE", mode: AddressingMode::ZeroPageX, cycles: 6, function: crate::cpu::Cpu::sre },
        Instruction { opcode: 0x58, name: "CLI", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::cli },
        Instruction { opcode: 0x59, name: "EOR", mode: AddressingMode::AbsoluteY, cycles: 4, function: crate::cpu::Cpu::eor },
        Instruction { opcode: 0x5A, name: "NOP", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x5B, name: "SRE", mode: AddressingMode::AbsoluteY, cycles: 7, function: crate::cpu::Cpu::sre },
        Instruction { opcode: 0x5C, name: "NOP", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x5D, name: "EOR", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::eor },
        Instruction { opcode: 0x5E, name: "LSR", mode: AddressingMode::AbsoluteX, cycles: 7, function: crate::cpu::Cpu::lsr },
        Instruction { opcode: 0x5F, name: "SRE", mode: AddressingMode::AbsoluteX, cycles: 7, function: crate::cpu::Cpu::sre },
        Instruction { opcode: 0x60, name: "RTS", mode: AddressingMode::Implied, cycles: 6, function: crate::cpu::Cpu::rts },
        Instruction { opcode: 0x61, name: "ADC", mode: AddressingMode::IndexedIndirect, cycles: 6, function: crate::cpu::Cpu::adc },
        Instruction { opcode: 0x62, name: "KIL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::kil },
        Instruction { opcode: 0x63, name: "RRA", mode: AddressingMode::IndexedIndirect, cycles: 8, function: crate::cpu::Cpu::rra },
        Instruction { opcode: 0x64, name: "NOP", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x65, name: "ADC", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::adc },
        Instruction { opcode: 0x66, name: "ROR", mode: AddressingMode::ZeroPage, cycles: 5, function: crate::cpu::Cpu::ror },
        Instruction { opcode: 0x67, name: "RRA", mode: AddressingMode::ZeroPage, cycles: 5, function: crate::cpu::Cpu::rra },
        Instruction { opcode: 0x68, name: "PLA", mode: AddressingMode::Implied, cycles: 4, function: crate::cpu::Cpu::pla },
        Instruction { opcode: 0x69, name: "ADC", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::adc },
        Instruction { opcode: 0x6A, name: "RORA", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::ror_a },
        Instruction { opcode: 0x6B, name: "ARR", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::arr },
        Instruction { opcode: 0x6C, name: "JMP", mode: AddressingMode::Indirect, cycles: 5, function: crate::cpu::Cpu::jmp },
        Instruction { opcode: 0x6D, name: "ADC", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::adc },
        Instruction { opcode: 0x6E, name: "ROR", mode: AddressingMode::Absolute, cycles: 6, function: crate::cpu::Cpu::ror },
        Instruction { opcode: 0x6F, name: "RRA", mode: AddressingMode::Absolute, cycles: 6, function: crate::cpu::Cpu::rra },
        Instruction { opcode: 0x70, name: "BVS", mode: AddressingMode::Relative, cycles: 2, function: crate::cpu::Cpu::bvs },
        Instruction { opcode: 0x71, name: "ADC", mode: AddressingMode::IndirectIndexed, cycles: 5, function: crate::cpu::Cpu::adc },
        Instruction { opcode: 0x72, name: "KIL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::kil },
        Instruction { opcode: 0x73, name: "RRA", mode: AddressingMode::IndirectIndexed, cycles: 8, function: crate::cpu::Cpu::rra },
        Instruction { opcode: 0x74, name: "NOP", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x75, name: "ADC", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::adc },
        Instruction { opcode: 0x76, name: "ROR", mode: AddressingMode::ZeroPageX, cycles: 6, function: crate::cpu::Cpu::ror },
        Instruction { opcode: 0x77, name: "RRA", mode: AddressingMode::ZeroPageX, cycles: 6, function: crate::cpu::Cpu::rra },
        Instruction { opcode: 0x78, name: "SEI", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::sei },
        Instruction { opcode: 0x79, name: "ADC", mode: AddressingMode::AbsoluteY, cycles: 4, function: crate::cpu::Cpu::adc },
        Instruction { opcode: 0x7A, name: "NOP", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x7B, name: "RRA", mode: AddressingMode::AbsoluteY, cycles: 7, function: crate::cpu::Cpu::rra },
        Instruction { opcode: 0x7C, name: "NOP", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x7D, name: "ADC", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::adc },
        Instruction { opcode: 0x7E, name: "ROR", mode: AddressingMode::AbsoluteX, cycles: 7, function: crate::cpu::Cpu::ror },
        Instruction { opcode: 0x7F, name: "RRA", mode: AddressingMode::AbsoluteX, cycles: 7, function: crate::cpu::Cpu::rra },
        Instruction { opcode: 0x80, name: "NOP", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x81, name: "STA", mode: AddressingMode::IndexedIndirect, cycles: 6, function: crate::cpu::Cpu::sta },
        Instruction { opcode: 0x82, name: "NOP", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x83, name: "SAX", mode: AddressingMode::IndexedIndirect, cycles: 6, function: crate::cpu::Cpu::sax },
        Instruction { opcode: 0x84, name: "STY", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::sty },
        Instruction { opcode: 0x85, name: "STA", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::sta },
        Instruction { opcode: 0x86, name: "STX", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::stx },
        Instruction { opcode: 0x87, name: "SAX", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::sax },
        Instruction { opcode: 0x88, name: "DEY", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::dey },
        Instruction { opcode: 0x89, name: "NOP", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0x8A, name: "TXA", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::txa },
        Instruction { opcode: 0x8B, name: "XAA", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::xaa },
        Instruction { opcode: 0x8C, name: "STY", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::sty },
        Instruction { opcode: 0x8D, name: "STA", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::sta },
        Instruction { opcode: 0x8E, name: "STX", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::stx },
        Instruction { opcode: 0x8F, name: "SAX", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::sax },
        Instruction { opcode: 0x90, name: "BCC", mode: AddressingMode::Relative, cycles: 2, function: crate::cpu::Cpu::bcc },
        Instruction { opcode: 0x91, name: "STA", mode: AddressingMode::IndirectIndexed, cycles: 6, function: crate::cpu::Cpu::sta },
        Instruction { opcode: 0x92, name: "KIL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::kil },
        Instruction { opcode: 0x93, name: "AHX", mode: AddressingMode::IndirectIndexed, cycles: 6, function: crate::cpu::Cpu::ahx },
        Instruction { opcode: 0x94, name: "STY", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::sty },
        Instruction { opcode: 0x95, name: "STA", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::sta },
        Instruction { opcode: 0x96, name: "STX", mode: AddressingMode::ZeroPageY, cycles: 4, function: crate::cpu::Cpu::stx },
        Instruction { opcode: 0x97, name: "SAX", mode: AddressingMode::ZeroPageY, cycles: 4, function: crate::cpu::Cpu::sax },
        Instruction { opcode: 0x98, name: "TYA", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::tya },
        Instruction { opcode: 0x99, name: "STA", mode: AddressingMode::AbsoluteY, cycles: 5, function: crate::cpu::Cpu::sta },
        Instruction { opcode: 0x9A, name: "TXS", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::txs },
        Instruction { opcode: 0x9B, name: "TAS", mode: AddressingMode::AbsoluteY, cycles: 5, function: crate::cpu::Cpu::tas },
        Instruction { opcode: 0x9C, name: "SHY", mode: AddressingMode::AbsoluteX, cycles: 5, function: crate::cpu::Cpu::shy },
        Instruction { opcode: 0x9D, name: "STA", mode: AddressingMode::AbsoluteX, cycles: 5, function: crate::cpu::Cpu::sta },
        Instruction { opcode: 0x9E, name: "SHX", mode: AddressingMode::AbsoluteY, cycles: 5, function: crate::cpu::Cpu::shx },
        Instruction { opcode: 0x9F, name: "AHX", mode: AddressingMode::AbsoluteY, cycles: 5, function: crate::cpu::Cpu::ahx },
        Instruction { opcode: 0xA0, name: "LDY", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::ldy },
        Instruction { opcode: 0xA1, name: "LDA", mode: AddressingMode::IndexedIndirect, cycles: 6, function: crate::cpu::Cpu::lda },
        Instruction { opcode: 0xA2, name: "LDX", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::ldx },
        Instruction { opcode: 0xA3, name: "LAX", mode: AddressingMode::IndexedIndirect, cycles: 6, function: crate::cpu::Cpu::lax },
        Instruction { opcode: 0xA4, name: "LDY", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::ldy },
        Instruction { opcode: 0xA5, name: "LDA", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::lda },
        Instruction { opcode: 0xA6, name: "LDX", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::ldx },
        Instruction { opcode: 0xA7, name: "LAX", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::lax },
        Instruction { opcode: 0xA8, name: "TAY", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::tay },
        Instruction { opcode: 0xA9, name: "LDA", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::lda },
        Instruction { opcode: 0xAA, name: "TAX", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::tax },
        Instruction { opcode: 0xAB, name: "LAX", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::lax },
        Instruction { opcode: 0xAC, name: "LDY", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::ldy },
        Instruction { opcode: 0xAD, name: "LDA", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::lda },
        Instruction { opcode: 0xAE, name: "LDX", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::ldx },
        Instruction { opcode: 0xAF, name: "LAX", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::lax },
        Instruction { opcode: 0xB0, name: "BCS", mode: AddressingMode::Relative, cycles: 2, function: crate::cpu::Cpu::bcs },
        Instruction { opcode: 0xB1, name: "LDA", mode: AddressingMode::IndirectIndexed, cycles: 5, function: crate::cpu::Cpu::lda },
        Instruction { opcode: 0xB2, name: "KIL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::kil },
        Instruction { opcode: 0xB3, name: "LAX", mode: AddressingMode::IndirectIndexed, cycles: 5, function: crate::cpu::Cpu::lax },
        Instruction { opcode: 0xB4, name: "LDY", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::ldy },
        Instruction { opcode: 0xB5, name: "LDA", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::lda },
        Instruction { opcode: 0xB6, name: "LDX", mode: AddressingMode::ZeroPageY, cycles: 4, function: crate::cpu::Cpu::ldx },
        Instruction { opcode: 0xB7, name: "LAX", mode: AddressingMode::ZeroPageY, cycles: 4, function: crate::cpu::Cpu::lax },
        Instruction { opcode: 0xB8, name: "CLV", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::clv },
        Instruction { opcode: 0xB9, name: "LDA", mode: AddressingMode::AbsoluteY, cycles: 4, function: crate::cpu::Cpu::lda },
        Instruction { opcode: 0xBA, name: "TSX", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::tsx },
        Instruction { opcode: 0xBB, name: "LAS", mode: AddressingMode::AbsoluteY, cycles: 4, function: crate::cpu::Cpu::las },
        Instruction { opcode: 0xBC, name: "LDY", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::ldy },
        Instruction { opcode: 0xBD, name: "LDA", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::lda },
        Instruction { opcode: 0xBE, name: "LDX", mode: AddressingMode::AbsoluteY, cycles: 4, function: crate::cpu::Cpu::ldx },
        Instruction { opcode: 0xBF, name: "LAX", mode: AddressingMode::AbsoluteY, cycles: 4, function: crate::cpu::Cpu::lax },
        Instruction { opcode: 0xC0, name: "CPY", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::cpy },
        Instruction { opcode: 0xC1, name: "CMP", mode: AddressingMode::IndexedIndirect, cycles: 6, function: crate::cpu::Cpu::cmp },
        Instruction { opcode: 0xC2, name: "NOP", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0xC3, name: "DCP", mode: AddressingMode::IndexedIndirect, cycles: 8, function: crate::cpu::Cpu::dcp },
        Instruction { opcode: 0xC4, name: "CPY", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::cpy },
        Instruction { opcode: 0xC5, name: "CMP", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::cmp },
        Instruction { opcode: 0xC6, name: "DEC", mode: AddressingMode::ZeroPage, cycles: 5, function: crate::cpu::Cpu::dec },
        Instruction { opcode: 0xC7, name: "DCP", mode: AddressingMode::ZeroPage, cycles: 5, function: crate::cpu::Cpu::dcp },
        Instruction { opcode: 0xC8, name: "INY", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::iny },
        Instruction { opcode: 0xC9, name: "CMP", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::cmp },
        Instruction { opcode: 0xCA, name: "DEX", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::dex },
        Instruction { opcode: 0xCB, name: "AXS", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::axs },
        Instruction { opcode: 0xCC, name: "CPY", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::cpy },
        Instruction { opcode: 0xCD, name: "CMP", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::cmp },
        Instruction { opcode: 0xCE, name: "DEC", mode: AddressingMode::Absolute, cycles: 6, function: crate::cpu::Cpu::dec },
        Instruction { opcode: 0xCF, name: "DCP", mode: AddressingMode::Absolute, cycles: 6, function: crate::cpu::Cpu::dcp },
        Instruction { opcode: 0xD0, name: "BNE", mode: AddressingMode::Relative, cycles: 2, function: crate::cpu::Cpu::bne },
        Instruction { opcode: 0xD1, name: "CMP", mode: AddressingMode::IndirectIndexed, cycles: 5, function: crate::cpu::Cpu::cmp },
        Instruction { opcode: 0xD2, name: "KIL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::kil },
        Instruction { opcode: 0xD3, name: "DCP", mode: AddressingMode::IndirectIndexed, cycles: 8, function: crate::cpu::Cpu::dcp },
        Instruction { opcode: 0xD4, name: "NOP", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0xD5, name: "CMP", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::cmp },
        Instruction { opcode: 0xD6, name: "DEC", mode: AddressingMode::ZeroPageX, cycles: 6, function: crate::cpu::Cpu::dec },
        Instruction { opcode: 0xD7, name: "DCP", mode: AddressingMode::ZeroPageX, cycles: 6, function: crate::cpu::Cpu::dcp },
        Instruction { opcode: 0xD8, name: "CLD", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::cld },
        Instruction { opcode: 0xD9, name: "CMP", mode: AddressingMode::AbsoluteY, cycles: 4, function: crate::cpu::Cpu::cmp },
        Instruction { opcode: 0xDA, name: "NOP", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0xDB, name: "DCP", mode: AddressingMode::AbsoluteY, cycles: 7, function: crate::cpu::Cpu::dcp },
        Instruction { opcode: 0xDC, name: "NOP", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0xDD, name: "CMP", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::cmp },
        Instruction { opcode: 0xDE, name: "DEC", mode: AddressingMode::AbsoluteX, cycles: 7, function: crate::cpu::Cpu::dec },
        Instruction { opcode: 0xDF, name: "DCP", mode: AddressingMode::AbsoluteX, cycles: 7, function: crate::cpu::Cpu::dcp },
        Instruction { opcode: 0xE0, name: "CPX", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::cpx },
        Instruction { opcode: 0xE1, name: "SBC", mode: AddressingMode::IndexedIndirect, cycles: 6, function: crate::cpu::Cpu::sbc },
        Instruction { opcode: 0xE2, name: "NOP", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0xE3, name: "ISC", mode: AddressingMode::IndexedIndirect, cycles: 8, function: crate::cpu::Cpu::isc },
        Instruction { opcode: 0xE4, name: "CPX", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::cpx },
        Instruction { opcode: 0xE5, name: "SBC", mode: AddressingMode::ZeroPage, cycles: 3, function: crate::cpu::Cpu::sbc },
        Instruction { opcode: 0xE6, name: "INC", mode: AddressingMode::ZeroPage, cycles: 5, function: crate::cpu::Cpu::inc },
        Instruction { opcode: 0xE7, name: "ISC", mode: AddressingMode::ZeroPage, cycles: 5, function: crate::cpu::Cpu::isc },
        Instruction { opcode: 0xE8, name: "INX", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::inx },
        Instruction { opcode: 0xE9, name: "SBC", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::sbc },
        Instruction { opcode: 0xEA, name: "NOP", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0xEB, name: "SBC", mode: AddressingMode::Immediate, cycles: 2, function: crate::cpu::Cpu::sbc },
        Instruction { opcode: 0xEC, name: "CPX", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::cpx },
        Instruction { opcode: 0xED, name: "SBC", mode: AddressingMode::Absolute, cycles: 4, function: crate::cpu::Cpu::sbc },
        Instruction { opcode: 0xEE, name: "INC", mode: AddressingMode::Absolute, cycles: 6, function: crate::cpu::Cpu::inc },
        Instruction { opcode: 0xEF, name: "ISC", mode: AddressingMode::Absolute, cycles: 6, function: crate::cpu::Cpu::isc },
        Instruction { opcode: 0xF0, name: "BEQ", mode: AddressingMode::Relative, cycles: 2, function: crate::cpu::Cpu::beq },
        Instruction { opcode: 0xF1, name: "SBC", mode: AddressingMode::IndirectIndexed, cycles: 5, function: crate::cpu::Cpu::sbc },
        Instruction { opcode: 0xF2, name: "KIL", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::kil },
        Instruction { opcode: 0xF3, name: "ISC", mode: AddressingMode::IndirectIndexed, cycles: 8, function: crate::cpu::Cpu::isc },
        Instruction { opcode: 0xF4, name: "NOP", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0xF5, name: "SBC", mode: AddressingMode::ZeroPageX, cycles: 4, function: crate::cpu::Cpu::sbc },
        Instruction { opcode: 0xF6, name: "INC", mode: AddressingMode::ZeroPageX, cycles: 6, function: crate::cpu::Cpu::inc },
        Instruction { opcode: 0xF7, name: "ISC", mode: AddressingMode::ZeroPageX, cycles: 6, function: crate::cpu::Cpu::isc },
        Instruction { opcode: 0xF8, name: "SED", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::sed },
        Instruction { opcode: 0xF9, name: "SBC", mode: AddressingMode::AbsoluteY, cycles: 4, function: crate::cpu::Cpu::sbc },
        Instruction { opcode: 0xFA, name: "NOP", mode: AddressingMode::Implied, cycles: 2, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0xFB, name: "ISC", mode: AddressingMode::AbsoluteY, cycles: 7, function: crate::cpu::Cpu::isc },
        Instruction { opcode: 0xFC, name: "NOP", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::nop },
        Instruction { opcode: 0xFD, name: "SBC", mode: AddressingMode::AbsoluteX, cycles: 4, function: crate::cpu::Cpu::sbc },
        Instruction { opcode: 0xFE, name: "INC", mode: AddressingMode::AbsoluteX, cycles: 7, function: crate::cpu::Cpu::inc },
        Instruction { opcode: 0xFF, name: "ISC", mode: AddressingMode::AbsoluteX, cycles: 7, function: crate::cpu::Cpu::isc },
    ];

    pub fn get_cycles(opcode: u8) -> u8 {
        INSTRUCTION_LIST[opcode as usize].cycles
    }

    pub fn get_addr_mode(opcode: u8) -> AddressingMode {
        INSTRUCTION_LIST[opcode as usize].mode
    }

    pub fn execute_instruction(opcode: u8, cpu: &mut Cpu) -> u8 {
        let instruction = &INSTRUCTION_LIST[opcode as usize];
        (instruction.function)(cpu)
    }

    pub fn print_instruction_list() {
        println!("OPCODE\tNAME\tMODE\tCYCLES");
        for instruction in INSTRUCTION_LIST.iter() {
            print!("{:02X}\t", instruction.opcode);
            print!("{}\t", instruction.name);
            print!("{:?}\t", instruction.mode);
            println!("{}", instruction.cycles);
        }
    }
}