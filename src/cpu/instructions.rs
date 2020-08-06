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
    AND_IMMEDIATE   = 0x29;
    AND_ZERO_PAGE   = 0x25;
    AND_ZERO_PAGE_X = 0x35;
    AND_ABSOLUTE    = 0x2D;
    AND_ABSOLUTE_X  = 0x3D;
    AND_ABSOLUTE_Y  = 0x39;
    AND_INDIRECT_X  = 0x21;
    AND_INDIRECT_Y  = 0x32;
    ASL_ACCUMULATOR = 0x0A;
    ASL_ZERO_PAGE   = 0x06;
    ASL_ZERO_PAGE_X = 0x16;
    ASL_ABSOLUTE    = 0x0E;
    ASL_ABSOLUTE_X  = 0x1E;
    BCC_RELATIVE    = 0x90;
    BCS_RELATIVE    = 0xB0;
    BEQ_RELATIVE    = 0xF0;
    BIT_ZERO_PAGE   = 0x24;
    BIT_ABSOLUTE    = 0x2C;
    BMI_RELATIVE    = 0x30;
    BNE_RELATIVE    = 0xD0;
    BPL_RELATIVE    = 0x10;
    BVC_RELATIVE    = 0x50;
    BVS_RELATIVE    = 0x70;
    CLC_IMPLIED     = 0x18;
    CLD_IMPLIED     = 0xD8;
    CLI_IMPLIED     = 0x58;
    CLV_IMPLIED     = 0xB8;
    CMP_IMMEDIATE   = 0xC9;
    CMP_ZERO_PAGE   = 0xC5;
    CMP_ZERO_PAGE_X = 0xD5;
    CMP_ABSOLUTE    = 0xCD;
    CMP_ABSOLUTE_X  = 0xDD;
    CMP_ABSOLUTE_Y  = 0xD9;
    CMP_INDIRECT_X  = 0xC1;
    CMP_INDIRECT_Y  = 0xD1;
    CPX_IMMEDIATE   = 0xE0;
    CPX_ZERO_PAGE   = 0xE4;
    CPX_ABSOLUTE    = 0xEC;
    CPY_IMMEDIATE   = 0xC0;
    CPY_ZERO_PAGE   = 0xC4;
    CPY_ABSOLUTE    = 0xCC;
    DEC_ZERO_PAGE   = 0xC6;
    DEC_ZERO_PAGE_X = 0xD6;
    DEC_ABSOLUTE    = 0xCE;
    DEC_ABSOLUTE_X  = 0xDE;
    DEX_IMPLIED     = 0xCA;
    DEY_IMPLIED     = 0x88;
    INC_ZERO_PAGE   = 0xE6;
    INC_ZERO_PAGE_X = 0xF6;
    INC_ABSOLUTE    = 0xEE;
    INC_ABSOLUTE_X  = 0xFE;
    INX_IMPLIED     = 0xE8;
    INY_IMPLIED     = 0xC8;
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
    TAX_IMPLIED     = 0xAA;
    TAY_IMPLIED     = 0xA8;
    TSX_IMPLIED     = 0xBA;
    TXA_IMPLIED     = 0x8A;
    TXS_IMPLIED     = 0x9A;
    TYA_IMPLIED     = 0x98;
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
            AND_IMMEDIATE   => (And, Immediate,   2, 2),
            AND_ZERO_PAGE   => (And, ZeroPage,    2, 3),
            AND_ZERO_PAGE_X => (And, ZeroPageX,   2, 4),
            AND_ABSOLUTE    => (And, Absolute,    3, 4),
            AND_ABSOLUTE_X  => (And, AbsoluteX,   3, 4),
            AND_ABSOLUTE_Y  => (And, AbsoluteY,   3, 4),
            AND_INDIRECT_X  => (And, IndirectX,   2, 6),
            AND_INDIRECT_Y  => (And, IndirectY,   2, 5),
            ASL_ACCUMULATOR => (Asl, Accumulator, 1, 2),
            ASL_ZERO_PAGE   => (Asl, ZeroPage,    2, 5),
            ASL_ZERO_PAGE_X => (Asl, ZeroPageX,   2, 6),
            ASL_ABSOLUTE    => (Asl, Absolute,    3, 6),
            ASL_ABSOLUTE_X  => (Asl, AbsoluteX,   3, 7),
            BIT_ZERO_PAGE   => (Bit, ZeroPage,    2, 3),
            BIT_ABSOLUTE    => (Bit, Absolute,    3, 4),
            BCC_RELATIVE    => (Bcc, Relative,    2, 2),
            BCS_RELATIVE    => (Bcs, Relative,    2, 2),
            BEQ_RELATIVE    => (Beq, Relative,    2, 2),
            BMI_RELATIVE    => (Bmi, Relative,    2, 2),
            BNE_RELATIVE    => (Bne, Relative,    2, 2),
            BPL_RELATIVE    => (Bpl, Relative,    2, 2),
            BVC_RELATIVE    => (Bvc, Relative,    2, 2),
            BVS_RELATIVE    => (Bvs, Relative,    2, 2),
            CLC_IMPLIED     => (Clc, Implied,     1, 2),
            CLD_IMPLIED     => (Cld, Implied,     1, 2),
            CLI_IMPLIED     => (Cli, Implied,     1, 2),
            CLV_IMPLIED     => (Clv, Implied,     1, 2),
            CMP_IMMEDIATE   => (Cmp, Immediate,   2, 2),
            CMP_ZERO_PAGE   => (Cmp, ZeroPage,    2, 3),
            CMP_ZERO_PAGE_X => (Cmp, ZeroPageX,   2, 4),
            CMP_ABSOLUTE    => (Cmp, Absolute,    3, 4),
            CMP_ABSOLUTE_X  => (Cmp, AbsoluteX,   3, 4),
            CMP_ABSOLUTE_Y  => (Cmp, AbsoluteY,   3, 4),
            CMP_INDIRECT_X  => (Cmp, IndirectX,   2, 6),
            CMP_INDIRECT_Y  => (Cmp, IndirectY,   2, 5),
            CPX_IMMEDIATE   => (Cpx, Immediate,   2, 2),
            CPX_ZERO_PAGE   => (Cpx, ZeroPage,    2, 3),
            CPX_ABSOLUTE    => (Cpx, Absolute,    3, 4),
            CPY_IMMEDIATE   => (Cpy, Immediate,   2, 2),
            CPY_ZERO_PAGE   => (Cpy, ZeroPage,    2, 3),
            CPY_ABSOLUTE    => (Cpy, Absolute,    3, 4),
            DEC_ZERO_PAGE   => (Dec, ZeroPage,    2, 5),
            DEC_ZERO_PAGE_X => (Dec, ZeroPageX,   2, 6),
            DEC_ABSOLUTE    => (Dec, Absolute,    3, 6),
            DEC_ABSOLUTE_X  => (Dec, AbsoluteX,   3, 7),
            DEX_IMPLIED     => (Dex, Implied,     1, 2),
            DEY_IMPLIED     => (Dey, Implied,     1, 2),
            INC_ZERO_PAGE   => (Inc, ZeroPage,    2, 5),
            INC_ZERO_PAGE_X => (Inc, ZeroPageX,   2, 6),
            INC_ABSOLUTE    => (Inc, Absolute,    3, 6),
            INC_ABSOLUTE_X  => (Inc, AbsoluteX,   3, 7),
            INX_IMPLIED     => (Inx, Implied,     1, 2),
            INY_IMPLIED     => (Iny, Implied,     1, 2),
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
            TAX_IMPLIED     => (Tax, Implied,     1, 2),
            TAY_IMPLIED     => (Tay, Implied,     1, 2),
            TSX_IMPLIED     => (Tsx, Implied,     1, 2),
            TXA_IMPLIED     => (Txa, Implied,     1, 2),
            TXS_IMPLIED     => (Txs, Implied,     1, 2),
            TYA_IMPLIED     => (Tya, Implied,     1, 2),
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
