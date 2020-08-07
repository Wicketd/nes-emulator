mod clock;

use self::clock::{Clock, ClockMode};
use crate::bus::Bus;
use crate::types::Result;

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

        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    const ADDRESS_PRG: u16 = 0x8000;

    fn bus() -> Bus {
        let mut bus = Bus::new();
        bus.write_u16(ADDRESS_RESET, ADDRESS_PRG).unwrap();
        bus
    }

    fn cpu(bus: Bus) -> Cpu {
        Cpu::new(bus).unwrap()
    }

    #[test]
    fn stack_push_pull() {
        let mut cpu = cpu(bus());

        cpu.stack_push(0x10);
        cpu.stack_push(0x20);
        cpu.stack_push(0x30);
        assert_eq!(cpu.bus.read(0x01FD), 0x30);
        assert_eq!(cpu.registers.s, 0xFC);

        assert_eq!(cpu.stack_pull(), 0x30);
        assert_eq!(cpu.stack_pull(), 0x20);
        assert_eq!(cpu.bus.read(0x01FF), 0x10);
        assert_eq!(cpu.stack_pull(), 0x10);
        assert_eq!(cpu.bus.read(0x01FF), 0);
        assert_eq!(cpu.registers.s, 0xFF);
    }

    #[test]
    fn stack_overflow() {
        let mut cpu = cpu(bus());

        for i in (0..0xFF).rev() {
            cpu.stack_push(0x10);
            assert_eq!(cpu.registers.s, i);
        }

        cpu.stack_push(0x10);
        assert_eq!(cpu.registers.s, 0xFF);
    }

    #[test]
    fn stack_underflow() {
        let mut cpu = cpu(bus());
        assert_eq!(cpu.registers.s, 0xFF);
        cpu.stack_pull();
        assert_eq!(cpu.registers.s, 0x00);
    }
}
