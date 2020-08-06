use self::opcodes::*;

macro_rules! opcodes {
    ($($name:ident = $value:literal;)+) => {
        pub mod opcodes {
            $(pub const $name: u8 = $value;)+
        }
    };
}

opcodes! {
    ADC_IMMEDIATE   = 0x69;
    ADC_ZERO_PAGE   = 0x65;
    ADC_ZERO_PAGE_X = 0x75;
    ADC_ABSOLUTE    = 0x6D;
    ADC_ABSOLUTE_X  = 0x7D;
    ADC_ABSOLUTE_Y  = 0x79;
    ADC_INDIRECT_X  = 0x61;
    ADC_INDIRECT_Y  = 0x71;
    CLC_IMPLIED     = 0x18;
    CLD_IMPLIED     = 0xD8;
    CLI_IMPLIED     = 0x58;
    JMP_ABSOLUTE    = 0x4C;
    JMP_INDIRECT    = 0x6C;
    LDA_IMMEDIATE   = 0xA9;
    LDA_ZERO_PAGE   = 0xA5;
    LDA_ZERO_PAGE_X = 0xB5;
    LDA_ABSOLUTE    = 0xAD;
    LDA_ABSOLUTE_X  = 0xBD;
    LDA_ABSOLUTE_Y  = 0xB9;
    LDA_INDIRECT_X  = 0xA1;
    LDA_INDIRECT_Y  = 0xB1;
    NOP_IMPLIED     = 0xEA;
    SEC_IMPLIED     = 0x38;
    SED_IMPLIED     = 0xF8;
    SEI_IMPLIED     = 0x78;
}

macro_rules! match_opcode {
    (
        use $opcode_ident:ident;

        $($opcode:ident => (
            $operation:ident,
            $mode:ident,
            $len:literal,
            $cycles_base:literal
        ),)+
    ) => {
        match $opcode_ident {
            $($opcode => Instruction {
                opcode: $opcode,
                operation: InstructionOperation::$operation,
                mode: InstructionMode::$mode,
                len: $len,
                cycles_base: $cycles_base,
            },)+
            _ => unimplemented!("no instruction found for opcode `${:02X}`", $opcode_ident),
        }
    };
}

#[derive(CopyGetters)]
#[getset(get_copy = "pub")]
pub struct Instruction {
    opcode: u8,
    operation: InstructionOperation,
    mode: InstructionMode,
    len: u8,
    cycles_base: u8,
}

impl Instruction {
    pub fn from_opcode(opcode: u8) -> Self {
        match_opcode! {
            use opcode;

            // opcode => (operation, mode, len, cycles_base)
            ADC_IMMEDIATE   => (Adc, Immediate,   2, 2),
            ADC_ZERO_PAGE   => (Adc, ZeroPage,    2, 3),
            ADC_ZERO_PAGE_X => (Adc, ZeroPageX,   2, 4),
            ADC_ABSOLUTE    => (Adc, Absolute,    3, 4),
            ADC_ABSOLUTE_X  => (Adc, AbsoluteX,   3, 4),
            ADC_ABSOLUTE_Y  => (Adc, AbsoluteY,   3, 4),
            ADC_INDIRECT_X  => (Adc, IndirectX,   2, 6),
            ADC_INDIRECT_Y  => (Adc, IndirectY,   2, 5),
            CLC_IMPLIED     => (Clc, Implied,     1, 2),
            CLD_IMPLIED     => (Cld, Implied,     1, 2),
            CLI_IMPLIED     => (Cli, Implied,     1, 2),
            JMP_ABSOLUTE    => (Jmp, Absolute,    3, 3),
            JMP_INDIRECT    => (Jmp, Indirect,    3, 5),
            LDA_IMMEDIATE   => (Lda, Immediate,   2, 2),
            LDA_ZERO_PAGE   => (Lda, ZeroPage,    2, 3),
            LDA_ZERO_PAGE_X => (Lda, ZeroPageX,   2, 4),
            LDA_ABSOLUTE    => (Lda, Absolute,    3, 4),
            LDA_ABSOLUTE_X  => (Lda, AbsoluteX,   3, 4),
            LDA_ABSOLUTE_Y  => (Lda, AbsoluteY,   3, 4),
            LDA_INDIRECT_X  => (Lda, IndirectX,   2, 6),
            LDA_INDIRECT_Y  => (Lda, IndirectY,   2, 5),
            NOP_IMPLIED     => (Nop, Implied,     1, 2),
            SEC_IMPLIED     => (Sec, Implied,     1, 2),
            SED_IMPLIED     => (Sed, Implied,     1, 2),
            SEI_IMPLIED     => (Sei, Implied,     1, 2),
        }
    }
}

#[derive(Debug, Copy, Clone)]
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
