mod instructions;
mod tests;

use self::instructions::{Instruction, InstructionOperation, InstructionMode};
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
        let bytes = self.bus.read_n(self.registers.pc, instruction.len() as u16);
        let len = instruction.len();
        self.run_instruction(instruction, &bytes[1..])?;
        self.registers.pc += len as Address;
        Ok(())
    }

    // TODO: no calls to unwrap()
    // TODO: cut down on duplicated code
    fn run_instruction(&mut self, instruction: Instruction, bytes: &[u8]) -> Result {
        match instruction.operation() {
            InstructionOperation::Adc => {
                let input = self.determine_input_byte(instruction.mode(), bytes)?.unwrap();
                self.run_adc(input);
            },
            InstructionOperation::And => {
                let input = self.determine_input_byte(instruction.mode(), bytes)?.unwrap();
                self.run_and(input);
            },
            InstructionOperation::Asl => {
                let target = self.resolve_location_by_mode(instruction.mode(), bytes)?.unwrap();
                self.run_asl(target);
            },
            InstructionOperation::Bcc => {
                if !self.registers.p.contains(StatusFlags::CARRY) {
                    let target = self.resolve_address_by_mode(instruction.mode(), bytes)?;
                    self.run_branch(target);
                }
            },
            InstructionOperation::Bcs => {
                if self.registers.p.contains(StatusFlags::CARRY) {
                    let target = self.resolve_address_by_mode(instruction.mode(), bytes)?;
                    self.run_branch(target);
                }
            },
            InstructionOperation::Beq => {
                if self.registers.p.contains(StatusFlags::ZERO) {
                    let target = self.resolve_address_by_mode(instruction.mode(), bytes)?;
                    self.run_branch(target);
                }
            },
            InstructionOperation::Bmi => {
                if self.registers.p.contains(StatusFlags::NEGATIVE) {
                    let target = self.resolve_address_by_mode(instruction.mode(), bytes)?;
                    self.run_branch(target);
                }
            },
            InstructionOperation::Bne => {
                if !self.registers.p.contains(StatusFlags::ZERO) {
                    let target = self.resolve_address_by_mode(instruction.mode(), bytes)?;
                    self.run_branch(target);
                }
            },
            InstructionOperation::Bpl => {
                if !self.registers.p.contains(StatusFlags::NEGATIVE) {
                    let target = self.resolve_address_by_mode(instruction.mode(), bytes)?;
                    self.run_branch(target);
                }
            },
            InstructionOperation::Bvc => {
                if !self.registers.p.contains(StatusFlags::OVERFLOW) {
                    let target = self.resolve_address_by_mode(instruction.mode(), bytes)?;
                    self.run_branch(target);
                }
            },
            InstructionOperation::Bvs => {
                if self.registers.p.contains(StatusFlags::OVERFLOW) {
                    let target = self.resolve_address_by_mode(instruction.mode(), bytes)?;
                    self.run_branch(target);
                }
            },
            InstructionOperation::Clc => self.run_clc(),
            InstructionOperation::Cld => self.run_cld(),
            InstructionOperation::Cli => self.run_cli(),
            InstructionOperation::Jmp => {
                let target = self.resolve_address_by_mode(instruction.mode(), bytes)?;
                // TODO: hacky
                let target = target.wrapping_sub(instruction.len() as Address);
                self.run_jmp(target);
            }
            InstructionOperation::Lda => {
                let input = self.determine_input_byte(instruction.mode(), bytes)?.unwrap();
                self.run_lda(input);
            },
            InstructionOperation::Nop => {},
            InstructionOperation::Sec => self.run_sec(),
            InstructionOperation::Sed => self.run_sed(),
            InstructionOperation::Sei => self.run_sei(),
            InstructionOperation::Tax => self.run_tax(),
            InstructionOperation::Tay => self.run_tay(),
            _ => unimplemented!(),
        }

        Ok(())
    }

    fn determine_input_byte(&self, mode: InstructionMode, bytes: &[u8]) -> Result<Option<u8>> {
        let input = match mode {
            InstructionMode::Implied => None,
            InstructionMode::Accumulator => return Err(anyhow!("invalid input byte mode: `Accumulator`")),
            InstructionMode::Immediate => Some(bytes[0]),
            InstructionMode::Relative => return Err(anyhow!("invalid input byte mode: `Relative`")),
            InstructionMode::ZeroPage => Some(self.determine_input_byte_from_address(mode, bytes)?),
            InstructionMode::ZeroPageX => Some(self.determine_input_byte_from_address(mode, bytes)?),
            InstructionMode::ZeroPageY => Some(self.determine_input_byte_from_address(mode, bytes)?),
            InstructionMode::Absolute => Some(self.determine_input_byte_from_address(mode, bytes)?),
            InstructionMode::AbsoluteX => Some(self.determine_input_byte_from_address(mode, bytes)?),
            InstructionMode::AbsoluteY => Some(self.determine_input_byte_from_address(mode, bytes)?),
            InstructionMode::Indirect => Some(self.determine_input_byte_from_address(mode, bytes)?),
            InstructionMode::IndirectX => Some(self.determine_input_byte_from_address(mode, bytes)?),
            InstructionMode::IndirectY => Some(self.determine_input_byte_from_address(mode, bytes)?),
        };

        Ok(input)
    }

    fn determine_input_byte_from_address(&self, mode: InstructionMode, bytes: &[u8]) -> Result<u8> {
        Ok(self.bus.read(self.resolve_address_by_mode(mode, bytes)?))
    }

    fn resolve_address_by_mode(&self, mode: InstructionMode, bytes: &[u8]) -> Result<Address> {
        match self.resolve_location_by_mode(mode, bytes)? {
            Some(location) => match location {
                Location::Address(address) => Ok(address),
                _ => Err(anyhow!("no address found in input location")),
            },
            None => Err(anyhow!("no input location found")),
        }
    }

    fn resolve_location_by_mode(&self, mode: InstructionMode, bytes: &[u8]) -> Result<Option<Location>> {
        let location = match mode {
            InstructionMode::Implied => None,
            InstructionMode::Accumulator => Some(Location::Accumulator),
            InstructionMode::Immediate => None,
            InstructionMode::Relative => {
                let offset = i32::from(bytes[0] as i8);
                let address = (self.registers.pc as i32).wrapping_add(offset) as Address;
                Some(Location::Address(address))
            },
            InstructionMode::ZeroPage => Some(Location::Address(bytes[0].into())),
            InstructionMode::ZeroPageX => {
                let address = (bytes[0] + self.registers.x) as Address;
                Some(Location::Address(address))
            },
            InstructionMode::ZeroPageY => {
                let address = (bytes[0] + self.registers.y) as Address;
                Some(Location::Address(address))
            },
            InstructionMode::Absolute => {
                let address = u16::from_le_bytes([bytes[0], bytes[1]]);
                Some(Location::Address(address))
            },
            InstructionMode::AbsoluteX => {
                // TODO: overflow check
                let address = u16::from_le_bytes([bytes[0], bytes[1]]);
                let address = address + self.registers.x as Address;
                Some(Location::Address(address))
            },
            InstructionMode::AbsoluteY => {
                // TODO: overflow check
                let address = u16::from_le_bytes([bytes[0], bytes[1]]);
                let address = address + self.registers.y as Address;
                Some(Location::Address(address))
            },
            InstructionMode::Indirect => {
                let address_first = u16::from_le_bytes([bytes[0], bytes[1]]);
                let address_second = self.bus.read_u16(address_first)?;
                Some(Location::Address(address_second))
            },
            InstructionMode::IndirectX => {
                let address_first = bytes[0].wrapping_add(self.registers.x);
                let address_second = self.bus.read_zp_u16(address_first)?;
                Some(Location::Address(address_second))
            },
            InstructionMode::IndirectY => {
                // TODO: overflow check
                let address_first = self.bus.read_zp_u16(bytes[0])?;
                let address_second = address_first + self.registers.y as Address;
                Some(Location::Address(address_second))
            },
        };

        Ok(location)
    }

    fn persist_result(&mut self, result: u8, location: Location) {
        match location {
            Location::Accumulator => self.registers.a = result,
            Location::Address(address) => self.bus.write(address, result),
        }
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

    fn run_and(&mut self, input: u8) {
        self.registers.a &= input;

        self.registers.p.set(StatusFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.a));
    }

    fn run_asl(&mut self, target: Location) {
        let input = match target {
            Location::Accumulator => self.registers.a,
            Location::Address(address) => self.bus.read(address),
        };
        let result = input.wrapping_shl(1);
        self.persist_result(result, target);

        self.registers.p.set(StatusFlags::CARRY, is_carry(input, result));
        self.registers.p.set(StatusFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(result));
    }

    fn run_branch(&mut self, target: Address) {
        self.registers.pc = target;

        // TODO: cycle calculation
    }

    fn run_clc(&mut self) {
        self.registers.p.remove(StatusFlags::CARRY);
    }

    fn run_cld(&mut self) {
        self.registers.p.remove(StatusFlags::DECIMAL);
    }

    fn run_cli(&mut self) {
        self.registers.p.remove(StatusFlags::INTERRUPT_DISABLE);
    }

    fn run_jmp(&mut self, target: Address) {
        self.registers.pc = target;
    }

    fn run_lda(&mut self, input: u8) {
        self.registers.a = input;

        self.registers.p.set(StatusFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.a));
    }

    fn run_sec(&mut self) {
        self.registers.p |= StatusFlags::CARRY;
    }

    fn run_sed(&mut self) {
        self.registers.p |= StatusFlags::DECIMAL;
    }

    fn run_sei(&mut self) {
        self.registers.p |= StatusFlags::INTERRUPT_DISABLE;
    }

    fn run_tax(&mut self) {
        self.registers.x = self.registers.a;

        self.registers.p.set(StatusFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.x));
    }

    fn run_tay(&mut self) {
        self.registers.y = self.registers.a;

        self.registers.p.set(StatusFlags::ZERO, self.registers.y == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.y));
    }

    fn run_tsx(&mut self) {
        self.registers.x = self.registers.s;

        self.registers.p.set(StatusFlags::ZERO, self.registers.x == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.x));
    }

    fn run_txa(&mut self) {
        self.registers.a = self.registers.x;

        self.registers.p.set(StatusFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.a));
    }

    fn run_txs(&mut self) {
        self.registers.s = self.registers.x;
    }

    fn run_tya(&mut self) {
        self.registers.a = self.registers.y;

        self.registers.p.set(StatusFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.a));
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
