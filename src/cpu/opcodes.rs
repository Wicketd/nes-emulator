macro_rules! opcodes {
    ($($name:ident = $value:literal;)+) => {
        $(pub const $name: u8 = $value;)+
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
    SEC_IMPLIED     = 0x38;
    SED_IMPLIED     = 0xF8;
    SEI_IMPLIED     = 0x78;
}
