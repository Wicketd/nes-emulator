#![cfg(test)]

use super::*;
use crate::cpu::opcodes::*;

const ADDRESS_PRG: Address = 0x8000;
// TODO: confusing because of indirect addressing; rename
const ADDRESS_INDIRECT: Address = 0x2040;
const ADDRESS_INDIRECT_HIGH: u8 = 0x20;
const ADDRESS_INDIRECT_LOW: u8 = 0x40;
const ADDRESS_INDIRECT_2: Address = 0x4080;
const ADDRESS_ZERO_PAGE: u8 = 0x30;
const OFFSET_REGISTER_X: u8 = 0x12;
const OFFSET_REGISTER_Y: u8 = 0x24;
const INPUT_BYTE: u8 = 0x10;

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

fn assert_input_byte_eq(mode: InstructionMode, expected: Option<u8>, bytes: &[u8], cpu: &Cpu) {
    assert_eq!(expected, cpu.determine_input_byte(mode, bytes).unwrap());
}

#[test]
fn determine_input_byte_implied() {
    let cpu = cpu(bus());
    assert_input_byte_eq(
        InstructionMode::Implied,
        None,
        &[0xFF, 0xFF, 0xFF],
        &cpu
    );
}

#[test]
fn determine_input_byte_immediate() {
    let cpu = cpu(bus());
    assert_input_byte_eq(
        InstructionMode::Immediate,
        Some(INPUT_BYTE),
        &[INPUT_BYTE, 0x20, 0x30],
        &cpu
    );
}

#[test]
fn determine_input_byte_absolute() {
    let mut bus = bus();
    bus.write(ADDRESS_INDIRECT, INPUT_BYTE);
    bus.write(ADDRESS_INDIRECT + OFFSET_REGISTER_X as Address, INPUT_BYTE);
    bus.write(ADDRESS_INDIRECT + OFFSET_REGISTER_Y as Address, INPUT_BYTE);

    let mut cpu = cpu(bus);
    cpu.registers.x = OFFSET_REGISTER_X;
    cpu.registers.y = OFFSET_REGISTER_Y;

    assert_input_byte_eq(
        InstructionMode::Absolute,
        Some(INPUT_BYTE),
        &ADDRESS_INDIRECT.to_le_bytes(),
        &cpu
    );

    assert_input_byte_eq(
        InstructionMode::AbsoluteX,
        Some(INPUT_BYTE),
        &ADDRESS_INDIRECT.to_le_bytes(),
        &cpu
    );

    assert_input_byte_eq(
        InstructionMode::AbsoluteY,
        Some(INPUT_BYTE),
        &ADDRESS_INDIRECT.to_le_bytes(),
        &cpu
    );
}

#[test]
fn determine_input_byte_zero_page() {
    let mut bus = bus();
    bus.write_zp(ADDRESS_ZERO_PAGE, INPUT_BYTE);
    bus.write_zp(ADDRESS_ZERO_PAGE + OFFSET_REGISTER_X, INPUT_BYTE);
    bus.write_zp(ADDRESS_ZERO_PAGE + OFFSET_REGISTER_Y, INPUT_BYTE);

    let mut cpu = cpu(bus);
    cpu.registers.x = OFFSET_REGISTER_X;
    cpu.registers.y = OFFSET_REGISTER_Y;

    assert_input_byte_eq(
        InstructionMode::ZeroPage,
        Some(INPUT_BYTE),
        &[ADDRESS_ZERO_PAGE],
        &cpu
    );

    assert_input_byte_eq(
        InstructionMode::ZeroPageX,
        Some(INPUT_BYTE),
        &[ADDRESS_ZERO_PAGE],
        &cpu
    );

    assert_input_byte_eq(
        InstructionMode::ZeroPageY,
        Some(INPUT_BYTE),
        &[ADDRESS_ZERO_PAGE],
        &cpu
    );
}

#[test]
fn determine_input_byte_indirect() {
    let mut bus = bus();
    bus.write_u16(ADDRESS_INDIRECT, ADDRESS_INDIRECT_2).unwrap();
    bus.write(ADDRESS_INDIRECT_2, INPUT_BYTE);

    let cpu = cpu(bus);
    assert_input_byte_eq(
        InstructionMode::Indirect,
        Some(INPUT_BYTE),
        &ADDRESS_INDIRECT.to_le_bytes(),
        &cpu
    );
}

#[test]
fn determine_input_byte_indirect_x() {
    let mut bus = bus();
    bus.write_zp_u16(ADDRESS_ZERO_PAGE + OFFSET_REGISTER_X, ADDRESS_INDIRECT).unwrap();
    bus.write(ADDRESS_INDIRECT, INPUT_BYTE);

    let mut cpu = cpu(bus);
    cpu.registers.x = OFFSET_REGISTER_X;

    assert_input_byte_eq(
        InstructionMode::IndirectX,
        Some(INPUT_BYTE),
        &[ADDRESS_ZERO_PAGE],
        &cpu
    );
}

#[test]
fn determine_input_byte_indirect_y() {
    let mut bus = bus();
    bus.write_zp_u16(ADDRESS_ZERO_PAGE, ADDRESS_INDIRECT).unwrap();
    bus.write(ADDRESS_INDIRECT + OFFSET_REGISTER_Y as Address, INPUT_BYTE);

    let mut cpu = cpu(bus);
    cpu.registers.y = OFFSET_REGISTER_Y;

    assert_input_byte_eq(
        InstructionMode::IndirectY,
        Some(INPUT_BYTE),
        &[ADDRESS_ZERO_PAGE],
        &cpu
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

#[test]
fn process_lda() {
    let mut bus = bus();
    bus.write(ADDRESS_INDIRECT, 0x10);
    bus.write(ADDRESS_INDIRECT + 2, 0x80);

    let mut cpu = cpu(bus);

    process_instruction(&mut cpu, &[LDA_ABSOLUTE, ADDRESS_INDIRECT_LOW, ADDRESS_INDIRECT_HIGH]);
    assert_eq!(cpu.registers.a, 0x10);
    assert_eq!(cpu.registers.p, StatusFlags::empty());

    process_instruction(&mut cpu, &[LDA_ABSOLUTE, ADDRESS_INDIRECT_LOW + 1, ADDRESS_INDIRECT_HIGH]);
    assert_eq!(cpu.registers.a, 0x00);
    assert_eq!(cpu.registers.p, StatusFlags::ZERO);

    process_instruction(&mut cpu,  &[LDA_ABSOLUTE, ADDRESS_INDIRECT_LOW + 2, ADDRESS_INDIRECT_HIGH]);
    assert_eq!(cpu.registers.a, 0x80);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE);
}

#[test]
fn process_jmp() {
    let mut bus = bus();
    bus.write_u16(ADDRESS_INDIRECT, ADDRESS_INDIRECT_2).unwrap();

    let mut cpu = cpu(bus);
    process_instruction(&mut cpu, &[JMP_INDIRECT, ADDRESS_INDIRECT_LOW, ADDRESS_INDIRECT_HIGH]);
    assert_eq!(cpu.registers.pc, ADDRESS_INDIRECT_2);
}
