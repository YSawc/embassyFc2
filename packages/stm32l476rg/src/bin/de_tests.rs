#![no_std]
#![no_main]

use defmt::*;
use embassy_fc2_app::middleware::mode::{CpuMode, OpeMode, TxReg};
use embassy_stm32::dma::NoDma;
use embassy_stm32::gpio::{Input, Level, Output, Pin, Pull, Speed};
use embassy_stm32::usart::{BasicInstance, Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use stm32l476rg::pin::util::*;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

pub fn test_dey_impl_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x33]);
    check_valid_register_status(usart, TxReg::Y, &[0x33]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x88]);
    check_valid_register_status(usart, TxReg::Y, &[0x32]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_dey_impl_within_mocking_memory passed!");
}

pub fn test_dey_impl_with_rising_z_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::Y, &[0x01]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x88]);
    check_valid_register_status(usart, TxReg::Y, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    info!("test_dey_impl_with_rising_z_flag_within_mocking_memory passed!");
}

pub fn test_dey_impl_with_rising_n_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x88]);
    check_valid_register_status(usart, TxReg::Y, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_dey_impl_with_rising_n_flag_within_mocking_memory passed!");
}

pub fn test_dey_impl_with_falling_n_flag_within_mocking_memory<
    T: BasicInstance,
    P: Pin,
    P2: Pin,
>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x80]);
    check_valid_register_status(usart, TxReg::Y, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x88]);
    check_valid_register_status(usart, TxReg::Y, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_dey_impl_with_falling_n_flag_within_mocking_memory passed!");
}

pub fn test_dex_impl_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x33]);
    check_valid_register_status(usart, TxReg::X, &[0x33]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCA]);
    check_valid_register_status(usart, TxReg::X, &[0x32]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_dex_impl_within_mocking_memory passed!");
}

pub fn test_dex_impl_with_rising_z_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x01]);
    check_valid_register_status(usart, TxReg::X, &[0x01]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCA]);
    check_valid_register_status(usart, TxReg::X, &[0x00]);
    check_valid_register_status(usart, TxReg::P, &[0b00000010]);
    info!("test_dex_impl_with_rising_z_flag_within_mocking_memory passed!");
}

pub fn test_dex_impl_with_rising_n_flag_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCA]);
    check_valid_register_status(usart, TxReg::X, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_dex_impl_with_rising_n_flag_within_mocking_memory passed!");
}

pub fn test_dex_impl_with_falling_n_flag_within_mocking_memory<
    T: BasicInstance,
    P: Pin,
    P2: Pin,
>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x80]);
    check_valid_register_status(usart, TxReg::X, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCA]);
    check_valid_register_status(usart, TxReg::X, &[0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_dex_impl_with_falling_n_flag_within_mocking_memory passed!");
}

pub fn test_dec_zp_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xC6, 0x78]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x00]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x78, 0x00, 0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_dec_zp_within_mocking_memory passed!");
}

pub fn test_dec_abs_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCE, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x80]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x78, 0x06, 0x7F]);
    check_valid_register_status(usart, TxReg::P, &[0b00000000]);
    info!("test_dec_abs_within_mocking_memory passed!");
}

pub fn test_dec_zpx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x55]);
    check_valid_register_status(usart, TxReg::X, &[0x55]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xDE, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x55, 0x06]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x55, 0x06, 0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_dec_zpx_within_mocking_memory passed!");
}

pub fn test_dec_absx_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA2, 0x55]);
    check_valid_register_status(usart, TxReg::X, &[0x55]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xDE, 0x00, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x55, 0x06]);
    usart.blocking_write(&[0x00]).unwrap();
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0x55, 0x06, 0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    info!("test_dec_absx_within_mocking_memory passed!");
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
    test_dey_impl_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_dey_impl_with_rising_z_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_dey_impl_with_rising_n_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_dey_impl_with_falling_n_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_dex_impl_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_dex_impl_with_rising_z_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_dex_impl_with_rising_n_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_dex_impl_with_falling_n_flag_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_dec_zp_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_dec_abs_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_dec_zpx_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_dec_absx_within_mocking_memory(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
