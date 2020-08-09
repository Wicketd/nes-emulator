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

const ADDRESS_VECTOR_NMI: u16 = 0xFFFA;
const ADDRESS_VECTOR_RESET: u16 = 0xFFFC;
const ADDRESS_VECTOR_IRQ: u16 = 0xFFFE;

pub struct Cpu {
    bus: Bus,
    registers: RegisterSet,
    vectors: VectorSet,
    clock: Clock,
}

impl Cpu {
    pub fn new(bus: Bus) -> Result<Self> {
        let vectors = VectorSet {
            nmi: bus.read_u16(ADDRESS_VECTOR_NMI)?,
            reset: bus.read_u16(ADDRESS_VECTOR_RESET)?,
            irq: bus.read_u16(ADDRESS_VECTOR_IRQ)?,
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
        if (self.registers.pc + instruction.len() as u16) < ADDRESS_VECTOR_NMI {
            Some(instruction)
        } else {
            None
        }
    }

    fn process_instruction(&mut self, instruction: Instruction) -> Result {
        let len = instruction.len() as u16;
        let bytes = self.bus.read_n(self.registers.pc, len)?;

        // TODO: calculate final cycles
        self.clock.tick(instruction.cycles_base());
        self.call_instruction(instruction, &bytes)?;
        self.registers.pc += len;

        Ok(())
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
            InstructionOperation::Bpl => self.run_bpl(input.unwrap_address()?),
            InstructionOperation::Brk => self.run_brk(),
            InstructionOperation::Bvc => self.run_bvc(input.unwrap_address()?),
            InstructionOperation::Bvs => self.run_bvs(input.unwrap_address()?),
            InstructionOperation::Clc => self.run_clc(),
            InstructionOperation::Cld => self.run_cld(),
            InstructionOperation::Cli => self.run_cli(),
            InstructionOperation::Clv => self.run_clv(),
            InstructionOperation::Cmp => self.run_cmp(self.resolve_input_byte(input)?),
            InstructionOperation::Cpx => self.run_cpx(self.resolve_input_byte(input)?),
            InstructionOperation::Cpy => self.run_cpy(self.resolve_input_byte(input)?),
            InstructionOperation::Dec => self.run_dec(input.unwrap_address()?),
            InstructionOperation::Dex => self.run_dex(),
            InstructionOperation::Dey => self.run_dey(),
            InstructionOperation::Eor => self.run_eor(self.resolve_input_byte(input)?),
            InstructionOperation::Inc => self.run_inc(input.unwrap_address()?),
            InstructionOperation::Inx => self.run_inx(),
            InstructionOperation::Iny => self.run_iny(),
            InstructionOperation::Jmp => self.run_jmp(input.unwrap_address()?),
            InstructionOperation::Jsr => self.run_jsr(input.unwrap_address()?, instruction.len()),
            InstructionOperation::Lda => self.run_lda(self.resolve_input_byte(input)?),
            InstructionOperation::Ldx => self.run_ldx(self.resolve_input_byte(input)?),
            InstructionOperation::Ldy => self.run_ldy(self.resolve_input_byte(input)?),
            InstructionOperation::Lsr => self.run_lsr(input.unwrap_location()?),
            InstructionOperation::Nop => {},
            InstructionOperation::Ora => self.run_ora(self.resolve_input_byte(input)?),
            InstructionOperation::Pha => self.run_pha(),
            InstructionOperation::Php => self.run_php(),
            InstructionOperation::Pla => self.run_pla(),
            InstructionOperation::Plp => self.run_plp(),
            InstructionOperation::Rol => unimplemented!("call | Rol"),
            InstructionOperation::Ror => unimplemented!("call | Ror"),
            InstructionOperation::Rti => unimplemented!("call | Rti"),
            InstructionOperation::Rts => unimplemented!("call | Rts"),
            InstructionOperation::Sbc => unimplemented!("call | Sbc"),
            InstructionOperation::Sec => self.run_sec(),
            InstructionOperation::Sed => self.run_sed(),
            InstructionOperation::Sei => self.run_sei(),
            InstructionOperation::Sta => self.run_sta(input.unwrap_address()?),
            InstructionOperation::Stx => self.run_stx(input.unwrap_address()?),
            InstructionOperation::Sty => self.run_sty(input.unwrap_address()?),
            InstructionOperation::Tax => self.run_tax(),
            InstructionOperation::Tay => self.run_tay(),
            InstructionOperation::Tsx => self.run_tsx(),
            InstructionOperation::Txa => self.run_txa(),
            InstructionOperation::Txs => self.run_txs(),
            InstructionOperation::Tya => self.run_tya(),
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
        self.persist_result_by_location(result, target);

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

    fn run_bpl(&mut self, target: u16) {
        if !self.registers.p.contains(StatusFlags::NEGATIVE) {
            self.registers.pc = target;
        }
    }

    fn run_brk(&mut self) {
        if !self.registers.p.contains(StatusFlags::INTERRUPT_DISABLE) {
            self.generate_interrupt(BreakType::Program);

            // TODO: hacky, find better way to account for instruction length being added
            self.registers.pc -= 1;
        }
    }

    fn run_bvc(&mut self, target: u16) {
        if !self.registers.p.contains(StatusFlags::OVERFLOW) {
            self.registers.pc = target;
        }
    }

    fn run_bvs(&mut self, target: u16) {
        if self.registers.p.contains(StatusFlags::OVERFLOW) {
            self.registers.pc = target;
        }
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

    fn run_clv(&mut self) {
        self.registers.p.remove(StatusFlags::OVERFLOW);
    }

    fn run_cmp(&mut self, input: u8) {
        let result = self.registers.a.wrapping_sub(input);
        self.registers.p.set(StatusFlags::CARRY, self.registers.a >= input);
        self.registers.p.set(StatusFlags::ZERO, self.registers.a == input);
        self.registers.p.set(StatusFlags::NEGATIVE, result.is_bit_set(7));
    }

    fn run_cpx(&mut self, input: u8) {
        let result = self.registers.x.wrapping_sub(input);
        self.registers.p.set(StatusFlags::CARRY, self.registers.x >= input);
        self.registers.p.set(StatusFlags::ZERO, self.registers.x == input);
        self.registers.p.set(StatusFlags::NEGATIVE, result.is_bit_set(7));
    }

    fn run_cpy(&mut self, input: u8) {
        let result = self.registers.y.wrapping_sub(input);
        self.registers.p.set(StatusFlags::CARRY, self.registers.y >= input);
        self.registers.p.set(StatusFlags::ZERO, self.registers.y == input);
        self.registers.p.set(StatusFlags::NEGATIVE, result.is_bit_set(7));
    }

    fn run_dec(&mut self, target: u16) {
        let result = self.bus.read(target).wrapping_sub(1);
        self.bus.write(target, result);
        self.set_status_flag_zero(result);
        self.set_status_flag_negative(result);
    }

    fn run_dex(&mut self) {
        self.registers.x = self.registers.x.wrapping_sub(1);
        self.set_status_flag_zero(self.registers.x);
        self.set_status_flag_negative(self.registers.x);
    }

    fn run_dey(&mut self) {
        self.registers.y = self.registers.y.wrapping_sub(1);
        self.set_status_flag_zero(self.registers.y);
        self.set_status_flag_negative(self.registers.y);
    }

    fn run_eor(&mut self, input: u8) {
        self.registers.a ^= input;
        self.set_status_flag_zero(self.registers.a);
        self.set_status_flag_negative(self.registers.a);
    }

    fn run_inc(&mut self, target: u16) {
        let result = self.bus.read(target).wrapping_add(1);
        self.bus.write(target, result);
        self.set_status_flag_zero(result);
        self.set_status_flag_negative(result);
    }

    fn run_inx(&mut self) {
        self.registers.x = self.registers.x.wrapping_add(1);
        self.set_status_flag_zero(self.registers.x);
        self.set_status_flag_negative(self.registers.x);
    }

    fn run_iny(&mut self) {
        self.registers.y = self.registers.y.wrapping_add(1);
        self.set_status_flag_zero(self.registers.y);
        self.set_status_flag_negative(self.registers.y);
    }

    fn run_jmp(&mut self, target: u16) {
        // TODO: hacky, find better way to account for instruction length being added
        self.registers.pc = target - 3;
    }

    fn run_jsr(&mut self, target: u16, instruction_len: u8) {
        let stack_address = self.registers.pc.wrapping_add(instruction_len as u16).wrapping_sub(1);
        self.stack_push_u16(stack_address);

        // TODO: hacky, find better way to account for instruction length being added
        self.registers.pc = target - instruction_len as u16;
    }

    fn run_lda(&mut self, input: u8) {
        self.registers.a = input;
        self.set_status_flag_zero(input);
        self.set_status_flag_negative(input);
    }

    fn run_ldx(&mut self, input: u8) {
        self.registers.x = input;
        self.set_status_flag_zero(input);
        self.set_status_flag_negative(input);
    }

    fn run_ldy(&mut self, input: u8) {
        self.registers.y = input;
        self.set_status_flag_zero(input);
        self.set_status_flag_negative(input);
    }

    fn run_lsr(&mut self, target: InstructionInputLocation) {
        let input = match target {
            InstructionInputLocation::Accumulator => self.registers.a,
            InstructionInputLocation::Address(address) => self.bus.read(address),
        };
        let result = input.wrapping_shr(1);
        self.persist_result_by_location(result, target);

        self.registers.p.set(StatusFlags::CARRY, input.is_bit_set(0));
        self.set_status_flag_zero(result);

        // TODO: is this correct? bit 7 seems to never be set
        self.registers.p.remove(StatusFlags::NEGATIVE);
    }

    fn run_ora(&mut self, input: u8) {
        self.registers.a |= input;
        self.set_status_flag_zero(self.registers.a);
        self.set_status_flag_negative(self.registers.a);
    }

    fn run_pha(&mut self) {
        self.stack_push(self.registers.a);
    }

    fn run_php(&mut self) {
        self.stack_push(self.registers.p.bits());
    }

    fn run_pla(&mut self) {
        self.registers.a = self.stack_pull();
        self.set_status_flag_zero(self.registers.a);
        self.set_status_flag_negative(self.registers.a);
    }

    fn run_plp(&mut self) {
        self.registers.p = StatusFlags::from_bits(self.stack_pull()).unwrap();
    }

    fn run_sec(&mut self) {
        self.registers.p.insert(StatusFlags::CARRY);
    }

    fn run_sed(&mut self) {
        self.registers.p.insert(StatusFlags::DECIMAL);
    }

    fn run_sei(&mut self) {
        self.registers.p.insert(StatusFlags::INTERRUPT_DISABLE);
    }

    fn run_sta(&mut self, target: u16) {
        self.bus.write(target, self.registers.a);
    }

    fn run_stx(&mut self, target: u16) {
        self.bus.write(target, self.registers.x);
    }

    fn run_sty(&mut self, target: u16) {
        self.bus.write(target, self.registers.y);
    }

    fn run_tax(&mut self) {
        self.registers.x = self.registers.a;
        self.set_status_flag_zero(self.registers.x);
        self.set_status_flag_negative(self.registers.x);
    }

    fn run_tay(&mut self) {
        self.registers.y = self.registers.a;
        self.set_status_flag_zero(self.registers.y);
        self.set_status_flag_negative(self.registers.y);
    }

    fn run_tsx(&mut self) {
        self.registers.x = self.registers.s;
        self.set_status_flag_zero(self.registers.x);
        self.set_status_flag_negative(self.registers.x);
    }

    fn run_txa(&mut self) {
        self.registers.a = self.registers.x;
        self.set_status_flag_zero(self.registers.a);
        self.set_status_flag_negative(self.registers.a);
    }

    fn run_txs(&mut self) {
        self.registers.s = self.registers.x;
    }

    fn run_tya(&mut self) {
        self.registers.a = self.registers.y;
        self.set_status_flag_zero(self.registers.a);
        self.set_status_flag_negative(self.registers.a);
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

    fn persist_result_by_location(&mut self, result: u8, target: InstructionInputLocation) {
        match target {
            InstructionInputLocation::Accumulator => self.registers.a = result,
            InstructionInputLocation::Address(address) => self.bus.write(address, result),
        }
    }

    fn stack_push(&mut self, value: u8) {
        self.bus.write(self.stack_determine_address(), value);
        self.registers.s = self.registers.s.wrapping_sub(1);
    }

    fn stack_push_u16(&mut self, value: u16) {
        let bytes = value.to_le_bytes();
        self.stack_push(bytes[0]);
        self.stack_push(bytes[1]);
    }

    fn stack_pull(&mut self) -> u8 {
        let address = self.stack_determine_address().wrapping_add(1);
        let value = self.bus.read(address);
        self.bus.write(address, 0);
        self.registers.s = self.registers.s.wrapping_add(1);
        value
    }

    fn stack_pull_u16(&mut self) -> u16 {
        let mut bytes = [self.stack_pull(), self.stack_pull()];
        bytes.reverse();
        u16::from_le_bytes(bytes)
    }

    fn stack_determine_address(&self) -> u16 {
        0x0100 + self.registers.s as u16
    }

    // TODO: unit test separately?
    fn generate_interrupt(&mut self, break_type: BreakType) {
        self.stack_push_u16(self.registers.pc);
        self.stack_push(self.registers.p.bits());
        self.registers.pc = self.vectors.irq;
        self.registers.p.set_break(break_type);
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
            p: StatusFlags::default(),
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

impl StatusFlags {
    pub fn set_break(&mut self, break_type: BreakType) {
        match break_type {
            BreakType::Internal => {
                self.insert(StatusFlags::BREAK_LEFT);
                self.remove(StatusFlags::BREAK_RIGHT);
            },
            BreakType::Program => {
                self.insert(StatusFlags::BREAK_LEFT);
                self.insert(StatusFlags::BREAK_RIGHT);
            },
        }
    }
}

impl Default for StatusFlags {
    fn default() -> Self {
        let mut flags = Self::empty();
        flags |= StatusFlags::INTERRUPT_DISABLE;
        flags
    }
}

enum BreakType {
    Internal,
    Program,
}
