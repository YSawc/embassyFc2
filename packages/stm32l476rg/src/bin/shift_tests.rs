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

pub fn test_asl_zp_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x06, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x80]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    info!("test_asl_zp_within_mocking_memory passed!");
}

pub fn test_asl_acc_without_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x21]);
    check_valid_register_status(usart, TxReg::A, &[0x21]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x0A]);
    check_valid_register_status(usart, TxReg::A, &[0x42]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_asl_acc_without_flag_within_mocking_memory passed!");
}

pub fn test_asl_acc_with_n_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x0A]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_asl_acc_with_n_within_mocking_memory passed!");
}

pub fn test_asl_acc_with_cz_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
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
    usart_write(usart, &[OpeMode::Inst as u8, 0x0A]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    info!("test_asl_acc_with_cz_within_mocking_memory passed!");
}

pub fn test_asl_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x0E, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x80]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    info!("test_asl_abs_within_mocking_memory passed!");
}

pub fn test_asl_zpx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x55]);
    check_valid_register_status(usart, TxReg::X, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x16, 0x00]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x55, 0x00]);
    usart.blocking_write(&[0x55]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_asl_zpx_within_mocking_memory passed!");
}

pub fn test_asl_absx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x55]);
    check_valid_register_status(usart, TxReg::X, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x1E, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x55, 0x06]);
    usart.blocking_write(&[0x55]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_asl_absx_within_mocking_memory passed!");
}

pub fn test_rol_zp_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x26, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x55]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_rol_zp_within_mocking_memory passed!");
}

pub fn test_rol_acc_without_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x20]);
    check_valid_register_status(usart, TxReg::A, &[0x20]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x2A]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_rol_acc_without_flag_within_mocking_memory passed!");
}

pub fn test_rol_acc_with_c_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x2A]);
    check_valid_register_status(usart, TxReg::A, &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    info!("test_rol_acc_with_c_within_mocking_memory passed!");
}

pub fn test_rol_acc_with_z_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x2A]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_rol_acc_with_z_within_mocking_memory passed!");
}

pub fn test_rol_acc_with_n_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x40]);
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x2A]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_rol_acc_with_n_within_mocking_memory passed!");
}

pub fn test_rol_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x2E, 0x78, 0x26]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x26]);
    usart.blocking_write(&[0x55]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_rol_abs_within_mocking_memory passed!");
}

pub fn test_rol_zpx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x55]);
    check_valid_register_status(usart, TxReg::X, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x36, 0x00]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x55, 0x00]);
    usart.blocking_write(&[0x80]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    info!("test_rol_zpx_within_mocking_memory passed!");
}

pub fn test_rol_absx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x55]);
    check_valid_register_status(usart, TxReg::X, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x3E, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x55, 0x06]);
    usart.blocking_write(&[0x80]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    info!("test_rol_absx_within_mocking_memory passed!");
}

pub fn test_lsr_zp_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x46, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x01]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    info!("test_lsr_zp_within_mocking_memory passed!");
}

pub fn test_lsr_acc_without_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x0C]);
    check_valid_register_status(usart, TxReg::A, &[0x0C]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4A]);
    check_valid_register_status(usart, TxReg::A, &[0x06]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_lsr_acc_without_flag_within_mocking_memory passed!");
}

pub fn test_lsr_acc_with_cz_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4A]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    info!("test_lsr_acc_with_cz_within_mocking_memory passed!");
}

pub fn test_lsr_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4E, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x01]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    info!("test_lsr_abs_within_mocking_memory passed!");
}

pub fn test_lsr_zpx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x55]);
    check_valid_register_status(usart, TxReg::X, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x56, 0x00]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x55, 0x00]);
    usart.blocking_write(&[0xAA]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_lsr_zpx_within_mocking_memory passed!");
}

pub fn test_lsr_absx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x55]);
    check_valid_register_status(usart, TxReg::X, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x5E, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x55, 0x06]);
    usart.blocking_write(&[0xAA]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_lsr_absx_within_mocking_memory passed!");
}

pub fn test_ror_zp_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x66, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x01]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10100101]);
    info!("test_ror_zp_within_mocking_memory passed!");
}

pub fn test_ror_acc_without_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x02]);
    check_valid_register_status(usart, TxReg::A, &[0x02]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x6A]);
    check_valid_register_status(usart, TxReg::A, &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_ror_acc_without_flag_within_mocking_memory passed!");
}

pub fn test_ror_acc_with_zc_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x6A]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    info!("test_ror_acc_with_c_within_mocking_memory passed!");
}

pub fn test_ror_acc_with_cn_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x6A]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10100101]);
    info!("test_ror_acc_with_n_within_mocking_memory passed!");
}

pub fn test_ror_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x6E, 0x78, 0x26]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x26]);
    usart.blocking_write(&[0x55]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x2A]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    info!("test_ror_abs_within_mocking_memory passed!");
}

pub fn test_ror_zpx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x55]);
    check_valid_register_status(usart, TxReg::X, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x76, 0x00]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x55, 0x00]);
    usart.blocking_write(&[0x55]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x2A]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    info!("test_ror_zpx_within_mocking_memory passed!");
}

pub fn test_ror_absx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC9, 0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x55]);
    check_valid_register_status(usart, TxReg::X, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x7E, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x55, 0x06]);
    usart.blocking_write(&[0x01]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 1], &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10100101]);
    info!("test_ror_absx_within_mocking_memory passed!");
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
    test_asl_zp_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_asl_acc_without_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_asl_acc_with_n_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_asl_acc_with_cz_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_asl_abs_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_asl_zpx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_asl_absx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_rol_zp_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_rol_acc_without_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_rol_acc_with_c_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_rol_acc_with_z_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_rol_acc_with_n_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_rol_abs_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_rol_zpx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_rol_absx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_lsr_zp_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_lsr_acc_without_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_lsr_acc_with_cz_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_lsr_abs_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_lsr_zpx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_lsr_absx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ror_zp_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ror_acc_without_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ror_acc_with_zc_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ror_acc_with_cn_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ror_abs_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ror_zpx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ror_absx_within_mocking_memory(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
