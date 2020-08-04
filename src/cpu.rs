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

    // TODO: find clean way to prevent unwrapping
    fn run_instruction(&mut self, instruction: Instruction) -> Result {
        match instruction.operation {
            InstructionOperation::Adc => {
                self.run_adc(self.determine_input_byte(instruction.mode)?.unwrap());
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
            InstructionOperation::Lda => {
                self.run_lda(self.determine_input_byte(instruction.mode)?.unwrap());
            },
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

    fn determine_input_byte(&self, instruction_mode: InstructionMode) -> Result<Option<u8>> {
        let input = match instruction_mode {
            InstructionMode::Implied => None,
            InstructionMode::Accumulator => unimplemented!("input byte | Accumulator"),
            InstructionMode::Immediate => {
                Some(self.bus.read(self.registers.pc))
            },
            InstructionMode::Relative => unimplemented!("input byte | Relative"),
            InstructionMode::ZeroPage => unimplemented!("input byte | ZeroPage"),
            InstructionMode::ZeroPageX => unimplemented!("input byte | ZeroPageX"),
            InstructionMode::ZeroPageY => unimplemented!("input byte | ZeroPageY"),
            InstructionMode::Absolute => {
                let address = self.bus.read_u16(self.registers.pc)?;
                Some(self.bus.read(address))
            },
            InstructionMode::AbsoluteX => unimplemented!("input byte | AbsoluteX"),
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

    fn run_and(&mut self, input: u8) {
        unimplemented!("run | and");
    }

    fn run_asl(&mut self, target: Location) {
        unimplemented!("run | asl");
    }

    fn run_bcc(&mut self, target: Address) {
        unimplemented!("run | bcc");
    }

    fn run_bcs(&mut self, target: Address) {
        unimplemented!("run | bcs");
    }

    fn run_beq(&mut self, target: Address) {
        unimplemented!("run | beq");
    }

    fn run_bit(&mut self, target: Address) {
        unimplemented!("run | bit");
    }

    fn run_bmi(&mut self, target: Address) {
        unimplemented!("run | bmi");
    }

    fn run_bne(&mut self, target: Location) {
        unimplemented!("run | bne");
    }

    fn run_bpl(&mut self, target: Location) {
        unimplemented!("run | bpl");
    }

    fn run_brk(&mut self) {
        unimplemented!("run | brk");
    }

    fn run_bvc(&mut self, target: Address) {
        unimplemented!("run | bvc");
    }

    fn run_bvs(&mut self, target: Address) {
        unimplemented!("run | bvs");
    }

    fn run_clc(&mut self) {
        unimplemented!("run | clc");
    }

    fn run_cld(&mut self) {
        unimplemented!("run | cld");
    }

    fn run_cli(&mut self) {
        unimplemented!("run | cli");
    }

    fn run_clv(&mut self) {
        unimplemented!("run | clv");
    }

    fn run_cmp(&mut self, input: u8) {
        unimplemented!("run | cmp");
    }

    fn run_cpx(&mut self, input: u8) {
        unimplemented!("run | cpx");
    }

    fn run_cpy(&mut self, input: u8) {
        unimplemented!("run | cpy");
    }

    fn run_dec(&mut self, target: Address) {
        unimplemented!("run | dec");
    }

    fn run_dex(&mut self) {
        unimplemented!("run | dex");
    }

    fn run_dey(&mut self) {
        unimplemented!("run | dey");
    }

    fn run_eor(&mut self, input: u8) {
        unimplemented!("run | eor");
    }

    fn run_inc(&mut self, target: Address) {
        unimplemented!("run | inc");
    }

    fn run_inx(&mut self) {
        unimplemented!("run | inx");
    }

    fn run_iny(&mut self) {
        unimplemented!("run | iny");
    }

    fn run_jmp(&mut self, target: Address) {
        unimplemented!("run | jmp");
    }

    fn run_jsr(&mut self, target: Address) {
        unimplemented!("run | jsr");
    }

    fn run_lda(&mut self, input: u8) {
        self.registers.a = input;

        self.registers.p.set(StatusFlags::ZERO, self.registers.a == 0);
        self.registers.p.set(StatusFlags::NEGATIVE, is_negative(self.registers.a));
    }

    fn run_ldx(&mut self, input: u8) {
        unimplemented!("run | ldx");
    }

    fn run_ldy(&mut self, input: u8) {
        unimplemented!("run | ldy");
    }

    fn run_lsr(&mut self, target: Location) {
        unimplemented!("run | lsr");
    }

    fn run_ora(&mut self, input: u8) {
        unimplemented!("run | ora");
    }

    fn run_pha(&mut self) {
        unimplemented!("run | pha");
    }

    fn run_php(&mut self) {
        unimplemented!("run | php");
    }

    fn run_pla(&mut self) {
        unimplemented!("run | pla");
    }

    fn run_plp(&mut self) {
        unimplemented!("run | plp");
    }

    fn run_rol(&mut self, target: Location) {
        unimplemented!("run | rol");
    }

    fn run_ror(&mut self, target: Location) {
        unimplemented!("run | ror");
    }

    fn run_rti(&mut self) {
        unimplemented!("run | rti");
    }

    fn run_rts(&mut self) {
        unimplemented!("run | rts");
    }

    fn run_sbc(&mut self, input: u8) {
        unimplemented!("run | sbc");
    }

    fn run_sec(&mut self) {
        unimplemented!("run | sec");
    }

    fn run_sed(&mut self) {
        unimplemented!("run | sed");
    }

    fn run_sei(&mut self) {
        unimplemented!("run | sei");
    }

    fn run_sta(&mut self, target: Address) {
        unimplemented!("run | sta");
    }

    fn run_stx(&mut self, target: Address) {
        unimplemented!("run | stx");
    }

    fn run_sty(&mut self, target: Address) {
        unimplemented!("run | sty");
    }

    fn run_tax(&mut self) {
        unimplemented!("run | tax");
    }

    fn run_tay(&mut self) {
        unimplemented!("run | tay");
    }

    fn run_tsx(&mut self) {
        unimplemented!("run | tsx");
    }

    fn run_txa(&mut self) {
        unimplemented!("run | txa");
    }

    fn run_txs(&mut self) {
        unimplemented!("run | txs");
    }

    fn run_tya(&mut self) {
        unimplemented!("run | tya");
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

enum Location {
    Accumulator,
    Address(Address),
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
            _ => unimplemented!("no instruction found for opcode `${:02X}`", $opcode_ident),
        }
    };
}

impl Instruction {
    fn from_opcode(opcode: u8) -> Self {
        match_opcode! {
            use opcode;

            // opcode => (operation, mode, len, cycles_base)
            0x69 => (Adc, Immediate, 2, 2),
            0xAD => (Lda, Absolute,  3, 4),
            0xEA => (Nop, Implied,   1, 2),
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
    const ADDRESS_INDIRECT: Address = 0x2000;
    const ADDRESS_INDIRECT_HIGH: u8 = 0x20;
    const ADDRESS_INDIRECT_LOW: u8 = 0x00;

    fn bus() -> Bus {
        let mut bus = Bus::new();
        bus.write_u16(ADDRESS_RESET, ADDRESS_PRG).unwrap();
        bus
    }

    fn cpu(bus: Bus) -> Cpu {
        Cpu::new(bus).unwrap()
    }

    fn write_instruction(cpu: &mut Cpu, bytes: &[u8]) {
        for (i, byte) in bytes.iter().enumerate() {
            cpu.bus.write(cpu.registers.pc + (i as Address), *byte);
        }
    }

    fn process_instruction(cpu: &mut Cpu, bytes: &[u8]) {
        write_instruction(cpu, bytes);
        let instruction = cpu.determine_instruction_next().unwrap().unwrap();
        cpu.process_instruction(instruction).unwrap();
    }

    #[test]
    fn determine_input_byte_immediate() {
        let mut bus = bus();
        bus.write(ADDRESS_PRG, 0xF4);

        let cpu = cpu(bus);
        let input = cpu.determine_input_byte(InstructionMode::Immediate).unwrap().unwrap();
        assert_eq!(input, 0xF4);
    }

    #[test]
    fn determine_input_byte_absolute() {
        let mut bus = bus();
        bus.write_u16(ADDRESS_PRG, ADDRESS_INDIRECT).unwrap();
        bus.write(ADDRESS_INDIRECT, 0xF4);

        let cpu = cpu(bus);
        let input = cpu.determine_input_byte(InstructionMode::Absolute).unwrap().unwrap();
        assert_eq!(input, 0xF4);
    }

    #[test]
    fn process_adc() {
        let mut cpu = cpu(bus());

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

    #[test]
    fn process_lda() {
        let mut bus = bus();
        bus.write(ADDRESS_INDIRECT, 0x10);
        bus.write(ADDRESS_INDIRECT + 2, 0x80);

        let mut cpu = cpu(bus);

        process_instruction(&mut cpu, &[0xAD, ADDRESS_INDIRECT_LOW, ADDRESS_INDIRECT_HIGH]);
        assert_eq!(cpu.registers.a, 0x10);
        assert_eq!(cpu.registers.p, StatusFlags::empty());

        process_instruction(&mut cpu, &[0xAD, ADDRESS_INDIRECT_LOW + 1, ADDRESS_INDIRECT_HIGH]);
        assert_eq!(cpu.registers.a, 0x00);
        assert_eq!(cpu.registers.p, StatusFlags::ZERO);

        process_instruction(&mut cpu,  &[0xAD, ADDRESS_INDIRECT_LOW + 2, ADDRESS_INDIRECT_HIGH]);
        assert_eq!(cpu.registers.a, 0x80);
        assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE);
    }
}
