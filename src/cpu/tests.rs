#![cfg(test)]

use super::*;

const ADDRESS_PRG: u16 = 0x8000;
const INPUT_OPCODE: u8 = 0xFF;
const INPUT_BYTE: u8 = 0x4F;
const INPUT_ADDRESS_ZP: u16 = 0x0040;
const INPUT_ADDRESS: u16 = 0x4020;
const INPUT_ADDRESS_LOW: u8 = INPUT_ADDRESS.to_le_bytes()[0];
const INPUT_ADDRESS_HIGH: u8 = INPUT_ADDRESS.to_le_bytes()[1];
const INPUT_ADDRESS_INDIRECT: u16 = 0x1234;
const INPUT_ADDRESS_INDIRECT_LOW: u8 = INPUT_ADDRESS_INDIRECT.to_le_bytes()[0];
const INPUT_ADDRESS_INDIRECT_HIGH: u8 = INPUT_ADDRESS_INDIRECT.to_le_bytes()[1];
const OFFSET_REGISTER_X: u8 = 0x12;
const OFFSET_REGISTER_Y: u8 = 0x24;

fn bus() -> Bus {
    let mut bus = Bus::new();
    bus.write_u16(ADDRESS_RESET, ADDRESS_PRG).unwrap();
    bus
}

fn cpu(bus: Bus) -> Cpu {
    Cpu::new(bus).unwrap()
}

fn process_instruction(cpu: &mut Cpu, opcode: u8, args: &[u8]) {
    cpu.bus.write(cpu.registers.pc, opcode);
    cpu.bus.write_n(cpu.registers.pc + 1, args);
    let instruction = cpu.determine_instruction_next().unwrap();
    cpu.process_instruction(instruction).unwrap();
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

#[test]
fn determine_input_implied() {
    let cpu = cpu(bus());
    let input = cpu.determine_input(
        InstructionMode::Implied,
        &[INPUT_OPCODE],
    ).unwrap();
    assert_eq!(input, InstructionInput::Implied);
}

#[test]
fn determine_input_accumulator() {
    let cpu = cpu(bus());
    let input = cpu.determine_input(
        InstructionMode::Accumulator,
        &[INPUT_OPCODE],
    ).unwrap();
    assert_eq!(input, InstructionInput::Accumulator);
}

// TODO: constants
#[test]
fn determine_input_relative_positive() {
    let cpu = cpu(bus());
    let input = cpu.determine_input(
        InstructionMode::Relative,
        &[INPUT_OPCODE, 0x0F],
    ).unwrap();
    assert_eq!(input, InstructionInput::Address(0x800F));
}

#[test]
fn determine_input_zero_page() {
    let cpu = cpu(bus());
    let input = cpu.determine_input(
        InstructionMode::ZeroPage,
        &[INPUT_OPCODE, INPUT_ADDRESS_ZP as u8],
    ).unwrap();
    assert_eq!(input, InstructionInput::Address(INPUT_ADDRESS_ZP));
}

#[test]
fn determine_input_zero_page_x() {
    let mut cpu = cpu(bus());
    cpu.registers.x = OFFSET_REGISTER_X;

    let input = cpu.determine_input(
        InstructionMode::ZeroPageX,
        &[INPUT_OPCODE, INPUT_ADDRESS_ZP as u8],
    ).unwrap();
    assert_eq!(input, InstructionInput::Address(INPUT_ADDRESS_ZP + OFFSET_REGISTER_X as u16));
}

#[test]
fn determine_input_zero_page_y() {
    let mut cpu = cpu(bus());
    cpu.registers.y = OFFSET_REGISTER_Y;

    let input = cpu.determine_input(
        InstructionMode::ZeroPageY,
        &[INPUT_OPCODE, INPUT_ADDRESS_ZP as u8],
    ).unwrap();
    assert_eq!(input, InstructionInput::Address(INPUT_ADDRESS_ZP + OFFSET_REGISTER_Y as u16));
}

// TODO: constants
#[test]
fn determine_input_relative_negative() {
    let cpu = cpu(bus());
    let input = cpu.determine_input(
        InstructionMode::Relative,
        &[INPUT_OPCODE, 0xF0],
    ).unwrap();
    assert_eq!(input, InstructionInput::Address(0x7FF0));
}

#[test]
fn determine_input_immediate() {
    let cpu = cpu(bus());
    let input = cpu.determine_input(
        InstructionMode::Immediate,
        &[INPUT_OPCODE, INPUT_BYTE],
    ).unwrap();
    assert_eq!(input, InstructionInput::Byte(INPUT_BYTE));
}

#[test]
fn determine_input_absolute() {
    let cpu = cpu(bus());
    let input = cpu.determine_input(
        InstructionMode::Absolute,
        &[INPUT_OPCODE, INPUT_ADDRESS_LOW, INPUT_ADDRESS_HIGH],
    ).unwrap();
    assert_eq!(input, InstructionInput::Address(INPUT_ADDRESS));
}

#[test]
fn determine_input_absolute_x() {
    let mut cpu = cpu(bus());
    cpu.registers.x = OFFSET_REGISTER_X;

    let input = cpu.determine_input(
        InstructionMode::AbsoluteX,
        &[INPUT_OPCODE, INPUT_ADDRESS_LOW, INPUT_ADDRESS_HIGH],
    ).unwrap();
    assert_eq!(input, InstructionInput::Address(INPUT_ADDRESS + OFFSET_REGISTER_X as u16));
}

#[test]
fn determine_input_absolute_y() {
    let mut cpu = cpu(bus());
    cpu.registers.y = OFFSET_REGISTER_Y;

    let input = cpu.determine_input(
        InstructionMode::AbsoluteY,
        &[INPUT_OPCODE, INPUT_ADDRESS_LOW, INPUT_ADDRESS_HIGH],
    ).unwrap();
    assert_eq!(input, InstructionInput::Address(INPUT_ADDRESS + OFFSET_REGISTER_Y as u16));
}

#[test]
fn determine_input_indirect() {
    let mut bus = bus();
    bus.write_u16(INPUT_ADDRESS_INDIRECT, INPUT_ADDRESS).unwrap();

    let cpu = cpu(bus);
    let input = cpu.determine_input(
        InstructionMode::Indirect,
        &[INPUT_OPCODE, INPUT_ADDRESS_INDIRECT_LOW, INPUT_ADDRESS_INDIRECT_HIGH],
    ).unwrap();
    assert_eq!(input, InstructionInput::Address(INPUT_ADDRESS));
}

#[test]
fn determine_input_indirect_x() {
    let mut bus = bus();
    bus.write_u16(INPUT_ADDRESS_ZP + OFFSET_REGISTER_X as u16, INPUT_ADDRESS).unwrap();

    let mut cpu = cpu(bus);
    cpu.registers.x = OFFSET_REGISTER_X;

    let input = cpu.determine_input(
        InstructionMode::IndirectX,
        &[INPUT_OPCODE, INPUT_ADDRESS_ZP as u8],
    ).unwrap();
    assert_eq!(input, InstructionInput::Address(INPUT_ADDRESS));
}

#[test]
fn determine_input_indirect_y() {
    let mut bus = bus();
    bus.write_u16(INPUT_ADDRESS_ZP + OFFSET_REGISTER_Y as u16, INPUT_ADDRESS).unwrap();

    let mut cpu = cpu(bus);
    cpu.registers.y = OFFSET_REGISTER_Y;

    let input = cpu.determine_input(
        InstructionMode::IndirectY,
        &[INPUT_OPCODE, INPUT_ADDRESS_ZP as u8],
    ).unwrap();
    assert_eq!(input, InstructionInput::Address(INPUT_ADDRESS));
}

#[test]
fn process_adc_absolute() {
    let mut cpu = cpu(bus());

    cpu.bus.write(INPUT_ADDRESS, 0x10);
    process_instruction(&mut cpu, 0x6D, &[INPUT_ADDRESS_LOW, INPUT_ADDRESS_HIGH]);
    assert_eq!(cpu.registers.a, 0x10);
    assert_eq!(cpu.registers.p, StatusFlags::empty());

    cpu.bus.write(INPUT_ADDRESS, 0x70);
    process_instruction(&mut cpu, 0x6D, &[INPUT_ADDRESS_LOW, INPUT_ADDRESS_HIGH]);
    assert_eq!(cpu.registers.a, 0x80);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE | StatusFlags::OVERFLOW);

    cpu.bus.write(INPUT_ADDRESS, 0x80);
    process_instruction(&mut cpu, 0x6D, &[INPUT_ADDRESS_LOW, INPUT_ADDRESS_HIGH]);
    assert_eq!(cpu.registers.a, 0x00);
    assert_eq!(cpu.registers.p, StatusFlags::OVERFLOW | StatusFlags::ZERO | StatusFlags::CARRY);

    cpu.bus.write(INPUT_ADDRESS, 0x10);
    process_instruction(&mut cpu, 0x6D, &[INPUT_ADDRESS_LOW, INPUT_ADDRESS_HIGH]);
    assert_eq!(cpu.registers.a, 0x11);
    assert_eq!(cpu.registers.p, StatusFlags::empty());
}

#[test]
fn process_lda_immediate() {
    let mut cpu = cpu(bus());

    process_instruction(&mut cpu, 0xA9, &[0x10]);
    assert_eq!(cpu.registers.a, 0x10);
    assert_eq!(cpu.registers.p, StatusFlags::empty());

    process_instruction(&mut cpu, 0xA9, &[0x00]);
    assert_eq!(cpu.registers.a, 0x00);
    assert_eq!(cpu.registers.p, StatusFlags::ZERO);

    process_instruction(&mut cpu, 0xA9, &[0xF0]);
    assert_eq!(cpu.registers.a, 0xF0);
    assert_eq!(cpu.registers.p, StatusFlags::NEGATIVE);
}
