macro_rules! opcodes {
    ($($name:ident = $value:literal;)+) => {
        $(pub const $name: u8 = $value;)+
    };
}

opcodes! {
    ADC_IMMEDIATE   = 0x69;
    ADC_ZERO_PAGE   = 0x65;
    ADC_ZERO_PAGE_X = 0x75;
    CLC_IMPLIED     = 0x18;
    CLD_IMPLIED     = 0xD8;
    CLI_IMPLIED     = 0x58;
    CLV_IMPLIED     = 0xB8;
    LDA_ABSOLUTE    = 0xAD;
    NOP_IMPLIED     = 0xEA;
    SEC_IMPLIED     = 0x38;
    SED_IMPLIED     = 0xF8;
    SEI_IMPLIED     = 0x78;
}
