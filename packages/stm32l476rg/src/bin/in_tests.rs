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

pub fn test_inc_abs_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x04]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x04]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xee, 0x00, 0x04]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0x00, 0x04]);
    check_valid_register_status(usart, TxReg::A, &[0x41]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);

    info!("test_inc_abs_within_internal_memory passed!");
}

pub fn test_inc_absx_without_carry_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x60]);
    check_valid_register_status(usart, TxReg::A, &[0x60]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x60, 0x40]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2, 0x30]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xFE, 0x30, 0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0x60, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x61]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_inc_absx_without_carry_within_internal_memory passed!");
}

pub fn test_inc_absx_with_carry_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x60]);
    check_valid_register_status(usart, TxReg::A, &[0x60]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x2F, 0x41]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2, 0xff]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xFE, 0x30, 0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0x2F, 0x41]);
    check_valid_register_status(usart, TxReg::A, &[0x61]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_inc_absx_with_carry_within_internal_memory passed!");
}

pub fn test_inc_zp_without_triger_of_p_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x7E]);
    check_valid_register_status(usart, TxReg::A, &[0x7E]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x2C, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xe6, 0x2C]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0x2C, 0x00]);
    check_valid_register_status(usart, TxReg::A, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_inc_zp_without_triger_of_p_within_internal_memory passed!");
}

pub fn test_inc_zp_with_zero_flag_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xFF]);
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x2C, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xe6, 0x2c]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0x2C, 0x00]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_inc_zp_with_zero_flag_within_internal_memory passed!");
}

pub fn test_inc_zp_with_negative_flag_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
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
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x2C, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xe6, 0x2c]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0x2C, 0x00]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_inc_zp_with_negative_flag_within_internal_memory passed!");
}

pub fn test_inc_zpx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xA0]);
    check_valid_register_status(usart, TxReg::A, &[0xA0]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0xB7, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2, 0x50]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xf6, 0x67]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xB7, 0x00]);
    check_valid_register_status(usart, TxReg::A, &[0xA1]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_inc_zpx_within_internal_memory passed!");
}

pub fn test_inx_impl_without_flag_within_internal_memory<
    T: BasicInstance,
    P: Pin,
    P2: Pin,
    P3: Pin,
>(
    usart: &mut Uart<T>,
    rw: &Input<P>,
    nop: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x59]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x59]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE8]);
    check_valid_register_status(usart, TxReg::X, &[0x5A]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
}

pub fn test_inx_impl_with_zero_within_internal_memory<
    T: BasicInstance,
    P: Pin,
    P2: Pin,
    P3: Pin,
>(
    usart: &mut Uart<T>,
    rw: &Input<P>,
    nop: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x7F]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE8]);
    check_valid_register_status(usart, TxReg::X, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
}

pub fn test_iny_impl_without_flag_within_internal_memory<
    T: BasicInstance,
    P: Pin,
    P2: Pin,
    P3: Pin,
>(
    usart: &mut Uart<T>,
    rw: &Input<P>,
    nop: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x23]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0x23]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC8]);
    check_valid_register_status(usart, TxReg::Y, &[0x24]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
}

pub fn test_iny_impl_with_negative_within_internal_memory<
    T: BasicInstance,
    P: Pin,
    P2: Pin,
    P3: Pin,
>(
    usart: &mut Uart<T>,
    rw: &Input<P>,
    nop: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x9B]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0x9B]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC8]);
    check_valid_register_status(usart, TxReg::Y, &[0x9C]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
}

pub fn test_inc_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xee, 0x00, 0x04]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart_write(usart, &[0x40]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x00, 0x04, 0x41]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_inc_abs_within_mocking_memory passed!");
}

pub fn test_inc_absx_without_carry_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2, 0x30]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xFE, 0x30, 0x40]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x60, 0x40]);
    usart_write(usart, &[0x60]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x60, 0x40, 0x61]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_inc_absx_without_carry_within_mocking_memory passed!");
}

pub fn test_inc_absx_with_carry_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2, 0xff]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xFE, 0x30, 0x40]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x2F, 0x41]);
    usart_write(usart, &[0x60]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x2F, 0x41, 0x61]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_inc_absx_with_carry_within_mocking_memory passed!");
}

pub fn test_inc_zp_without_triger_of_p_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xe6, 0x2c]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x2C, 0x00]);
    usart.blocking_write(&[0x7e]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x2C, 0x00, 0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_inc_zp_without_triger_of_p_within_mocking_memory passed!");
}

pub fn test_inc_zp_with_zero_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xe6, 0x2c]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x2C, 0x00]);
    usart.blocking_write(&[0xff]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x2C, 0x00, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_inc_zp_with_zero_flag_within_mocking_memory passed!");
}

pub fn test_inc_zp_with_negative_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xe6, 0x2c]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x2C, 0x00]);
    usart.blocking_write(&[0x7f]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x2C, 0x00, 0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_inc_zp_with_negative_flag_within_mocking_memory passed!");
}

pub fn test_inc_zpx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2, 0x50]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xf6, 0x67]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xB7, 0x00]);
    usart.blocking_write(&[0xa0]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xB7, 0x00, 0xA1]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_inc_zpx_within_mocking_memory passed!");
}

pub fn test_inx_impl_without_flag_within_mocking_memory<
    T: BasicInstance,
    P: Pin,
    P2: Pin,
    P3: Pin,
>(
    usart: &mut Uart<T>,
    rw: &Input<P>,
    nop: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x59]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x59]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE8]);
    check_valid_register_status(usart, TxReg::X, &[0x5A]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
}

pub fn test_inx_impl_with_zero_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    rw: &Input<P>,
    nop: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x7F]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xE8]);
    check_valid_register_status(usart, TxReg::X, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
}

pub fn test_iny_impl_without_flag_within_mocking_memory<
    T: BasicInstance,
    P: Pin,
    P2: Pin,
    P3: Pin,
>(
    usart: &mut Uart<T>,
    rw: &Input<P>,
    nop: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x23]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0x23]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC8]);
    check_valid_register_status(usart, TxReg::Y, &[0x24]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
}

pub fn test_iny_impl_with_negative_within_mocking_memory<
    T: BasicInstance,
    P: Pin,
    P2: Pin,
    P3: Pin,
>(
    usart: &mut Uart<T>,
    rw: &Input<P>,
    nop: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x9B]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0x9B]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC8]);
    check_valid_register_status(usart, TxReg::Y, &[0x9C]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = embassy_stm32::init(Default::default());
    let config = Config::default();
    let mut usart = Uart::new_with_rtscts(
        p.USART1, p.PA10, p.PA9, Irqs, p.PA12, p.PA11, NoDma, NoDma, config,
    )
    .unwrap();
    let rw = Input::new(p.PA0, Pull::None);
    let nop = Input::new(p.PA1, Pull::None);
    let mut resb = Output::new(p.PA4, Level::Low, Speed::Medium);
    test_inc_abs_within_internal_memory(&mut usart, &nop, &mut resb);
    test_inc_absx_without_carry_within_internal_memory(&mut usart, &nop, &mut resb);
    test_inc_absx_with_carry_within_internal_memory(&mut usart, &nop, &mut resb);
    test_inc_zp_without_triger_of_p_within_internal_memory(&mut usart, &nop, &mut resb);
    test_inc_zp_with_zero_flag_within_internal_memory(&mut usart, &nop, &mut resb);
    test_inc_zp_with_negative_flag_within_internal_memory(&mut usart, &nop, &mut resb);
    test_inc_zpx_within_internal_memory(&mut usart, &nop, &mut resb);
    test_inx_impl_without_flag_within_internal_memory(&mut usart, &rw, &nop, &mut resb);
    test_inx_impl_with_zero_within_internal_memory(&mut usart, &rw, &nop, &mut resb);
    test_iny_impl_without_flag_within_internal_memory(&mut usart, &rw, &nop, &mut resb);
    test_iny_impl_with_negative_within_internal_memory(&mut usart, &rw, &nop, &mut resb);

    test_inc_abs_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_inc_absx_without_carry_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_inc_absx_with_carry_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_inc_zp_without_triger_of_p_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_inc_zp_with_zero_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_inc_zp_with_negative_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_inc_zpx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_inx_impl_without_flag_within_mocking_memory(&mut usart, &rw, &nop, &mut resb);
    test_inx_impl_with_zero_within_mocking_memory(&mut usart, &rw, &nop, &mut resb);
    test_iny_impl_without_flag_within_mocking_memory(&mut usart, &rw, &nop, &mut resb);
    test_iny_impl_with_negative_within_mocking_memory(&mut usart, &rw, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
