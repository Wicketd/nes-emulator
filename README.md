# NES emulator

Just a simple NES emulator to get myself accustomed to both Rust and a lower level of programming, coming from a web development world.

This README really only functions as my personal to-do list. Why are you here?

## Instructions

Note: Partial mode coverage, implement full mode coverage after the instructions themselves.

- [x] ADC
- [ ] AND
- [x] ASL
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
- [x] CLC
- [x] CLD
- [x] CLI
- [x] CLV
- [ ] CMP
- [ ] CPX
- [ ] CPY
- [ ] DEC
- [ ] DEX
- [ ] DEY
- [ ] EOR
- [ ] INC
- [x] INX
- [x] INY
- [ ] JMP
- [ ] JSR
- [x] LDA
- [x] LDX
- [x] LDY
- [ ] LSR
- [x] NOP
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
- [x] SEC
- [x] SED
- [x] SEI
- [ ] STA
- [ ] STX
- [ ] STY
- [x] TAX
- [x] TAY
- [ ] TSX
- [x] TXA
- [ ] TXS
- [x] TYA

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
  - [x] Accumulator
  - [x] Relative
  - [ ] Immediate
  - [ ] ZeroPage
  - [x] ZeroPageX
  - [ ] Absolute
  - [ ] AbsoluteX
  - [ ] AbsoluteY
  - [ ] Indirect
  - [ ] IndirectX
  - [ ] IndirectY

## Miscellaneous
- [ ] Implement CPU wrapping + page boundary error (check opcode list @ 6502.org)
