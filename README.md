## CPU
- [ ] Implement `auto_pc_advance` instruction flag
- [ ] Real test ROMs with ca65
- [ ] Refactor + reduce code duplication in both source and tests
- [ ] Doubtful instruction implementations
  - [ ] SBC
  - [ ] BRK
  - [ ] RTI
  - [ ] RTS
- [ ] Check if status flags modified during instructions are only set if relevant, or _always_ overridden (latter is currently the case)
- [ ] Check overflow and wrapping rules for each instruction

## Bus
- [ ] Delegate read/writes to devices

## PPU
- [ ] Pre-rendered frame buffer