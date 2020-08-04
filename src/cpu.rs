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
        if self.registers.pc + (instruction.len as Address) < ADDRESS_NMI {
            Ok(Some(instruction))
        } else {
            Ok(None)
        }
    }

    fn process_instruction(&mut self, instruction: Instruction) -> Result {
        // account for opcode
        self.registers.pc += 1;

        let instruction_len = instruction.len;
        self.run_instruction(instruction)?;
        self.registers.pc += (instruction_len as Address) - 1;

        Ok(())
    }

    fn run_instruction(&mut self, instruction: Instruction) -> Result {
        match instruction.operation {
            InstructionOperation::Adc => {
                self.run_adc(self.determine_input_byte(&instruction).unwrap());
            },
            InstructionOperation::And => unimplemented!("execute | And"),
            InstructionOperation::Asl => unimplemented!("execute | Asl"),
            InstructionOperation::Bcc => unimplemented!("execute | Bcc"),
            InstructionOperation::Bcs => unimplemented!("execute | Bcs"),
            InstructionOperation::Beq => unimplemented!("execute | Beq"),
            InstructionOperation::Bit => unimplemented!("execute | Bit"),
            InstructionOperation::Bmi => unimplemented!("execute | Bmi"),
            InstructionOperation::Bne => unimplemented!("execute | Bne"),
            InstructionOperation::Bpl => unimplemented!("execute | Bpl"),
            InstructionOperation::Brk => {
                // TODO
            },
            InstructionOperation::Bvc => unimplemented!("execute | Bvc"),
            InstructionOperation::Bvs => unimplemented!("execute | Bvs"),
            InstructionOperation::Clc => unimplemented!("execute | Clc"),
            InstructionOperation::Cld => unimplemented!("execute | Cld"),
            InstructionOperation::Cli => unimplemented!("execute | Cli"),
            InstructionOperation::Clv => unimplemented!("execute | Clv"),
            InstructionOperation::Cmp => unimplemented!("execute | Cmp"),
            InstructionOperation::Cpx => unimplemented!("execute | Cpx"),
            InstructionOperation::Cpy => unimplemented!("execute | Cpy"),
            InstructionOperation::Dec => unimplemented!("execute | Dec"),
            InstructionOperation::Dex => unimplemented!("execute | Dex"),
            InstructionOperation::Dey => unimplemented!("execute | Dey"),
            InstructionOperation::Eor => unimplemented!("execute | Eor"),
            InstructionOperation::Inc => unimplemented!("execute | Inc"),
            InstructionOperation::Inx => unimplemented!("execute | Inx"),
            InstructionOperation::Iny => unimplemented!("execute | Iny"),
            InstructionOperation::Jmp => unimplemented!("execute | Jmp"),
            InstructionOperation::Jsr => unimplemented!("execute | Jsr"),
            InstructionOperation::Lda => unimplemented!("execute | Lda"),
            InstructionOperation::Ldx => unimplemented!("execute | Ldx"),
            InstructionOperation::Ldy => unimplemented!("execute | Ldy"),
            InstructionOperation::Lsr => unimplemented!("execute | Lsr"),
            InstructionOperation::Nop => {},
            InstructionOperation::Ora => unimplemented!("execute | Ora"),
            InstructionOperation::Pha => unimplemented!("execute | Pha"),
            InstructionOperation::Php => unimplemented!("execute | Php"),
            InstructionOperation::Pla => unimplemented!("execute | Pla"),
            InstructionOperation::Plp => unimplemented!("execute | Plp"),
            InstructionOperation::Rol => unimplemented!("execute | Rol"),
            InstructionOperation::Ror => unimplemented!("execute | Ror"),
            InstructionOperation::Rti => unimplemented!("execute | Rti"),
            InstructionOperation::Rts => unimplemented!("execute | Rts"),
            InstructionOperation::Sbc => unimplemented!("execute | Sbc"),
            InstructionOperation::Sec => unimplemented!("execute | Sec"),
            InstructionOperation::Sed => unimplemented!("execute | Sed"),
            InstructionOperation::Sei => unimplemented!("execute | Sei"),
            InstructionOperation::Sta => unimplemented!("execute | Sta"),
            InstructionOperation::Stx => unimplemented!("execute | Stx"),
            InstructionOperation::Sty => unimplemented!("execute | Sty"),
            InstructionOperation::Tax => unimplemented!("execute | Tax"),
            InstructionOperation::Tay => unimplemented!("execute | Tay"),
            InstructionOperation::Tsx => unimplemented!("execute | Tsx"),
            InstructionOperation::Txa => unimplemented!("execute | Txa"),
            InstructionOperation::Txs => unimplemented!("execute | Txs"),
            InstructionOperation::Tya => unimplemented!("execute | Tya"),
        };

        Ok(())
    }

    fn determine_input_byte(&self, instruction: &Instruction) -> Option<u8> {
        match instruction.mode {
            InstructionMode::Implied => None,
            InstructionMode::Accumulator => unimplemented!("input byte | Accumulator"),
            InstructionMode::Immediate => Some(self.bus.read(self.registers.pc)),
            InstructionMode::Relative => unimplemented!("input byte | Relative"),
            InstructionMode::ZeroPage => unimplemented!("input byte | ZeroPage"),
            InstructionMode::ZeroPageX => unimplemented!("input byte | ZeroPageX"),
            InstructionMode::ZeroPageY => unimplemented!("input byte | ZeroPageY"),
            InstructionMode::Absolute => unimplemented!("input byte | Absolute"),
            InstructionMode::AbsoluteX => unimplemented!("input byte | AbsoluteX"),
            InstructionMode::AbsoluteY => unimplemented!("input byte | AbsoluteY"),
            InstructionMode::Indirect => unimplemented!("input byte | Indirect"),
            InstructionMode::IndirectX => unimplemented!("input byte | IndirectX"),
            InstructionMode::IndirectY => unimplemented!("input byte | IndirectY"),
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

struct Instruction {
    opcode: u8,
    operation: InstructionOperation,
    mode: InstructionMode,
    len: u8,
    cycles_base: u8,
}

macro_rules! match_opcode {
    (
        use $opcode_ident:ident;

        $($opcode:literal => (
            $operation:ident,
            $mode:ident,
            $len:literal,
            $cycles_base:literal
        ),)+
    ) => {
        match $opcode_ident {
            $($opcode => Instruction {
                opcode: $opcode,
                operation: InstructionOperation::$operation,
                mode: InstructionMode::$mode,
                len: $len,
                cycles_base: $cycles_base,
            },)+
            _ => unimplemented!("no instruction found for opcode `{}`", $opcode_ident),
        }
    };
}

impl Instruction {
    fn from_opcode(opcode: u8) -> Self {
        match_opcode! {
            use opcode;

            // opcode => (operation, mode, len, cycles_base)
            0x69 => (Adc, Immediate, 2, 2),
            0x65 => (Adc, ZeroPage,  2, 3),
            0x75 => (Adc, ZeroPage,  2, 4),
            0x6D => (Adc, Absolute,  3, 4),
            0x00 => (Brk, Implied,   1, 7),
            0x6C => (Jmp, Indirect,  3, 5),
            0xEA => (Nop, Implied,   1, 2),
            0x8D => (Sta, Absolute,  3, 4),
            0x96 => (Stx, ZeroPageY, 2, 4),
        }
    }
}

enum InstructionOperation {
    Adc, And, Asl, Bcc, Bcs, Beq, Bit, Bmi, Bne, Bpl, Brk, Bvc, Bvs, Clc,
    Cld, Cli, Clv, Cmp, Cpx, Cpy, Dec, Dex, Dey, Eor, Inc, Inx, Iny, Jmp,
    Jsr, Lda, Ldx, Ldy, Lsr, Nop, Ora, Pha, Php, Pla, Plp, Rol, Ror, Rti,
    Rts, Sbc, Sec, Sed, Sei, Sta, Stx, Sty, Tax, Tay, Tsx, Txa, Txs, Tya,
}

enum InstructionMode {
    Implied,
    Accumulator,
    Immediate,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
}

#[cfg(test)]
mod tests {
    use super::*;

    const ADDRESS_PRG: Address = 0x8000;

    fn bus() -> Bus {
        let mut bus = Bus::new();
        bus.write_u16(ADDRESS_RESET, ADDRESS_PRG).unwrap();
        bus
    }

    fn cpu(bus: Bus) -> Cpu {
        Cpu::new(bus).unwrap()
    }

    fn process_instruction(cpu: &mut Cpu, bytes: &[u8]) {
        for (i, byte) in bytes.iter().enumerate() {
            cpu.bus.write(cpu.registers.pc + (i as Address), *byte);
        }

        let instruction = cpu.determine_instruction_next().unwrap().unwrap();
        cpu.process_instruction(instruction).unwrap();
    }

    #[test]
    fn process_adc() {
        let bus = bus();
        let mut cpu = cpu(bus);

        process_instruction(&mut cpu, &[0x69, 0x10]);
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.registers.p, StatusFlags::empty());

        process_instruction(&mut cpu, &[0x69, 0x70]);
        assert_eq!(cpu.registers.a, 0x80);
        assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE | StatusFlags::OVERFLOW);

        process_instruction(&mut cpu, &[0x69, 0x80]);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.p, StatusFlags::OVERFLOW | StatusFlags::ZERO | StatusFlags::CARRY);

        process_instruction(&mut cpu, &[0x69, 0x10]);
        assert_eq!(cpu.registers.a, 0x11);
        assert_eq!(cpu.registers.p, StatusFlags::empty());
    }
}
