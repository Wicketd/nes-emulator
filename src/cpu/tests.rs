#![cfg(test)]

use super::*;
use crate::cpu::opcodes::*;

const ADDRESS_PRG: Address = 0x8000;
const ADDRESS_ZERO_PAGE: Address = 0x0040;
const OFFSET_REGISTER_X: u8 = 0x20;
const ADDRESS_INDIRECT: Address = 0x2000;
const ADDRESS_INDIRECT_HIGH: u8 = 0x20;
const ADDRESS_INDIRECT_LOW: u8 = 0x00;

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
fn determine_input_location_accumulator() {
    let cpu = cpu(bus());
    let input = cpu.determine_input_location(InstructionMode::Accumulator).unwrap();
    assert_eq!(input, Location::Accumulator);
}

#[test]
// TODO: set X with LDX
fn determine_input_location_zero_page_x() {
    let mut bus = bus();
    bus.write_u16(ADDRESS_PRG, ADDRESS_ZERO_PAGE as Address).unwrap();

    let mut cpu = cpu(bus);
    cpu.registers.x = OFFSET_REGISTER_X;

    let input = cpu.determine_input_location(InstructionMode::ZeroPageX).unwrap();
    assert_eq!(input, Location::Address(ADDRESS_ZERO_PAGE + OFFSET_REGISTER_X as Address));
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
fn process_asl_accumulator() {
    let mut cpu = cpu(bus());

    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0b0100_0000]);
    assert_eq!(cpu.registers.a, 0b0100_0000);

    process_instruction(&mut cpu, &[ASL_ACCUMULATOR]);
    assert_eq!(cpu.registers.a, 0b1000_0000);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE);

    process_instruction(&mut cpu, &[ASL_ACCUMULATOR]);
    assert_eq!(cpu.registers.a, 0b0000_0000);
    assert_eq!(cpu.registers.p, StatusFlags::ZERO | StatusFlags::CARRY);
}

#[test]
// TODO: set X with LDX
// TODO: is this test necessary?
fn process_asl_zero_page_x() {
    let mut bus = bus();
    let address = ADDRESS_ZERO_PAGE + (OFFSET_REGISTER_X as Address);
    bus.write(address, 0b0100_0000);

    let mut cpu = cpu(bus);
    cpu.registers.x = OFFSET_REGISTER_X;

    process_instruction(&mut cpu, &[ASL_ZERO_PAGE_X, ADDRESS_ZERO_PAGE as u8]);
    assert_eq!(cpu.bus.read(address), 0b1000_0000);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE | StatusFlags::ZERO);

    process_instruction(&mut cpu, &[ASL_ZERO_PAGE_X, ADDRESS_ZERO_PAGE as u8]);
    assert_eq!(cpu.bus.read(address), 0b0000_0000);
    assert_eq!(cpu.registers.p, StatusFlags::ZERO | StatusFlags::CARRY);
}

#[test]
fn process_clc() {
    let mut cpu = cpu(bus());

    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0xFF]);
    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0x02]);
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.registers.p, StatusFlags::OVERFLOW | StatusFlags::CARRY);

    process_instruction(&mut cpu, &[CLC_IMPLIED]);
    assert_eq!(cpu.registers.p, StatusFlags::OVERFLOW);
}

#[test]
fn process_cld() {
    let mut cpu = cpu(bus());

    process_instruction(&mut cpu, &[SED_IMPLIED]);
    assert_eq!(cpu.registers.p, StatusFlags::DECIMAL);

    process_instruction(&mut cpu, &[CLD_IMPLIED]);
    assert_eq!(cpu.registers.p, StatusFlags::empty());
}

#[test]
fn process_cli() {
    let mut cpu = cpu(bus());

    process_instruction(&mut cpu, &[SEI_IMPLIED]);
    assert_eq!(cpu.registers.p, StatusFlags::INTERRUPT_DISABLE);

    process_instruction(&mut cpu, &[CLI_IMPLIED]);
    assert_eq!(cpu.registers.p, StatusFlags::empty());
}

#[test]
fn process_clv() {
    let mut cpu = cpu(bus());

    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0xFF]);
    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0x02]);
    assert_eq!(cpu.registers.a, 0x01);
    assert_eq!(cpu.registers.p, StatusFlags::OVERFLOW | StatusFlags::CARRY);

    process_instruction(&mut cpu, &[CLV_IMPLIED]);
    assert_eq!(cpu.registers.p, StatusFlags::CARRY);
}

#[test]
// TODO: set X with LDX
fn process_inx() {
    let mut cpu = cpu(bus());
    cpu.registers.x = 0x7E;

    process_instruction(&mut cpu, &[INX_IMPLIED]);
    assert_eq!(cpu.registers.x, 0x7F);
    assert_eq!(cpu.registers.p, StatusFlags::empty());

    process_instruction(&mut cpu, &[INX_IMPLIED]);
    assert_eq!(cpu.registers.x, 0x80);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE);

    cpu.registers.x = 0xFF;
    process_instruction(&mut cpu, &[INX_IMPLIED]);
    assert_eq!(cpu.registers.x, 0x00);
    assert_eq!(cpu.registers.p, StatusFlags::ZERO);
}

#[test]
// TODO: set Y with LDY
fn process_iny() {
    let mut cpu = cpu(bus());
    cpu.registers.y = 0x7E;

    process_instruction(&mut cpu, &[INY_IMPLIED]);
    assert_eq!(cpu.registers.y, 0x7F);
    assert_eq!(cpu.registers.p, StatusFlags::empty());

    process_instruction(&mut cpu, &[INY_IMPLIED]);
    assert_eq!(cpu.registers.y, 0x80);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE);

    cpu.registers.y = 0xFF;
    process_instruction(&mut cpu, &[INY_IMPLIED]);
    assert_eq!(cpu.registers.y, 0x00);
    assert_eq!(cpu.registers.p, StatusFlags::ZERO);
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
fn process_ldx() {
    let mut cpu = cpu(bus());

    process_instruction(&mut cpu, &[LDX_IMMEDIATE, 0x10]);
    assert_eq!(cpu.registers.x, 0x10);
    assert_eq!(cpu.registers.p, StatusFlags::empty());

    process_instruction(&mut cpu, &[LDX_IMMEDIATE, 0x00]);
    assert_eq!(cpu.registers.x, 0x00);
    assert_eq!(cpu.registers.p, StatusFlags::ZERO);

    process_instruction(&mut cpu, &[LDX_IMMEDIATE, 0xFF]);
    assert_eq!(cpu.registers.x, 0xFF);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE);
}

#[test]
fn process_ldy() {
    let mut cpu = cpu(bus());

    process_instruction(&mut cpu, &[LDY_IMMEDIATE, 0x10]);
    assert_eq!(cpu.registers.y, 0x10);
    assert_eq!(cpu.registers.p, StatusFlags::empty());

    process_instruction(&mut cpu, &[LDY_IMMEDIATE, 0x00]);
    assert_eq!(cpu.registers.y, 0x00);
    assert_eq!(cpu.registers.p, StatusFlags::ZERO);

    process_instruction(&mut cpu, &[LDY_IMMEDIATE, 0xFF]);
    assert_eq!(cpu.registers.y, 0xFF);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE);
}

#[test]
fn process_sec() {
    let mut cpu = cpu(bus());
    process_instruction(&mut cpu, &[SEC_IMPLIED]);
    assert_eq!(cpu.registers.p, StatusFlags::CARRY);
}

#[test]
fn process_sed() {
    let mut cpu = cpu(bus());
    process_instruction(&mut cpu, &[SED_IMPLIED]);
    assert_eq!(cpu.registers.p, StatusFlags::DECIMAL);
}

#[test]
fn process_sei() {
    let mut cpu = cpu(bus());
    process_instruction(&mut cpu, &[SEI_IMPLIED]);
    assert_eq!(cpu.registers.p, StatusFlags::INTERRUPT_DISABLE);
}

#[test]
fn process_tax() {
    let mut cpu = cpu(bus());

    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0x40]);
    process_instruction(&mut cpu, &[TAX_IMPLIED]);
    assert_eq!(cpu.registers.x, cpu.registers.a);
    assert_eq!(cpu.registers.p, StatusFlags::empty());

    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0x40]);
    cpu.registers.p = StatusFlags::empty();
    process_instruction(&mut cpu, &[TAX_IMPLIED]);
    assert_eq!(cpu.registers.x, cpu.registers.a);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE);

    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0x80]);
    cpu.registers.p = StatusFlags::empty();
    process_instruction(&mut cpu, &[TAX_IMPLIED]);
    assert_eq!(cpu.registers.x, cpu.registers.a);
    assert_eq!(cpu.registers.p, StatusFlags::ZERO);
}

#[test]
fn process_tay() {
    let mut cpu = cpu(bus());

    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0x40]);
    process_instruction(&mut cpu, &[TAY_IMPLIED]);
    assert_eq!(cpu.registers.y, cpu.registers.a);
    assert_eq!(cpu.registers.p, StatusFlags::empty());

    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0x40]);
    cpu.registers.p = StatusFlags::empty();
    process_instruction(&mut cpu, &[TAY_IMPLIED]);
    assert_eq!(cpu.registers.y, cpu.registers.a);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE);

    process_instruction(&mut cpu, &[ADC_IMMEDIATE, 0x80]);
    cpu.registers.p = StatusFlags::empty();
    process_instruction(&mut cpu, &[TAY_IMPLIED]);
    assert_eq!(cpu.registers.y, cpu.registers.a);
    assert_eq!(cpu.registers.p, StatusFlags::ZERO);
}

#[test]
// TODO: set X with LDX
fn process_txa() {
    let mut cpu = cpu(bus());

    process_instruction(&mut cpu, &[TXA_IMPLIED]);
    assert_eq!(cpu.registers.a, cpu.registers.x);
    assert_eq!(cpu.registers.p, StatusFlags::ZERO);

    cpu.registers.x = 0x40;
    process_instruction(&mut cpu, &[TXA_IMPLIED]);
    assert_eq!(cpu.registers.a, cpu.registers.x);
    assert_eq!(cpu.registers.p, StatusFlags::empty());

    cpu.registers.x = 0x80;
    process_instruction(&mut cpu, &[TXA_IMPLIED]);
    assert_eq!(cpu.registers.a, cpu.registers.x);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE);
}

#[test]
// TODO: set X with LDY
fn process_tya() {
    let mut cpu = cpu(bus());

    process_instruction(&mut cpu, &[TYA_IMPLIED]);
    assert_eq!(cpu.registers.a, cpu.registers.y);
    assert_eq!(cpu.registers.p, StatusFlags::ZERO);

    cpu.registers.y = 0x40;
    process_instruction(&mut cpu, &[TYA_IMPLIED]);
    assert_eq!(cpu.registers.a, cpu.registers.y);
    assert_eq!(cpu.registers.p, StatusFlags::empty());

    cpu.registers.y = 0x80;
    process_instruction(&mut cpu, &[TYA_IMPLIED]);
    assert_eq!(cpu.registers.a, cpu.registers.y);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE);
}
