use crate::bus::Bus;
use crate::types::Result;

#[derive(Debug, CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Instruction {
    operation: InstructionOperation,
    mode: InstructionMode,
    len: u8,
    cycles_base: u8,
}

macro_rules! instruction {
    ($operation:ident, $mode:ident, $cycles_base:literal) => {{
        let mode = InstructionMode::$mode;

        Instruction {
            operation: InstructionOperation::$operation,
            mode,
            len: mode.len_bytes(),
            cycles_base: $cycles_base,
        }
    }};
}

impl Instruction {
    pub fn from_opcode(opcode: u8) -> Instruction {
        match opcode {
            0x69 => instruction!(Adc, Immediate,   2),
            0x65 => instruction!(Adc, ZeroPage,    3),
            0x75 => instruction!(Adc, ZeroPageX,   4),
            0x6D => instruction!(Adc, Absolute,    4),
            0x7D => instruction!(Adc, AbsoluteX,   4),
            0x79 => instruction!(Adc, AbsoluteY,   4),
            0x61 => instruction!(Adc, IndirectX,   6),
            0x71 => instruction!(Adc, IndirectY,   5),
            0x29 => instruction!(And, Immediate,   2),
            0x25 => instruction!(And, ZeroPage,    3),
            0x35 => instruction!(And, ZeroPageX,   4),
            0x2D => instruction!(And, Absolute,    4),
            0x3D => instruction!(And, AbsoluteX,   4),
            0x39 => instruction!(And, AbsoluteY,   4),
            0x21 => instruction!(And, IndirectX,   6),
            0x31 => instruction!(And, IndirectY,   5),
            0x0A => instruction!(Asl, Accumulator, 2),
            0x06 => instruction!(Asl, ZeroPage,    5),
            0x16 => instruction!(Asl, ZeroPageX,   6),
            0x0E => instruction!(Asl, Absolute,    6),
            0x1E => instruction!(Asl, AbsoluteX,   7),
            0x90 => instruction!(Bcc, Relative,    2),
            0xB0 => instruction!(Bcs, Relative,    2),
            0xF0 => instruction!(Beq, Relative,    2),
            0x24 => instruction!(Bit, ZeroPage,    3),
            0x2C => instruction!(Bit, Absolute,    4),
            0x30 => instruction!(Bmi, Relative,    2),
            0xD0 => instruction!(Bne, Relative,    2),
            0x10 => instruction!(Bpl, Relative,    2),
            0x00 => instruction!(Brk, Implied,     7),
            0x50 => instruction!(Bvc, Relative,    2),
            0x70 => instruction!(Bvs, Relative,    2),
            0x18 => instruction!(Clc, Implied,     1),
            0xD8 => instruction!(Cld, Implied,     1),
            0x58 => instruction!(Cli, Implied,     1),
            0xB8 => instruction!(Clv, Implied,     1),
            0xC9 => instruction!(Cmp, Immediate,   2),
            0xC5 => instruction!(Cmp, ZeroPage,    3),
            0xD5 => instruction!(Cmp, ZeroPageX,   4),
            0xCD => instruction!(Cmp, Absolute,    4),
            0xDD => instruction!(Cmp, AbsoluteX,   4),
            0xD9 => instruction!(Cmp, AbsoluteY,   4),
            0xC1 => instruction!(Cmp, IndirectX,   6),
            0xD1 => instruction!(Cmp, IndirectY,   5),
            0xE0 => instruction!(Cpx, Immediate,   2),
            0xE4 => instruction!(Cpx, ZeroPage,    3),
            0xEC => instruction!(Cpx, Absolute,    4),
            0xC0 => instruction!(Cpy, Immediate,   2),
            0xC4 => instruction!(Cpy, ZeroPage,    3),
            0xCC => instruction!(Cpy, Absolute,    4),
            0xC6 => instruction!(Dec, ZeroPage,    5),
            0xD6 => instruction!(Dec, ZeroPageX,   6),
            0xCE => instruction!(Dec, Absolute,    6),
            0xDE => instruction!(Dec, AbsoluteX,   7),
            0xCA => instruction!(Dex, Implied,     2),
            0x88 => instruction!(Dey, Implied,     2),
            0x49 => instruction!(Eor, Immediate,   2),
            0x45 => instruction!(Eor, ZeroPage,    3),
            0x55 => instruction!(Eor, ZeroPageX,   4),
            0x4D => instruction!(Eor, Absolute,    4),
            0x5D => instruction!(Eor, AbsoluteX,   4),
            0x59 => instruction!(Eor, AbsoluteY,   4),
            0x41 => instruction!(Eor, IndirectX,   6),
            0x51 => instruction!(Eor, IndirectY,   5),
            0xE6 => instruction!(Inc, ZeroPage,    5),
            0xF6 => instruction!(Inc, ZeroPageX,   6),
            0xEE => instruction!(Inc, Absolute,    6),
            0xFE => instruction!(Inc, AbsoluteX,   7),
            0xE8 => instruction!(Inx, Implied,     2),
            0xC8 => instruction!(Iny, Implied,     2),
            0x4C => instruction!(Jmp, Absolute,    3),
            0x6C => instruction!(Jmp, Indirect,    5),
            0x20 => instruction!(Jsr, Absolute,    6),
            0xA9 => instruction!(Lda, Immediate,   2),
            0xA5 => instruction!(Lda, ZeroPage,    3),
            0xB5 => instruction!(Lda, ZeroPageX,   4),
            0xAD => instruction!(Lda, Absolute,    4),
            0xBD => instruction!(Lda, AbsoluteX,   4),
            0xB9 => instruction!(Lda, AbsoluteY,   4),
            0xA1 => instruction!(Lda, IndirectX,   6),
            0xB1 => instruction!(Lda, IndirectY,   5),
            0xA2 => instruction!(Ldx, Immediate,   2),
            0xA6 => instruction!(Ldx, ZeroPage,    3),
            0xB6 => instruction!(Ldx, ZeroPageY,   4),
            0xAE => instruction!(Ldx, Absolute,    4),
            0xBE => instruction!(Ldx, AbsoluteY,   4),
            0xA0 => instruction!(Ldy, Immediate,   2),
            0xA4 => instruction!(Ldy, ZeroPage,    3),
            0xB4 => instruction!(Ldy, ZeroPageX,   4),
            0xAC => instruction!(Ldy, Absolute,    4),
            0xBC => instruction!(Ldy, AbsoluteX,   4),
            0x4A => instruction!(Lsr, Accumulator, 2),
            0x46 => instruction!(Lsr, ZeroPage,    5),
            0x56 => instruction!(Lsr, ZeroPageX,   6),
            0x4E => instruction!(Lsr, Absolute,    6),
            0x5E => instruction!(Lsr, AbsoluteX,   7),
            0xEA => instruction!(Nop, Implied,     2),
            0x09 => instruction!(Ora, Immediate,   2),
            0x05 => instruction!(Ora, ZeroPage,    3),
            0x15 => instruction!(Ora, ZeroPageX,   4),
            0x0D => instruction!(Ora, Absolute,    4),
            0x1D => instruction!(Ora, AbsoluteX,   4),
            0x19 => instruction!(Ora, AbsoluteY,   4),
            0x01 => instruction!(Ora, IndirectX,   6),
            0x11 => instruction!(Ora, IndirectY,   5),
            0x48 => instruction!(Pha, Implied,     3),
            0x08 => instruction!(Php, Implied,     3),
            0x68 => instruction!(Pla, Implied,     4),
            0x28 => instruction!(Plp, Implied,     4),
            0x2A => instruction!(Rol, Accumulator, 2),
            0x26 => instruction!(Rol, ZeroPage,    5),
            0x36 => instruction!(Rol, ZeroPageX,   6),
            0x2E => instruction!(Rol, Absolute,    6),
            0x3E => instruction!(Rol, AbsoluteX,   7),
            0x6A => instruction!(Ror, Accumulator, 2),
            0x66 => instruction!(Ror, ZeroPage,    5),
            0x76 => instruction!(Ror, ZeroPageX,   6),
            0x6E => instruction!(Ror, Absolute,    6),
            0x7E => instruction!(Ror, AbsoluteX,   7),
            0x40 => instruction!(Rti, Implied,     6),
            0x38 => instruction!(Sec, Implied,     2),
            0xF8 => instruction!(Sed, Implied,     2),
            0x78 => instruction!(Sei, Implied,     2),
            0x85 => instruction!(Sta, ZeroPage,    3),
            0x95 => instruction!(Sta, ZeroPageX,   4),
            0x8D => instruction!(Sta, Absolute,    4),
            0x9D => instruction!(Sta, AbsoluteX,   5),
            0x99 => instruction!(Sta, AbsoluteY,   5),
            0x81 => instruction!(Sta, IndirectX,   6),
            0x91 => instruction!(Sta, IndirectY,   6),
            0x86 => instruction!(Stx, ZeroPage,    3),
            0x96 => instruction!(Stx, ZeroPageY,   4),
            0x8E => instruction!(Stx, Absolute,    4),
            0x84 => instruction!(Sty, ZeroPage,    3),
            0x94 => instruction!(Sty, ZeroPageX,   4),
            0x8C => instruction!(Sty, Absolute,    4),
            0xAA => instruction!(Tax, Implied,     2),
            0xA8 => instruction!(Tay, Implied,     2),
            0xBA => instruction!(Tsx, Implied,     2),
            0x8A => instruction!(Txa, Implied,     2),
            0x9A => instruction!(Txs, Implied,     2),
            0x98 => instruction!(Tya, Implied,     2),
            _ => unimplemented!("no instruction found for opcode `${:02X}`", opcode),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum InstructionOperation {
    Adc, And, Asl, Bcc, Bcs, Beq, Bit, Bmi, Bne, Bpl, Brk, Bvc, Bvs, Clc,
    Cld, Cli, Clv, Cmp, Cpx, Cpy, Dec, Dex, Dey, Eor, Inc, Inx, Iny, Jmp,
    Jsr, Lda, Ldx, Ldy, Lsr, Nop, Ora, Pha, Php, Pla, Plp, Rol, Ror, Rti,
    Rts, Sbc, Sec, Sed, Sei, Sta, Stx, Sty, Tax, Tay, Tsx, Txa, Txs, Tya,
}

#[derive(Debug, Copy, Clone)]
pub enum InstructionMode {
    Implied,
    Accumulator,
    Immediate,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

impl InstructionMode {
    pub fn len_bytes(&self) -> u8 {
        match self {
            InstructionMode::Implied | InstructionMode::Accumulator => 1,
            InstructionMode::Immediate
                | InstructionMode::Relative
                | InstructionMode::ZeroPage
                | InstructionMode::ZeroPageX
                | InstructionMode::ZeroPageY
                | InstructionMode::IndirectX
                | InstructionMode::IndirectY
                => 2,
            InstructionMode::Absolute
                | InstructionMode::AbsoluteX
                | InstructionMode::AbsoluteY
                | InstructionMode::Indirect
                => 3,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InstructionInput {
    Implied,
    Byte(u8),
    Location(InstructionInputLocation),
}

#[derive(Debug, PartialEq)]
pub enum InstructionInputLocation {
    Accumulator,
    Address(u16),
}

impl InstructionInput {
    pub fn from_address(address: u16) -> Self {
        Self::Location(InstructionInputLocation::Address(address))
    }

    pub fn unwrap_location(self) -> Result<InstructionInputLocation> {
        match self {
            InstructionInput::Location(location) => Ok(location),
            _ => Err(anyhow!("input is not a location")),
        }
    }

    pub fn unwrap_address(self) -> Result<u16> {
        match self.unwrap_location()? {
            InstructionInputLocation::Address(address) => Ok(address),
            _ => Err(anyhow!("input is not an address")),
        }
    }
}
