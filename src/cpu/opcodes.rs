macro_rules! opcodes {
    ($($name:ident = $value:literal;)+) => {
        $(pub const $name: u8 = $value;)+
    };
}

opcodes! {
    ADC_IMMEDIATE   = 0x69;
    ASL_ACCUMULATOR = 0x0A;
    ASL_ZERO_PAGE_X = 0x16;
    CLC_IMPLIED     = 0x18;
    CLD_IMPLIED     = 0xD8;
    CLI_IMPLIED     = 0x58;
    CLV_IMPLIED     = 0xB8;
    INX_IMPLIED     = 0xE8;
    INY_IMPLIED     = 0xC8;
    LDA_ABSOLUTE    = 0xAD;
    NOP_IMPLIED     = 0xEA;
    SEC_IMPLIED     = 0x38;
    SED_IMPLIED     = 0xF8;
    SEI_IMPLIED     = 0x78;
    TAX_IMPLIED     = 0xAA;
    TAY_IMPLIED     = 0xA8;
    TXA_IMPLIED     = 0x8A;
    TYA_IMPLIED     = 0x98;
}
