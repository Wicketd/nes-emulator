# NES emulator

Just a simple NES emulator to get myself accustomed to both Rust and a lower level of programming, coming from a web development world.

This README really only functions as my personal to-do list. Why are you here?

## Instructions

Note: Partial mode coverage, implement full mode coverage after the instructions themselves.

- [x] ADC
- [x] AND
- [x] ASL
- [x] BCC
- [x] BCS
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
- [x] JMP
- [ ] JSR
- [x] LDA
- [ ] LDX
- [ ] LDY
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
- [X] SEC
- [X] SED
- [X] SEI
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
- [x] Indirect
- [x] IndirectX
- [x] IndirectY
- [ ] Carries
- [ ] Overflows

## Miscellaneous
- [ ] Implement CPU wrapping + page boundary error (check opcode list @ 6502.org)
