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
- [x] BEQ
- [x] BIT
- [x] BMI
- [x] BNE
- [x] BPL
- [ ] BRK
- [x] BVC
- [x] BVS
- [x] CLC
- [x] CLD
- [x] CLI
- [x] CLV
- [x] CMP
- [x] CPX
- [x] CPY
- [x] DEC
- [x] DEX
- [x] DEY
- [ ] EOR
- [x] INC
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
- [x] TAX
- [x] TAY
- [x] TSX
- [x] TXA
- [x] TXS
- [x] TYA

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
