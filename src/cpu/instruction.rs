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
            0xA9 => instruction!(Lda, Immediate,   2),
            0xA5 => instruction!(Lda, ZeroPage,    3),
            0xB5 => instruction!(Lda, ZeroPageX,   4),
            0xAD => instruction!(Lda, Absolute,    4),
            0xBD => instruction!(Lda, AbsoluteX,   4),
            0xB9 => instruction!(Lda, AbsoluteY,   4),
            0xA1 => instruction!(Lda, IndirectX,   6),
            0xB1 => instruction!(Lda, IndirectY,   5),
            0xEA => instruction!(Nop, Implied,     2),
            0x48 => instruction!(Pha, Implied,     3),
            0x08 => instruction!(Php, Implied,     3),
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
