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

pub fn test_ora_indx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
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
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAA]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x02]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x01, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ora_indx_within_internal_memory passed!");
}

pub fn test_ora_zp_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x54]);
    check_valid_register_status(usart, TxReg::A, &[0x54]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x05, 0x78]);
    check_valid_register_status(usart, TxReg::A, &[0xFE]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ora_zp_within_internal_memory passed!");
}

pub fn test_ora_imm_without_flag_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x09, 0x10]);
    check_valid_register_status(usart, TxReg::A, &[0x10]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_ora_imm_without_flag_within_internal_memory passed!");
}

pub fn test_ora_imm_with_z_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x09, 0x00]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_ora_imm_with_z_within_internal_memory passed!");
}

pub fn test_ora_imm_with_n_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x09, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ora_imm_with_n_within_internal_memory passed!");
}

pub fn test_ora_abs_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x0D, 0x78, 0x06]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_ora_abs_within_internal_memory passed!");
}

pub fn test_ora_indy_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
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
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x04]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x11, 0x33]);
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ora_indy_within_internal_memory passed!");
}

pub fn test_ora_zpx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x15, 0x78]);
    check_valid_register_status(usart, TxReg::A, &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_ora_zpx_within_internal_memory passed!");
}

pub fn test_ora_absy_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x04]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x19, 0x00, 0x04]);
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ora_absy_within_internal_memory passed!");
}

pub fn test_ora_absx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x1D, 0x00, 0x06]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_ora_absx_within_internal_memory passed!");
}

pub fn test_and_indx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x82, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x03]);
    check_valid_register_status(usart, TxReg::A, &[0x03]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x83, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xEF]);
    check_valid_register_status(usart, TxReg::A, &[0xEF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x03]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xF8]);
    check_valid_register_status(usart, TxReg::A, &[0xF8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x21, 0x82]);
    check_valid_register_status(usart, TxReg::A, &[0xE8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_and_indx_within_internal_memory passed!");
}

pub fn test_and_zp_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x25, 0x78]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_and_zp_within_internal_memory passed!");
}

pub fn test_and_imm_without_flag_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x6F]);
    check_valid_register_status(usart, TxReg::A, &[0x6F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x29, 0xEF]);
    check_valid_register_status(usart, TxReg::A, &[0x6F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_and_imm_without_flag_within_internal_memory passed!");
}

pub fn test_and_imm_with_z_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x29, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_and_imm_with_z_within_internal_memory passed!");
}

pub fn test_and_imm_with_n_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x67]);
    check_valid_register_status(usart, TxReg::A, &[0x67]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x29, 0x98]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_and_imm_with_n_within_internal_memory passed!");
}

pub fn test_and_abs_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xEF]);
    check_valid_register_status(usart, TxReg::A, &[0xEF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);

    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xF8]);
    check_valid_register_status(usart, TxReg::A, &[0xF8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x2D, 0x78, 0x06]);
    check_valid_register_status(usart, TxReg::A, &[0xE8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_and_abs_within_internal_memory passed!");
}

pub fn test_and_indy_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
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
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x34]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x04]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x31, 0x33]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_and_indy_within_internal_memory passed!");
}

pub fn test_and_zpx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xEF]);
    check_valid_register_status(usart, TxReg::A, &[0xEF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xF8]);
    check_valid_register_status(usart, TxReg::A, &[0xF8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x35, 0x00]);
    check_valid_register_status(usart, TxReg::A, &[0xE8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_and_zpx_within_internal_memory passed!");
}

pub fn test_and_absy_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x04]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x39, 0x00, 0x04]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_and_absy_within_internal_memory passed!");
}

pub fn test_and_absx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xEF]);
    check_valid_register_status(usart, TxReg::A, &[0xEF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xF8]);
    check_valid_register_status(usart, TxReg::A, &[0xF8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x3D, 0x00, 0x06]);
    check_valid_register_status(usart, TxReg::A, &[0xE8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_and_absx_within_internal_memory passed!");
}

pub fn test_eor_indx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
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
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x02]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x5F]);
    check_valid_register_status(usart, TxReg::A, &[0x5F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x41, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0xF5]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_eor_indx_within_internal_memory passed!");
}

pub fn test_eor_zp_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x70]);
    check_valid_register_status(usart, TxReg::A, &[0x70]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x70]);
    check_valid_register_status(usart, TxReg::A, &[0x70]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x45, 0x78]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_eor_zp_within_internal_memory passed!");
}

pub fn test_eor_imm_without_flag_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xDF]);
    check_valid_register_status(usart, TxReg::A, &[0xDF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x49, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0x75]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_eor_imm_without_flag_within_internal_memory passed!");
}

pub fn test_eor_imm_with_z_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x70]);
    check_valid_register_status(usart, TxReg::A, &[0x70]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x49, 0x70]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_eor_imm_with_z_within_internal_memory passed!");
}

pub fn test_eor_imm_with_n_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x5F]);
    check_valid_register_status(usart, TxReg::A, &[0x5F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x49, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xF5]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_eor_imm_with_n_within_internal_memory passed!");
}

pub fn test_eor_abs_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x5F]);
    check_valid_register_status(usart, TxReg::A, &[0x5F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4D, 0x78, 0x06]);
    check_valid_register_status(usart, TxReg::A, &[0xF5]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_eor_abs_within_internal_memory passed!");
}

pub fn test_eor_indy_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
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
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x70]);
    check_valid_register_status(usart, TxReg::A, &[0x70]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x04]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x70]);
    check_valid_register_status(usart, TxReg::A, &[0x70]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x51, 0x33]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_eor_indy_within_internal_memory passed!");
}

pub fn test_eor_zpx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x5F]);
    check_valid_register_status(usart, TxReg::A, &[0x5F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x55, 0x00]);
    check_valid_register_status(usart, TxReg::A, &[0xF5]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_eor_zpx_within_internal_memory passed!");
}

pub fn test_eor_absy_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x70]);
    check_valid_register_status(usart, TxReg::A, &[0x70]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x04]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x70]);
    check_valid_register_status(usart, TxReg::A, &[0x70]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x59, 0x00, 0x04]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_eor_absy_within_internal_memory passed!");
}

pub fn test_eor_absx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);

    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x5F]);
    check_valid_register_status(usart, TxReg::A, &[0x5F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x5D, 0x00, 0x06]);
    check_valid_register_status(usart, TxReg::A, &[0xF5]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_eor_absx_within_internal_memory passed!");
}

pub fn test_ora_indx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x01, 0x80]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x80, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x81, 0x00]);
    usart.blocking_write(&[0x02]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x02]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ora_indx_within_mocking_memory passed!");
}

pub fn test_ora_zp_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x54]);
    check_valid_register_status(usart, TxReg::A, &[0x54]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x05, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xFE]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ora_zp_within_mocking_memory passed!");
}

pub fn test_ora_imm_without_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x09, 0x10]);
    check_valid_register_status(usart, TxReg::A, &[0x10]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_ora_imm_without_flag_within_mocking_memory passed!");
}

pub fn test_ora_imm_with_z_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x09, 0x00]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_ora_imm_with_z_within_mocking_memory passed!");
}

pub fn test_ora_imm_with_n_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x09, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ora_imm_with_n_within_mocking_memory passed!");
}

pub fn test_ora_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x0D, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x00]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_ora_abs_within_mocking_memory passed!");
}

pub fn test_ora_indy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x11, 0x33]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x33, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x34, 0x00]);
    usart.blocking_write(&[0x04]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0x55]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ora_indy_within_mocking_memory passed!");
}

pub fn test_ora_zpx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
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
    usart_write(usart, &[OpeMode::Inst as u8, 0x15, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_ora_zpx_within_mocking_memory passed!");
}

pub fn test_ora_absy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x19, 0x00, 0x04]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ora_absy_within_mocking_memory passed!");
}

pub fn test_ora_absx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x1D, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x00]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_ora_absx_within_mocking_memory passed!");
}

pub fn test_and_indx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xF8]);
    check_valid_register_status(usart, TxReg::A, &[0xF8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x21, 0x82]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x82, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x83, 0x00]);
    usart.blocking_write(&[0x03]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x03]);
    usart.blocking_write(&[0xEF]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xE8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_and_indx_within_mocking_memory passed!");
}

pub fn test_and_zp_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x25, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_and_zp_within_mocking_memory passed!");
}

pub fn test_and_imm_without_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x6F]);
    check_valid_register_status(usart, TxReg::A, &[0x6F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x29, 0xEF]);
    check_valid_register_status(usart, TxReg::A, &[0x6F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_and_imm_without_flag_within_mocking_memory passed!");
}

pub fn test_and_imm_with_z_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x29, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_and_imm_with_z_within_mocking_memory passed!");
}

pub fn test_and_imm_with_n_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x67]);
    check_valid_register_status(usart, TxReg::A, &[0x67]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x29, 0x98]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_and_imm_with_n_within_mocking_memory passed!");
}

pub fn test_and_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xF8]);
    check_valid_register_status(usart, TxReg::A, &[0xF8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x2D, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0xEF]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xE8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_and_abs_within_mocking_memory passed!");
}

pub fn test_and_indy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x31, 0x33]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x33, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x34, 0x00]);
    usart.blocking_write(&[0x04]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_and_indy_within_mocking_memory passed!");
}

pub fn test_and_zpx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xF8]);
    check_valid_register_status(usart, TxReg::A, &[0xF8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x35, 0x00]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0xEF]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xE8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_and_zpx_within_mocking_memory passed!");
}

pub fn test_and_absy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x39, 0x00, 0x04]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_and_absy_within_mocking_memory passed!");
}

pub fn test_and_absx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xF8]);
    check_valid_register_status(usart, TxReg::A, &[0xF8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x3D, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0xEF]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xE8]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_and_absx_within_mocking_memory passed!");
}

pub fn test_eor_indx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x5F]);
    check_valid_register_status(usart, TxReg::A, &[0x5F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x41, 0x80]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x80, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x81, 0x00]);
    usart.blocking_write(&[0x02]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x02]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xF5]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_eor_indx_within_mocking_memory passed!");
}

pub fn test_eor_zp_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x70]);
    check_valid_register_status(usart, TxReg::A, &[0x70]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x45, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x70]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_eor_zp_within_mocking_memory passed!");
}

pub fn test_eor_imm_without_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xDF]);
    check_valid_register_status(usart, TxReg::A, &[0xDF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x49, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0x75]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_eor_imm_without_flag_within_mocking_memory passed!");
}

pub fn test_eor_imm_with_z_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x70]);
    check_valid_register_status(usart, TxReg::A, &[0x70]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x49, 0x70]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_eor_imm_with_z_within_mocking_memory passed!");
}

pub fn test_eor_imm_with_n_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x5F]);
    check_valid_register_status(usart, TxReg::A, &[0x5F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x49, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xF5]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_eor_imm_with_n_within_mocking_memory passed!");
}

pub fn test_eor_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x5F]);
    check_valid_register_status(usart, TxReg::A, &[0x5F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4D, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xF5]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_eor_abs_within_mocking_memory passed!");
}

pub fn test_eor_indy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x70]);
    check_valid_register_status(usart, TxReg::A, &[0x70]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x51, 0x33]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x33, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x34, 0x00]);
    usart.blocking_write(&[0x04]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0x70]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_eor_indy_within_mocking_memory passed!");
}

pub fn test_eor_zpx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x5F]);
    check_valid_register_status(usart, TxReg::A, &[0x5F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x55, 0x00]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xF5]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_eor_zpx_within_mocking_memory passed!");
}

pub fn test_eor_absy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x70]);
    check_valid_register_status(usart, TxReg::A, &[0x70]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x59, 0x00, 0x04]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x04]);
    usart.blocking_write(&[0x70]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_eor_absy_within_mocking_memory passed!");
}

pub fn test_eor_absx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x5F]);
    check_valid_register_status(usart, TxReg::A, &[0x5F]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x78]);
    check_valid_register_status(usart, TxReg::X, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x5D, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xF5]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_eor_absx_within_mocking_memory passed!");
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
    test_ora_indx_within_internal_memory(&mut usart, &nop, &mut resb);
    test_ora_zp_within_internal_memory(&mut usart, &nop, &mut resb);
    test_ora_imm_without_flag_within_internal_memory(&mut usart, &nop, &mut resb);
    test_ora_imm_with_z_within_internal_memory(&mut usart, &nop, &mut resb);
    test_ora_imm_with_n_within_internal_memory(&mut usart, &nop, &mut resb);
    test_ora_abs_within_internal_memory(&mut usart, &nop, &mut resb);
    test_ora_indy_within_internal_memory(&mut usart, &nop, &mut resb);
    test_ora_zpx_within_internal_memory(&mut usart, &nop, &mut resb);
    test_ora_absy_within_internal_memory(&mut usart, &nop, &mut resb);
    test_ora_absx_within_internal_memory(&mut usart, &nop, &mut resb);
    test_and_indx_within_internal_memory(&mut usart, &nop, &mut resb);
    test_and_zp_within_internal_memory(&mut usart, &nop, &mut resb);
    test_and_imm_without_flag_within_internal_memory(&mut usart, &nop, &mut resb);
    test_and_imm_with_z_within_internal_memory(&mut usart, &nop, &mut resb);
    test_and_imm_with_n_within_internal_memory(&mut usart, &nop, &mut resb);
    test_and_abs_within_internal_memory(&mut usart, &nop, &mut resb);
    test_and_indy_within_internal_memory(&mut usart, &nop, &mut resb);
    test_and_zpx_within_internal_memory(&mut usart, &nop, &mut resb);
    test_and_absy_within_internal_memory(&mut usart, &nop, &mut resb);
    test_and_absx_within_internal_memory(&mut usart, &nop, &mut resb);
    test_eor_indx_within_internal_memory(&mut usart, &nop, &mut resb);
    test_eor_zp_within_internal_memory(&mut usart, &nop, &mut resb);
    test_eor_imm_without_flag_within_internal_memory(&mut usart, &nop, &mut resb);
    test_eor_imm_with_z_within_internal_memory(&mut usart, &nop, &mut resb);
    test_eor_imm_with_n_within_internal_memory(&mut usart, &nop, &mut resb);
    test_eor_abs_within_internal_memory(&mut usart, &nop, &mut resb);
    test_eor_indy_within_internal_memory(&mut usart, &nop, &mut resb);
    test_eor_zpx_within_internal_memory(&mut usart, &nop, &mut resb);
    test_eor_absy_within_internal_memory(&mut usart, &nop, &mut resb);
    test_eor_absx_within_internal_memory(&mut usart, &nop, &mut resb);

    test_ora_indx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ora_zp_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ora_imm_without_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ora_imm_with_z_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ora_imm_with_n_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ora_abs_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ora_indy_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ora_zpx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ora_absy_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_ora_absx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_and_indx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_and_zp_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_and_imm_without_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_and_imm_with_z_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_and_imm_with_n_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_and_abs_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_and_indy_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_and_zpx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_and_absy_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_and_absx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_eor_indx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_eor_zp_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_eor_imm_without_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_eor_imm_with_z_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_eor_imm_with_n_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_eor_abs_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_eor_indy_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_eor_zpx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_eor_absy_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_eor_absx_within_mocking_memory(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
