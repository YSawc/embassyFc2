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

pub fn test_php_impl_within_n<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::P, &[0b10000000]);
    check_valid_register_status(usart, TxReg::S, &[0xFF]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x08]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFE, 0x01, 0b10000000]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    check_valid_register_status(usart, TxReg::S, &[0xFE]);
    info!("test_php_impl_within_n passed!");
}

pub fn test_php_impl_within_cz<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x80]);
    check_valid_register_status(usart, TxReg::A, &[0x80]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCD, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x80]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    check_valid_register_status(usart, TxReg::S, &[0xFF]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x08]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFE, 0x01, 0b00000011]);
    check_valid_register_status(usart, TxReg::S, &[0xFE]);
    info!("test_php_impl_within_cz passed!");
}

pub fn test_php_impl_within_none_flag<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x08]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFE, 0x01, 0b00000000]);
    check_valid_register_status(usart, TxReg::S, &[0xFE]);
    info!("test_php_impl_within_none_flag passed!");
}

pub fn test_pha_impl<T: BasicInstance, P: Pin, P2: Pin>(
    usart: &mut Uart<T>,
    nop: &Input<P>,
    resb: &mut Output<P2>,
) {
    send_reset_signal_if_not_nop(&nop, resb);
    usart_write(usart, &[CpuMode::DebugWithinMockMemory as u8]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xA9, 0x82]);
    check_valid_register_status(usart, TxReg::A, &[0x82]);
    usart_write(usart, &[OpeMode::Inst as u8, 0xCD, 0x78, 0x06]);
    usart_read_with_check(usart, &mut [0x0u8; 2], &[0x78, 0x06]);
    usart.blocking_write(&[0x82]).unwrap();
    check_valid_register_status(usart, TxReg::P, &[0b00000011]);
    check_valid_register_status(usart, TxReg::S, &[0xFF]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFE, 0x01, 0x82]);
    check_valid_register_status(usart, TxReg::S, &[0xFE]);
    usart_write(usart, &[OpeMode::Inst as u8, 0x48]);
    usart_read_with_check(usart, &mut [0x0u8; 3], &[0xFD, 0x01, 0x82]);
    check_valid_register_status(usart, TxReg::S, &[0xFD]);
    info!("test_pha_impl passed!");
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
    test_php_impl_within_n(&mut usart, &nop, &mut resb);
    test_php_impl_within_cz(&mut usart, &nop, &mut resb);
    test_php_impl_within_none_flag(&mut usart, &nop, &mut resb);
    test_pha_impl(&mut usart, &nop, &mut resb);
    info!("all tests passed!");
    loop {}
}
