#![cfg(test)]

use super::*;
use crate::cpu::opcodes::*;

const ADDRESS_PRG: Address = 0x8000;
const ADDRESS_INDIRECT: Address = 0x2040;

fn bus() -> Bus {
    let mut bus = Bus::new();
    bus.write_u16(ADDRESS_RESET, ADDRESS_PRG).unwrap();
    bus
}

fn cpu(bus: Bus) -> Cpu {
    let mut registers_expected = RegisterSet::new();
    registers_expected.pc = ADDRESS_PRG;

    let cpu = Cpu::new(bus).unwrap();
    assert_eq!(cpu.registers, registers_expected);
    cpu
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
fn determine_input_byte_implied() {
    let cpu = cpu(bus());
    assert_eq!(
        None,
        cpu.determine_input_byte(InstructionMode::Implied, &[0xFF, 0xFF, 0xFF]).unwrap()
    );
}

#[test]
fn determine_input_byte_immediate() {
    let cpu = cpu(bus());
    assert_eq!(
        Some(0x10),
        cpu.determine_input_byte(InstructionMode::Immediate, &[0x10, 0x20, 0x30]).unwrap()
    );
}

#[test]
fn determine_input_byte_absolute() {
    let mut bus = bus();
    bus.write(ADDRESS_INDIRECT, 0x10);

    let cpu = cpu(bus);
    assert_eq!(
        Some(0x10),
        cpu.determine_input_byte(InstructionMode::Absolute, &ADDRESS_INDIRECT.to_le_bytes()).unwrap()
    );
}

#[test]
fn process_adc() {
    let mut cpu = cpu(bus());

    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0x10]);
    assert_eq!(cpu.registers.a, 0x10);
    assert_eq!(cpu.registers.p, StatusFlags::empty());

    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0x70]);
    assert_eq!(cpu.registers.a, 0x80);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE | StatusFlags::OVERFLOW);

    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0x80]);
    assert_eq!(cpu.registers.a, 0x00);
    assert_eq!(cpu.registers.p, StatusFlags::OVERFLOW | StatusFlags::ZERO | StatusFlags::CARRY);

    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0x10]);
    assert_eq!(cpu.registers.a, 0x11);
    assert_eq!(cpu.registers.p, StatusFlags::empty());
}
