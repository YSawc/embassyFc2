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

pub fn test_lda_nestest_head<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::NesTest as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0x00, 0xC0]);
    check_valid_register_status(usart, TxReg::A, &[0x4C]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0x01, 0xC0]);
    check_valid_register_status(usart, TxReg::A, &[0xF5]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0x02, 0xC0]);
    check_valid_register_status(usart, TxReg::A, &[0xC5]);
}

pub fn test_lda_indx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xFF]);
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x33, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xF0]);
    check_valid_register_status(usart, TxReg::A, &[0xF0]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x34, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xCF]);
    check_valid_register_status(usart, TxReg::A, &[0xCF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0xFF, 0xF0]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x18]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x18]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9]);
    usart.blocking_write(&[0x40]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA1]);
    usart.blocking_write(&[0x1B]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xCF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_lda_indx_within_internal_memory passed!");
}

pub fn test_lda_zp_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x45]);
    check_valid_register_status(usart, TxReg::A, &[0x45]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x25, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa5]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x25]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x45]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_lda_zp_within_internal_memory passed!");
}

pub fn test_lda_imm_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa9]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x34]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x34]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_lda_imm_within_internal_memory passed!");
}

pub fn test_lda_abs_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x80, 0x01]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD]);
    check_rw_is_high(&rw);
    usart_write(usart, &[0x80, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_lda_abs_within_internal_memory passed!");
}

pub fn test_lda_indy_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x89, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x03]);
    check_valid_register_status(usart, TxReg::A, &[0x03]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x8A, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x89]);
    check_valid_register_status(usart, TxReg::A, &[0x89]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x00, 0x03]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x00]);
    check_valid_register_status(usart, TxReg::Y, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB1]);
    check_rw_is_high(&rw);
    usart_write(usart, &[0x89]);
    check_valid_register_status(usart, TxReg::A, &[0x89]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_lda_indy_within_internal_memory passed!");
}

pub fn test_lda_zpx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x0C]);
    check_valid_register_status(usart, TxReg::A, &[0x0C]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x65, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x05]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x05]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB5, 0x60]);
    check_valid_register_status(usart, TxReg::A, &[0x0C]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_lda_zpx_within_internal_memory passed!");
}

pub fn test_lda_absy_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xBB]);
    check_valid_register_status(usart, TxReg::A, &[0xBB]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x74, 0x41]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x44]);
    check_valid_register_status(usart, TxReg::Y, &[0x44]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB9]);
    check_rw_is_high(&rw);
    usart_write(usart, &[0x30, 0x41]);
    check_valid_register_status(usart, TxReg::A, &[0xBB]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_lda_absy_within_internal_memory passed!");
}

pub fn test_lda_absx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xD4]);
    check_valid_register_status(usart, TxReg::A, &[0xD4]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x1C, 0x0A]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x75]);
    check_valid_register_status(usart, TxReg::X, &[0x75]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xBD]);
    check_rw_is_high(&rw);
    usart_write(usart, &[0xA7, 0x09]);
    check_valid_register_status(usart, TxReg::A, &[0xD4]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_lda_absx_within_internal_memory passed!");
}

pub fn test_ldx_imm_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x45]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x45]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_ldx_imm_within_internal_memory passed!");
}

pub fn test_ldx_zp_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x90]);
    check_valid_register_status(usart, TxReg::A, &[0x90]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0xF0, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa6]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0xF0]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x90]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ldx_zp_within_internal_memory passed!");
}

pub fn test_ldx_abs_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xFB]);
    check_valid_register_status(usart, TxReg::A, &[0xFB]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0xFF, 0x07]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAE]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0xFF, 0x07]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0xFB]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ldx_abs_within_internal_memory passed!");
}

pub fn test_ldx_zpy_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x3B]);
    check_valid_register_status(usart, TxReg::A, &[0x3B]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x51, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0xC0]);
    check_valid_register_status(usart, TxReg::Y, &[0xC0]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB6]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x91]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x3B]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_ldx_zpy_within_internal_memory passed!");
}

pub fn test_ldx_absy_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x33]);
    check_valid_register_status(usart, TxReg::A, &[0x33]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);

    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x78]);
    check_valid_register_status(usart, TxReg::Y, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xBE]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x00, 0x06]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x33]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_ldx_absy_within_internal_memory passed!");
}

pub fn test_ldy_imm_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa0]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0xba]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0xba]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ldy_imm_within_internal_memory passed!");
}

pub fn test_ldy_zp_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xDD]);
    check_valid_register_status(usart, TxReg::A, &[0xDD]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0xC3, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA4]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0xC3]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0xDD]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ldy_zp_within_internal_memory passed!");
}

pub fn test_ldy_abs_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x55]);
    check_valid_register_status(usart, TxReg::A, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAC]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x78, 0x06]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_ldy_abs_within_internal_memory passed!");
}

pub fn test_ldy_zpx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAA]);
    check_valid_register_status(usart, TxReg::A, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x33, 0x00]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x00]);
    check_valid_register_status(usart, TxReg::X, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB4]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x33]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ldy_zpx_within_internal_memory passed!");
}

pub fn test_ldy_absx_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x89, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x8A]);
    check_valid_register_status(usart, TxReg::X, &[0x8A]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xBC]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0xFF, 0x05]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_ldy_absx_within_internal_memory passed!");
}

pub fn test_lda_indx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x18]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x18]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9]);
    usart.blocking_write(&[0x40]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x40]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA1]);
    usart.blocking_write(&[0x1B]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x33, 0x00]);
    usart.blocking_write(&[0xFF]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x34, 0x00]);
    usart.blocking_write(&[0xF0]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xFF, 0xF0]);
    usart.blocking_write(&[0xCF]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0xCF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_lda_indx_within_mocking_memory passed!");
}

pub fn test_lda_zp_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa5]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x25]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x25, 0x00]);
    usart.blocking_write(&[0x45]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x45]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_lda_zp_within_mocking_memory passed!");
}

pub fn test_lda_imm_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa9]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x34]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x34]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_lda_imm_within_mocking_memory passed!");
}

pub fn test_lda_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD]);
    check_rw_is_high(&rw);
    usart_write(usart, &[0x80, 0x01]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x80, 0x01]);
    usart_write(usart, &[0x00]);
    check_valid_register_status(usart, TxReg::A, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_lda_abs_within_mocking_memory passed!");
}

pub fn test_lda_indy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x00]);
    check_valid_register_status(usart, TxReg::Y, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB1]);
    check_rw_is_high(&rw);
    usart_write(usart, &[0x89]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x89, 0x00]);
    usart_write(usart, &[0x00]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x8A, 0x00]);
    usart_write(usart, &[0x03]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x00, 0x03]);
    usart_write(usart, &[0x89]);
    check_valid_register_status(usart, TxReg::A, &[0x89]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_lda_indy_within_mocking_memory passed!");
}

pub fn test_lda_zpx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x05]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x05]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB5, 0x60]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x65, 0x00]);
    usart.blocking_write(&[0x0C]).unwrap();
    check_valid_register_status(usart, TxReg::A, &[0x0C]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_lda_zpx_within_mocking_memory passed!");
}

pub fn test_lda_absy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x44]);
    check_valid_register_status(usart, TxReg::Y, &[0x44]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB9]);
    check_rw_is_high(&rw);
    usart_write(usart, &[0x30, 0x41]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x74, 0x41]);
    usart_write(usart, &[0xBB]);
    check_valid_register_status(usart, TxReg::A, &[0xBB]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_lda_absy_within_mocking_memory passed!");
}

pub fn test_lda_absx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x75]);
    check_valid_register_status(usart, TxReg::X, &[0x75]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xBD]);
    check_rw_is_high(&rw);
    usart_write(usart, &[0xA7, 0x09]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x1C, 0x0A]);
    usart_write(usart, &[0xD4]);
    check_valid_register_status(usart, TxReg::A, &[0xD4]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_lda_absx_within_mocking_memory passed!");
}

pub fn test_ldx_imm_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa2]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x45]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x45]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_ldx_imm_within_mocking_memory passed!");
}

pub fn test_ldx_zp_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa6]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0xF0]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xF0, 0x00]);
    usart.blocking_write(&[0x90]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x90]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ldx_zp_within_mocking_memory passed!");
}

pub fn test_ldx_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAE]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0xFF, 0x07]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xFF, 0x07]);
    usart.blocking_write(&[0xFB]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0xFB]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ldx_abs_within_mocking_memory passed!");
}

pub fn test_ldx_zpy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0xC0]);
    check_valid_register_status(usart, TxReg::Y, &[0xC0]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB6]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x91]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x51, 0x00]);
    usart.blocking_write(&[0x3B]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x3B]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_ldx_zpy_within_mocking_memory passed!");
}

pub fn test_ldx_absy_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x78]);
    check_valid_register_status(usart, TxReg::Y, &[0x78]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xBE]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x00, 0x06]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x33]).unwrap();
    check_valid_register_status(usart, TxReg::X, &[0x33]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_ldx_absy_within_mocking_memory passed!");
}

pub fn test_ldy_imm_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xa0]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0xba]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0xba]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ldy_imm_within_mocking_memory passed!");
}

pub fn test_ldy_zp_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA4]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0xC3]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xC3, 0x00]);
    usart.blocking_write(&[0xDD]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0xDD]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ldy_zp_within_mocking_memory passed!");
}

pub fn test_ldy_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAC]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x78, 0x06]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x55]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0x55]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_ldy_abs_within_mocking_memory passed!");
}

pub fn test_ldy_zpx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x00]);
    check_valid_register_status(usart, TxReg::X, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xB4]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0x33]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x33, 0x00]);
    usart.blocking_write(&[0xAA]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0xAA]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_ldy_zpx_within_mocking_memory passed!");
}

pub fn test_ldy_absx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin, P3: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    rw: &Input<P2>,
    resb: &mut Output<P3>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x8A]);
    check_valid_register_status(usart, TxReg::X, &[0x8A]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xBC]);
    check_rw_is_high(&rw);
    usart.blocking_write(&[0xFF, 0x05]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x89, 0x06]);
    usart.blocking_write(&[0x00]).unwrap();
    check_valid_register_status(usart, TxReg::Y, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00100110]);
    info!("test_ldy_absx_within_mocking_memory passed!");
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
    test_lda_nestest_head(&mut usart, &nop, &mut resb);

    test_lda_indx_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_zp_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_imm_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_abs_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_indy_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_zpx_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_absy_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_absx_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldx_imm_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldx_zp_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldx_abs_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldx_zpy_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldx_absy_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldy_imm_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldy_zp_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldy_abs_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldy_zpx_within_internal_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldy_absx_within_internal_memory(&mut usart, &nop, &rw, &mut resb);

    test_lda_indx_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_zp_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_imm_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_abs_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_indy_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_zpx_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_absy_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_lda_absx_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldx_imm_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldx_zp_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldx_abs_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldx_zpy_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldx_absy_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldy_imm_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldy_zp_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldy_abs_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldy_zpx_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    test_ldy_absx_within_mocking_memory(&mut usart, &nop, &rw, &mut resb);
    info!("all tests passed!");
    loop {}
}
