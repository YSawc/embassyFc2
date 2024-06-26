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

pub fn test_php_impl_within_n_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
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
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x08]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::S, &[0xFC]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFD, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0b10100100]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    info!("test_php_impl_within_n_within_internal_memory passed!");
}

pub fn test_php_impl_within_cz_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
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
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCD, 0x78, 0x06]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x08]);
    check_valid_register_status(usart, TxReg::S, &[0xFC]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFD, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0b00100111]);
    check_valid_register_status(usart, TxReg::P, &[0b00100101]);
    info!("test_php_impl_within_cz_within_internal_memory passed!");
}

pub fn test_php_impl_within_none_flag_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x08]);
    check_valid_register_status(usart, TxReg::S, &[0xFC]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFD, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0b00100100]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_php_impl_within_none_flag_within_internal_memory passed!");
}

pub fn test_pha_impl_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x82]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x82]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCD, 0x78, 0x06]);
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    check_valid_register_status(usart, TxReg::S, &[0xFC]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    check_valid_register_status(usart, TxReg::S, &[0xFB]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFD, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    check_valid_register_status(usart, TxReg::P, &[0b10100101]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFC, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    check_valid_register_status(usart, TxReg::P, &[0b10100101]);
    info!("test_pha_impl_within_internal_memory passed!");
}

pub fn test_php_impl_within_n_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
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
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x08]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFD, 0x01, 0b10100100]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::S, &[0xFC]);
    info!("test_php_impl_within_n_within_mocking_memory passed!");
}

pub fn test_php_impl_within_cz_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCD, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x80]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x08]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFD, 0x01, 0b00100111]);
    check_valid_register_status(usart, TxReg::S, &[0xFC]);
    info!("test_php_impl_within_cz_within_mocking_memory passed!");
}

pub fn test_php_impl_within_none_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x08]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFD, 0x01, 0b00100100]);
    check_valid_register_status(usart, TxReg::S, &[0xFC]);
    info!("test_php_impl_within_none_flag_within_mocking_memory passed!");
}

pub fn test_pha_impl_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x82]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCD, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x82]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b00100111]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFD, 0x01, 0x82]);
    check_valid_register_status(usart, TxReg::S, &[0xFC]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFC, 0x01, 0x82]);
    check_valid_register_status(usart, TxReg::S, &[0xFB]);
    info!("test_pha_impl_within_mocking_memory passed!");
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
    test_php_impl_within_n_within_internal_memory(&mut usart, &nop, &mut resb);
    test_php_impl_within_cz_within_internal_memory(&mut usart, &nop, &mut resb);
    test_php_impl_within_none_flag_within_internal_memory(&mut usart, &nop, &mut resb);
    test_pha_impl_within_internal_memory(&mut usart, &nop, &mut resb);

    test_php_impl_within_n_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_php_impl_within_cz_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_php_impl_within_none_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_pha_impl_within_mocking_memory(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
