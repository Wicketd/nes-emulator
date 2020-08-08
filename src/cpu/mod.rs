mod clock;
mod instruction;
mod tests;

use self::clock::{Clock, ClockMode};
use self::instruction::{
    Instruction,
    InstructionOperation,
    InstructionMode,
    InstructionInput,
    InstructionInputLocation,
};
use crate::bus::Bus;
use crate::types::{Result, BitRead};

const ADDRESS_NMI: u16 = 0xFFFA;
const ADDRESS_RESET: u16 = 0xFFFC;
const ADDRESS_IRQ: u16 = 0xFFFE;

pub struct Cpu {
    bus: Bus,
    registers: RegisterSet,
    vectors: VectorSet,
    clock: Clock,
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

        // TODO: hard-coded
        let clock = Clock::new(ClockMode::Ntsc);

        Ok(Self { bus, registers, vectors, clock })
    }

    pub fn start(&mut self) -> Result {
        while let Some(instruction) = self.determine_instruction_next() {
            self.process_instruction(instruction)?;
        }

        Ok(())
    }

    fn determine_instruction_next(&self) -> Option<Instruction> {
        let opcode = self.bus.read(self.registers.pc);
        let instruction = Instruction::from_opcode(opcode);

        // TODO: check if correct
        if (self.registers.pc + instruction.len() as u16) < ADDRESS_NMI {
            Some(instruction)
        } else {
            None
        }
    }

    fn process_instruction(&mut self, instruction: Instruction) -> Result {
        let len = instruction.len() as u16;
        let bytes = self.bus.read_n(self.registers.pc, len)?;
        self.registers.pc += len;

        // TODO: calculate final cycles
        self.clock.tick(instruction.cycles_base());

        self.call_instruction(instruction, &bytes)
    }

    fn call_instruction(&mut self, instruction: Instruction, bytes: &[u8]) -> Result {
        let input = self.determine_input(instruction.mode(), bytes)?;

        match instruction.operation() {
            InstructionOperation::Adc => self.run_adc(self.resolve_input_byte(input)?),
            InstructionOperation::And => self.run_and(self.resolve_input_byte(input)?),
            InstructionOperation::Asl => self.run_asl(input.unwrap_location()?),
            InstructionOperation::Bcc => self.run_bcc(input.unwrap_address()?),
            InstructionOperation::Bcs => self.run_bcs(input.unwrap_address()?),
            InstructionOperation::Beq => self.run_beq(input.unwrap_address()?),
            InstructionOperation::Bit => self.run_bit(self.resolve_input_byte(input)?),
            InstructionOperation::Bmi => self.run_bmi(input.unwrap_address()?),
            InstructionOperation::Bne => self.run_bne(input.unwrap_address()?),
            InstructionOperation::Bpl => unimplemented!("call | Bpl"),
            InstructionOperation::Brk => self.run_brk(),
            InstructionOperation::Bvc => unimplemented!("call | Bvc"),
            InstructionOperation::Bvs => unimplemented!("call | Bvs"),
            InstructionOperation::Clc => unimplemented!("call | Clc"),
            InstructionOperation::Cld => unimplemented!("call | Cld"),
            InstructionOperation::Cli => unimplemented!("call | Cli"),
            InstructionOperation::Clv => unimplemented!("call | Clv"),
            InstructionOperation::Cmp => unimplemented!("call | Cmp"),
            InstructionOperation::Cpx => unimplemented!("call | Cpx"),
            InstructionOperation::Cpy => unimplemented!("call | Cpy"),
            InstructionOperation::Dec => unimplemented!("call | Dec"),
            InstructionOperation::Dex => unimplemented!("call | Dex"),
            InstructionOperation::Dey => unimplemented!("call | Dey"),
            InstructionOperation::Eor => unimplemented!("call | Eor"),
            InstructionOperation::Inc => unimplemented!("call | Inc"),
            InstructionOperation::Inx => unimplemented!("call | Inx"),
            InstructionOperation::Iny => unimplemented!("call | Iny"),
            InstructionOperation::Jmp => unimplemented!("call | Jmp"),
            InstructionOperation::Jsr => unimplemented!("call | Jsr"),
            InstructionOperation::Lda => self.run_lda(self.resolve_input_byte(input)?),
            InstructionOperation::Ldx => unimplemented!("call | Ldx"),
            InstructionOperation::Ldy => unimplemented!("call | Ldy"),
            InstructionOperation::Lsr => unimplemented!("call | Lsr"),
            InstructionOperation::Nop => {},
            InstructionOperation::Ora => unimplemented!("call | Ora"),
            InstructionOperation::Pha => self.run_pha(),
            InstructionOperation::Php => self.run_php(),
            InstructionOperation::Pla => unimplemented!("call | Pla"),
            InstructionOperation::Plp => unimplemented!("call | Plp"),
            InstructionOperation::Rol => unimplemented!("call | Rol"),
            InstructionOperation::Ror => unimplemented!("call | Ror"),
            InstructionOperation::Rti => unimplemented!("call | Rti"),
            InstructionOperation::Rts => unimplemented!("call | Rts"),
            InstructionOperation::Sbc => unimplemented!("call | Sbc"),
            InstructionOperation::Sec => unimplemented!("call | Sec"),
            InstructionOperation::Sed => unimplemented!("call | Sed"),
            InstructionOperation::Sei => unimplemented!("call | Sei"),
            InstructionOperation::Sta => unimplemented!("call | Sta"),
            InstructionOperation::Stx => unimplemented!("call | Stx"),
            InstructionOperation::Sty => unimplemented!("call | Sty"),
            InstructionOperation::Tax => unimplemented!("call | Tax"),
            InstructionOperation::Tay => unimplemented!("call | Tay"),
            InstructionOperation::Tsx => unimplemented!("call | Tsx"),
            InstructionOperation::Txa => unimplemented!("call | Txa"),
            InstructionOperation::Txs => unimplemented!("call | Txs"),
            InstructionOperation::Tya => unimplemented!("call | Tya"),
        }

        Ok(())
    }

    fn determine_input(&self, mode: InstructionMode, bytes: &[u8]) -> Result<InstructionInput> {
        let input = match mode {
            InstructionMode::Implied => InstructionInput::Implied,
            InstructionMode::Accumulator => {
                InstructionInput::Location(InstructionInputLocation::Accumulator)
            },
            InstructionMode::Immediate => {
                Self::assert_input_len(2, bytes);
                InstructionInput::Byte(bytes[1])
            },
            InstructionMode::Relative => {
                Self::assert_input_len(2, bytes);
                let offset = i32::from(bytes[1] as i8);
                let address = (self.registers.pc as i32).wrapping_add(offset) as u16;
                InstructionInput::from_address(address)
            },
            InstructionMode::ZeroPage => {
                Self::assert_input_len(2, bytes);
                InstructionInput::from_address(bytes[1] as u16)
            },
            InstructionMode::ZeroPageX => {
                Self::assert_input_len(2, bytes);
                let address = bytes[1].wrapping_add(self.registers.x) as u16;
                InstructionInput::from_address(address)
            },
            InstructionMode::ZeroPageY => {
                Self::assert_input_len(2, bytes);
                let address = bytes[1].wrapping_add(self.registers.y) as u16;
                InstructionInput::from_address(address)
            },
            InstructionMode::Absolute => {
                Self::assert_input_len(3, bytes);
                let address = u16::from_le_bytes([bytes[1], bytes[2]]);
                InstructionInput::from_address(address)
            },
            InstructionMode::AbsoluteX => {
                let input = self.determine_input(InstructionMode::Absolute, bytes)?;
                let address = input.unwrap_address()?.wrapping_add(self.registers.x as u16);
                InstructionInput::from_address(address)
            },
            InstructionMode::AbsoluteY => {
                let input = self.determine_input(InstructionMode::Absolute, bytes)?;
                let address = input.unwrap_address()?.wrapping_add(self.registers.y as u16);
                InstructionInput::from_address(address)
            },
            InstructionMode::Indirect => {
                Self::assert_input_len(3, bytes);
                let address_indirect = u16::from_le_bytes([bytes[1], bytes[2]]);
                let address = self.bus.read_u16(address_indirect)?;
                InstructionInput::from_address(address)
            },
            InstructionMode::IndirectX => {
                Self::assert_input_len(2, bytes);
                let address_indirect = bytes[1].wrapping_add(self.registers.x) as u16;
                let address = self.bus.read_u16(address_indirect)?;
                InstructionInput::from_address(address)
            },
            InstructionMode::IndirectY => {
                Self::assert_input_len(2, bytes);
                let address_indirect = bytes[1].wrapping_add(self.registers.y) as u16;
                let address = self.bus.read_u16(address_indirect)?;
                InstructionInput::from_address(address)
            },
        };

        Ok(input)
    }

    fn resolve_input_byte(&self, input: InstructionInput) -> Result<u8> {
        let value = match input {
            InstructionInput::Byte(value) => value,
            InstructionInput::Location(location) => match location {
                InstructionInputLocation::Address(address) => self.bus.read(address),
                _ => return Err(anyhow!("cannot resolve input byte for the current variant")),
            },
            _ => return Err(anyhow!("cannot resolve input byte for the current variant")),
        };

        Ok(value)
    }

    fn assert_input_len(len_expected: usize, bytes: &[u8]) {
        assert!(len_expected == bytes.len(), "expected args to have length `{}`, received `{}`", len_expected, bytes.len());
    }

    fn run_adc(&mut self, input: u8) {
        let a_old = self.registers.a;
        let carry = (self.registers.p & StatusFlags::CARRY).bits();
        let result = self.registers.a.wrapping_add(input).wrapping_add(carry);
        self.registers.a = result;
        self.set_status_flag_carry(input, result);
        self.set_status_flag_zero(result);
        self.set_status_flag_overflow(a_old, result);
        self.set_status_flag_negative(result);
    }

    fn run_and(&mut self, input: u8) {
        self.registers.a &= input;
        self.set_status_flag_zero(self.registers.a);
        self.set_status_flag_negative(self.registers.a);
    }

    fn run_asl(&mut self, target: InstructionInputLocation) {
        let input = match target {
            InstructionInputLocation::Accumulator => self.registers.a,
            InstructionInputLocation::Address(address) => self.bus.read(address),
        };
        let result = input.wrapping_shl(1);
        self.persist_result(result, target);

        self.set_status_flag_carry(input, result);
        self.set_status_flag_zero(result);
        self.set_status_flag_negative(result);
    }

    fn run_bcc(&mut self, target: u16) {
        if !self.registers.p.contains(StatusFlags::CARRY) {
            self.registers.pc = target;
        }
    }

    fn run_bcs(&mut self, target: u16) {
        if self.registers.p.contains(StatusFlags::CARRY) {
            self.registers.pc = target;
        }
    }

    fn run_beq(&mut self, target: u16) {
        if self.registers.p.contains(StatusFlags::ZERO) {
            self.registers.pc = target;
        }
    }

    fn run_bit(&mut self, input: u8) {
        self.set_status_flag_zero(self.registers.a & input);
        self.registers.p.set(StatusFlags::OVERFLOW, input.is_bit_set(6));
        self.registers.p.set(StatusFlags::NEGATIVE, input.is_bit_set(7));
    }

    fn run_brk(&self) {
        // TODO: implement
    }

    fn run_bmi(&mut self, target: u16) {
        if self.registers.p.contains(StatusFlags::NEGATIVE) {
            self.registers.pc = target;
        }
    }

    fn run_bne(&mut self, target: u16) {
        if !self.registers.p.contains(StatusFlags::ZERO) {
            self.registers.pc = target;
        }
    }

    fn run_lda(&mut self, input: u8) {
        self.registers.a = input;
        self.set_status_flag_zero(input);
        self.set_status_flag_negative(input);
    }

    fn run_pha(&mut self) {
        self.stack_push(self.registers.a);
    }

    fn run_php(&mut self) {
        self.stack_push(self.registers.p.bits());
    }

    fn set_status_flag_carry(&mut self, input: u8, result: u8) {
        self.registers.p.set(StatusFlags::CARRY, result < input);
    }

    fn set_status_flag_zero(&mut self, value: u8) {
        self.registers.p.set(StatusFlags::ZERO, value == 0);
    }

    fn set_status_flag_overflow(&mut self, value_old: u8, value_new: u8) {
        self.registers.p.set(StatusFlags::OVERFLOW, value_old.read_bit(7) != value_new.read_bit(7));
    }

    fn set_status_flag_negative(&mut self, value: u8) {
        self.registers.p.set(StatusFlags::NEGATIVE, value.is_bit_set(7));
    }

    fn persist_result(&mut self, result: u8, target: InstructionInputLocation) {
        match target {
            InstructionInputLocation::Accumulator => self.registers.a = result,
            InstructionInputLocation::Address(address) => self.bus.write(address, result),
        }
    }

    fn stack_push(&mut self, value: u8) {
        self.bus.write(self.stack_determine_address(), value);
        self.registers.s = self.registers.s.wrapping_sub(1);
    }

    fn stack_pull(&mut self) -> u8 {
        let address = self.stack_determine_address().wrapping_add(1);
        let value = self.bus.read(address);
        self.bus.write(address, 0);
        self.registers.s = self.registers.s.wrapping_add(1);
        value
    }

    fn stack_determine_address(&self) -> u16 {
        0x0100 + self.registers.s as u16
    }
}

#[derive(Debug, Eq, PartialEq)]
struct RegisterSet {
    a: u8,
    x: u8,
    y: u8,
    s: u8,
    p: StatusFlags,
    pc: u16,
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
    nmi: u16,
    reset: u16,
    irq: u16,
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
