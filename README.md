# NES emulator

Just a simple NES emulator to get myself accustomed to both Rust and a lower level of programming, coming from a web development world.

This README really only functions as my personal to-do list. Why are you here?

## Instructions

Note: Partial mode coverage, implement full mode coverage after the instructions themselves.

- [ ] ADC
- [ ] AND
- [ ] ASL
- [ ] BIT
- [ ] BPL
- [ ] BMI
- [ ] BVC
- [ ] BVS
- [ ] BCC
- [ ] BCS
- [ ] BNE
- [ ] BEQ
- [ ] BRK
- [ ] CMP
- [ ] CPX
- [ ] CPY
- [ ] DEC
- [ ] EOR
- [x] CLC
- [x] SEC
- [x] CLI
- [x] SEI
- [x] CLV
- [x] CLD
- [x] SED
- [ ] INC
- [ ] JMP
- [ ] JSR
- [x] LDA
- [ ] LDX
- [ ] LDY
- [ ] LSR
- [ ] NOP
- [ ] ORA
- [ ] ROL
- [ ] ROR
- [ ] RTI
- [ ] RTS
- [ ] SBC
- [ ] STA
- [ ] TXS
- [ ] TSX
- [ ] PHA
- [ ] PLA
- [ ] PHP
- [ ] PLP
- [ ] STX
- [ ] STY

## Instruction modes
- [ ] Byte
  - [x] Implied
  - [ ] Accumulator
  - [ ] Relative
  - [x] Immediate
  - [ ] ZeroPage
  - [ ] ZeroPageX
  - [x] Absolute
  - [ ] AbsoluteX
  - [ ] AbsoluteY
  - [ ] Indirect
  - [ ] IndirectX
  - [ ] IndirectY
- [ ] Location
  - [ ] Implied
  - [ ] Accumulator
  - [ ] Relative
  - [ ] Immediate
  - [ ] ZeroPage
  - [ ] ZeroPageX
  - [ ] Absolute
  - [ ] AbsoluteX
  - [ ] AbsoluteY
  - [ ] Indirect
  - [ ] IndirectX
  - [ ] IndirectY

## Miscellaneous
- [ ] Implement CPU wrapping + page boundary error (check opcode list @ 6502.org)
