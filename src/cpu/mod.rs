mod opcodes;
mod instruction;
mod tests;

use self::instruction::{Instruction, InstructionOperation, InstructionMode};
use crate::bus::Bus;
use crate::types::{Address, Result, BitRead};

const ADDRESS_NMI: Address = 0xFFFA;
const ADDRESS_RESET: Address = 0xFFFC;
const ADDRESS_IRQ: Address = 0xFFFE;

pub struct Cpu {
    bus: Bus,
    registers: RegisterSet,
    vectors: VectorSet,
}

impl Cpu {
    pub fn new(bus: Bus) -> Result<Self> {
        let vectors = VectorSet {
            nmi: bus.read_u16(ADDRESS_NMI)?,
            reset: bus.read_u16(ADDRESS_RESET)?,
            irq: bus.read_u16(ADDRESS_IRQ)?,
        };

        let mut registers = RegisterSet::new();
        registers.pc = vectors.reset;

        Ok(Self { bus, registers, vectors })
    }

    pub fn start(&mut self) -> Result {
        while let Some(instruction) = self.determine_instruction_next()? {
            self.process_instruction(instruction)?;
        }

        Ok(())
    }

    fn determine_instruction_next(&self) -> Result<Option<Instruction>> {
        let opcode = self.bus.read(self.registers.pc);
        let instruction = Instruction::from_opcode(opcode);

        // TODO: check if this is correct
        if self.registers.pc + (instruction.len() as Address) < ADDRESS_NMI {
            Ok(Some(instruction))
        } else {
            Ok(None)
        }
    }

    fn process_instruction(&mut self, instruction: Instruction) -> Result {
        // account for opcode
        self.registers.pc += 1;
        let bytes = self.bus.read_n(self.registers.pc, instruction.len() as u16 - 1);

        Ok(self.run_instruction(instruction, &bytes)?)
    }

    fn run_instruction(&mut self, instruction: Instruction, bytes: &[u8]) -> Result {
        match instruction.operation() {
            InstructionOperation::Adc => {
                let input = self.determine_input_byte(instruction.mode(), bytes)?.unwrap();
                self.run_adc(input);
            },
            _ => unimplemented!(),
        }

        Ok(())
    }

    // TODO: clear error messages
    fn determine_input_byte(&self, mode: InstructionMode, bytes: &[u8]) -> Result<Option<u8>> {
        let input = match mode {
            InstructionMode::Implied => None,
            InstructionMode::Accumulator => unimplemented!("input byte | Accumulator"),
            InstructionMode::Immediate => Some(bytes[0]),
            InstructionMode::Relative => unimplemented!("input byte | Relative"),
            InstructionMode::ZeroPage => unimplemented!("input byte | ZeroPage"),
            InstructionMode::ZeroPageX => unimplemented!("input byte | ZeroPageX"),
            InstructionMode::ZeroPageY => unimplemented!("input byte | ZeroPageY"),
            InstructionMode::Absolute => {
                let address = u16::from_le_bytes([bytes[0], bytes[1]]);
                Some(self.bus.read(address))
            },
            InstructionMode::AbsoluteX => {
                let address = u16::from_le_bytes([bytes[0], bytes[1]]);
                // TODO: overflow check
                Some(self.bus.read(address + self.registers.x as Address))
            },
            InstructionMode::AbsoluteY => unimplemented!("input byte | AbsoluteY"),
            InstructionMode::Indirect => unimplemented!("input byte | Indirect"),
            InstructionMode::IndirectX => unimplemented!("input byte | IndirectX"),
            InstructionMode::IndirectY => unimplemented!("input byte | IndirectY"),
        };

        Ok(input)
    }

    fn run_adc(&mut self, input: u8) {
        let carry = (self.registers.p & StatusFlags::CARRY).bits();
        let a_old = self.registers.a;
        let a_new = self.registers.a.wrapping_add(input).wrapping_add(carry);
        self.registers.a = a_new;

        self.registers.p.set(StatusFlags::CARRY, is_carry(input, a_new));
        self.registers.p.set(StatusFlags::ZERO, a_new == 0);
        self.registers.p.set(StatusFlags::OVERFLOW, has_overflown(a_old, a_new));
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(a_new));
    }
}

fn is_carry(input: u8, value_new: u8) -> bool {
    value_new < input
}

fn has_overflown(value_old: u8, value_new: u8) -> bool {
    value_old.read_bit(7) != value_new.read_bit(7)
}

fn is_negative(value: u8) -> bool {
    value.is_bit_set(7)
}

#[derive(Debug, Eq, PartialEq)]
struct RegisterSet {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    p: StatusFlags,
    pc: Address,
}

impl RegisterSet {
    fn new() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            s: 0xFF,
            p: StatusFlags::empty(),
            pc: 0,
        }
    }
}

struct VectorSet {
    nmi: Address,
    reset: Address,
    irq: Address,
}

bitflags! {
    struct StatusFlags: u8 {
        const NEGATIVE = 0b1000_0000;
        const OVERFLOW = 0b0100_0000;
        const BREAK_LEFT = 0b0010_0000;
        const BREAK_RIGHT = 0b0001_0000;
        const DECIMAL = 0b0000_1000;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const ZERO = 0b0000_0010;
        const CARRY = 0b0000_0001;
    }
}

impl StatusFlags {
    fn set_break(&mut self, break_type: BreakType) {
        match break_type {
            BreakType::Internal => {
                self.insert(StatusFlags::BREAK_LEFT);
                self.insert(StatusFlags::BREAK_RIGHT);
            },
            BreakType::Instruction => {
                self.insert(StatusFlags::BREAK_LEFT);
                self.remove(StatusFlags::BREAK_RIGHT);
            },
        }
    }

    fn clear_break(&mut self) {
        self.remove(StatusFlags::BREAK_LEFT);
        self.remove(StatusFlags::BREAK_RIGHT);
    }
}

enum BreakType {
    Internal,
    Instruction,
}

#[derive(Debug, Eq, PartialEq)]
enum Location {
    Accumulator,
    Address(Address),
}
