macro_rules! opcodes {
    ($($name:ident = $value:literal;)+) => {
        $(pub const $name: u8 = $value;)+
    };
}

opcodes! {
    ADC_IMMEDIATE   = 0x69;
    JMP_INDIRECT    = 0x6C;
    LDA_ABSOLUTE    = 0xAD;
}
