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
    JMP_INDIRECT    = 0x6C;
    LDA_ABSOLUTE    = 0xAD;
}
