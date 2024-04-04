#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::*;
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Input, Level, Output, Pin, Pull, Speed};
use embassy_stm32::usart::{BasicInstance, Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use stm32l476rg::pin::util::*;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

pub fn test_sbc_indx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x80, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x02]);
    check_valid_register_status(usart, TxReg::A, &[0x02]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x81, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x41]);
    check_valid_register_status(usart, TxReg::A, &[0x41]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x02]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE1, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_sbc_indx_within_internal_memory passed!");
}

pub fn test_sbc_zp_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x3F]);
    check_valid_register_status(usart, TxReg::A, &[0x3F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE5, 0x78]);
    check_valid_register_status(usart, TxReg::A, &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    info!("test_sbc_zp_within_internal_memory passed!");
}

pub fn test_sbc_imm_without_carry_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE9, 0x00]);
    check_valid_register_status(usart, TxReg::A, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b01100101]);
    info!("test_sbc_imm_without_carry_within_internal_memory passed!");
}

pub fn test_sbc_imm_with_carry_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    info!("test_sbc_imm_with_carry_within_internal_memory passed!");
}

pub fn test_sbc_imm_with_overflow_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE9, 0x41]);
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_sbc_imm_with_overflow_within_internal_memory passed!");
}

pub fn test_sbc_abs_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x3F]);
    check_valid_register_status(usart, TxReg::A, &[0x3F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xED, 0x78, 0x06]);
    check_valid_register_status(usart, TxReg::A, &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    info!("test_sbc_abs_within_internal_memory passed!");
}

pub fn test_sbc_indy_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x33, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x04]);
    check_valid_register_status(usart, TxReg::A, &[0x04]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x34, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x04]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xF1, 0x33]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    info!("test_sbc_indy_within_internal_memory passed!");
}

pub fn test_sbc_zpx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x41]);
    check_valid_register_status(usart, TxReg::A, &[0x41]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xF5, 0x00]);
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_sbc_zpx_within_internal_memory passed!");
}

pub fn test_sbc_absy_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x7F]);
    check_valid_register_status(usart, TxReg::A, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x04]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x81]);
    check_valid_register_status(usart, TxReg::A, &[0x81]);
    check_valid_register_status(usart, TxReg::P, &[0b10100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xF9, 0x00, 0x04]);
    check_valid_register_status(usart, TxReg::A, &[0x02]);
    check_valid_register_status(usart, TxReg::P, &[0b01100101]);
    info!("test_sbc_absy_within_internal_memory passed!");
}

pub fn test_sbc_absx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xFD, 0x00, 0x06]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    info!("test_sbc_absx_within_internal_memory passed!");
}

pub fn test_sbc_indx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE1, 0x80]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x80, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x81, 0x00]);
    usart.blocking_write(&[0x02]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x02]);
    usart.blocking_write(&[0x41]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_sbc_indx_within_mocking_memory passed!");
}

pub fn test_sbc_zp_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE5, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x3F]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    info!("test_sbc_zp_within_mocking_memory passed!");
}

pub fn test_sbc_imm_without_carry_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE9, 0x00]);
    check_valid_register_status(usart, TxReg::A, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b01100101]);
    info!("test_sbc_imm_without_carry_within_mocking_memory passed!");
}

pub fn test_sbc_imm_with_carry_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    info!("test_sbc_imm_with_carry_within_mocking_memory passed!");
}

pub fn test_sbc_imm_with_overflow_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE9, 0x41]);
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_sbc_imm_with_overflow_within_mocking_memory passed!");
}

pub fn test_sbc_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xED, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x3F]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    info!("test_sbc_abs_within_mocking_memory passed!");
}

pub fn test_sbc_indy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xF1, 0x33]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x33, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x34, 0x00]);
    usart.blocking_write(&[0x04]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0x40]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    info!("test_sbc_indy_within_mocking_memory passed!");
}

pub fn test_sbc_zpx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xF5, 0x00]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x41]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_sbc_zpx_within_mocking_memory passed!");
}

pub fn test_sbc_absy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x81]);
    check_valid_register_status(usart, TxReg::A, &[0x81]);
    check_valid_register_status(usart, TxReg::P, &[0b10100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xF9, 0x00, 0x04]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0x7F]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x02]);
    check_valid_register_status(usart, TxReg::P, &[0b01100101]);
    info!("test_sbc_absy_within_mocking_memory passed!");
}

pub fn test_sbc_absx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xFD, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x40]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    info!("test_sbc_absx_within_mocking_memory passed!");
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let config = Config::default();
    let mut usart = Uart::new_with_rtscts(
        p.USART1, p.PA10, p.PA9, Irqs, p.PA12, p.PA11, NoDma, NoDma, config,
    )
    .unwrap();
    let _rw = Input::new(p.PA0, Pull::None);
    let nop = Input::new(p.PA1, Pull::None);
    let mut resb = Output::new(p.PA4, Level::Low, Speed::Medium);
    test_sbc_indx_within_internal_memory(&mut usart, &nop, &mut resb);
    test_sbc_zp_within_internal_memory(&mut usart, &nop, &mut resb);
    test_sbc_imm_without_carry_within_internal_memory(&mut usart, &nop, &mut resb);
    test_sbc_imm_with_carry_within_internal_memory(&mut usart, &nop, &mut resb);
    test_sbc_imm_with_overflow_within_internal_memory(&mut usart, &nop, &mut resb);
    test_sbc_abs_within_internal_memory(&mut usart, &nop, &mut resb);
    test_sbc_indy_within_internal_memory(&mut usart, &nop, &mut resb);
    test_sbc_zpx_within_internal_memory(&mut usart, &nop, &mut resb);
    test_sbc_absy_within_internal_memory(&mut usart, &nop, &mut resb);
    test_sbc_absx_within_internal_memory(&mut usart, &nop, &mut resb);

    test_sbc_indx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_sbc_zp_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_sbc_imm_without_carry_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_sbc_imm_with_carry_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_sbc_imm_with_overflow_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_sbc_abs_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_sbc_indy_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_sbc_zpx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_sbc_absy_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_sbc_absx_within_mocking_memory(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
