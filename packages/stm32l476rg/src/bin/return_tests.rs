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

pub fn test_rti_impl_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xCE]);
    check_valid_register_status(usart, TxReg::A, &[0xCE]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    check_valid_register_status(usart, TxReg::A, &[0xCE]);
    check_valid_register_status(usart, TxReg::S, &[0xFC]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFD, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0xCE]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAD]);
    check_valid_register_status(usart, TxReg::A, &[0xAD]);
    check_valid_register_status(usart, TxReg::S, &[0xFC]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    check_valid_register_status(usart, TxReg::A, &[0xAD]);
    check_valid_register_status(usart, TxReg::S, &[0xFB]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFC, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0xAD]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x65]);
    check_valid_register_status(usart, TxReg::A, &[0x65]);
    check_valid_register_status(usart, TxReg::S, &[0xFB]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    check_valid_register_status(usart, TxReg::A, &[0x65]);
    check_valid_register_status(usart, TxReg::S, &[0xFA]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFB, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0x65]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x65]);
    check_valid_register_status(usart, TxReg::A, &[0x65]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0xFC, 0x01]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAD]);
    check_valid_register_status(usart, TxReg::A, &[0xAD]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0xFC, 0x01]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xCE]);
    check_valid_register_status(usart, TxReg::A, &[0xCE]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0xFD, 0x01]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x40]);
    check_valid_register_status(usart, TxReg::PC, &[0xAD, 0xCE]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    check_valid_register_status(usart, TxReg::P, &[0b01100101]);
    info!("test_rti_abs_within_internal_memory passed!");
}

pub fn test_rts_impl_within_internal_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinInternalMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xFC, 0xC5]);
    check_valid_register_status(usart, TxReg::PC, &[0xFC, 0xc5]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x20, 0x2D, 0xC7]);
    check_valid_register_status(usart, TxReg::PC, &[0x2D, 0xC7]);
    check_valid_register_status(usart, TxReg::S, &[0xFB]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFD, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0xC5]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xAD, 0xFC, 0x01]);
    check_valid_register_status(usart, TxReg::A, &[0xFE]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xFF]);
    check_valid_register_status(usart, TxReg::A, &[0xFF]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0xFC, 0x01]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xCE]);
    check_valid_register_status(usart, TxReg::A, &[0xCE]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x8D, 0x78, 0x06]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA0, 0x01]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x60]);
    check_valid_register_status(usart, TxReg::PC, &[0x00, 0xC6]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_rts_abs_within_internal_memory passed!");
}

pub fn test_rti_impl_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xCE]);
    check_valid_register_status(usart, TxReg::A, &[0xCE]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFD, 0x01, 0xCE]);
    check_valid_register_status(usart, TxReg::A, &[0xCE]);
    check_valid_register_status(usart, TxReg::S, &[0xFC]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0xAD]);
    check_valid_register_status(usart, TxReg::A, &[0xAD]);
    check_valid_register_status(usart, TxReg::S, &[0xFC]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFC, 0x01, 0xAD]);
    check_valid_register_status(usart, TxReg::A, &[0xAD]);
    check_valid_register_status(usart, TxReg::S, &[0xFB]);
    check_valid_register_status(usart, TxReg::P, &[0b10100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x24]);
    check_valid_register_status(usart, TxReg::A, &[0x24]);
    check_valid_register_status(usart, TxReg::S, &[0xFB]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFB, 0x01, 0x24]);
    check_valid_register_status(usart, TxReg::A, &[0x24]);
    check_valid_register_status(usart, TxReg::S, &[0xFA]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x40]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xFB, 0x01]);
    usart_write(usart, &[0x24]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xFC, 0x01]);
    usart_write(usart, &[0xAD]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xFD, 0x01]);
    usart_write(usart, &[0xCE]);
    check_valid_register_status(usart, TxReg::PC, &[0xAD, 0xCE]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_rti_abs_within_mocking_memory passed!");
}

pub fn test_rts_impl_within_mocking_memory<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[CassetteMode::None as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x4C, 0xFC, 0xC5]);
    check_valid_register_status(usart, TxReg::PC, &[0xFC, 0xc5]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x20, 0x2D, 0xC7]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFD, 0x01, 0xC5]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFC, 0x01, 0xFE]);
    check_valid_register_status(usart, TxReg::PC, &[0x2D, 0xC7]);
    check_valid_register_status(usart, TxReg::S, &[0xFB]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x60]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xFC, 0x01]);
    usart_write(usart, &[0xFF]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0xFD, 0x01]);
    usart_write(usart, &[0xC5]);
    check_valid_register_status(usart, TxReg::PC, &[0x00, 0xC6]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    check_valid_register_status(usart, TxReg::P, &[0b00100100]);
    info!("test_rts_abs_within_mocking_memory passed!");
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
    test_rti_impl_within_internal_memory(&mut usart, &nop, &mut resb);
    test_rts_impl_within_internal_memory(&mut usart, &nop, &mut resb);

    test_rti_impl_within_mocking_memory(&mut usart, &nop, &mut resb);
    test_rts_impl_within_mocking_memory(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
