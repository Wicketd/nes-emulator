# NES emulator

Just a simple NES emulator to get myself accustomed to both Rust and a lower level of programming, coming from a web development world.

This README really only functions as my personal to-do list. Why are you here?

## Instructions

Note: Partial mode coverage, implement full mode coverage after the instructions themselves.

- [x] ADC
- [ ] AND
- [ ] ASL
- [ ] BCC
- [ ] BCS
- [ ] BEQ
- [ ] BIT
- [ ] BMI
- [ ] BNE
- [ ] BPL
- [ ] BRK
- [ ] BVC
- [ ] BVS
- [ ] CLC
- [ ] CLD
- [ ] CLI
- [ ] CLV
- [ ] CMP
- [ ] CPX
- [ ] CPY
- [ ] DEC
- [ ] DEX
- [ ] DEY
- [ ] EOR
- [ ] INC
- [ ] INX
- [ ] INY
- [ ] JMP
- [ ] JSR
- [ ] LDA
- [ ] LDX
- [ ] LDY
- [ ] LSR
- [ ] NOP
- [ ] ORA
- [ ] PHA
- [ ] PHP
- [ ] PLA
- [ ] PLP
- [ ] ROL
- [ ] ROR
- [ ] RTI
- [ ] RTS
- [ ] SBC
- [ ] SEC
- [ ] SED
- [ ] SEI
- [ ] STA
- [ ] STX
- [ ] STY
- [ ] TAX
- [ ] TAY
- [ ] TSX
- [ ] TXA
- [ ] TXS
- [ ] TYA

## Instruction modes
- [ ] Byte
  - [x] Implied
  - [x] Accumulator
  - [x] Relative
  - [x] Immediate
  - [x] ZeroPage
  - [x] ZeroPageX
  - [x] ZeroPageY
  - [x] Absolute
  - [x] AbsoluteX
  - [x] AbsoluteY
  - [ ] Indirect
  - [ ] IndirectX
  - [ ] IndirectY
  - [ ] Carries
  - [ ] Overflows
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
  - [ ] Carries
  - [ ] Overflows

## Miscellaneous
- [ ] Implement CPU wrapping + page boundary error (check opcode list @ 6502.org)
